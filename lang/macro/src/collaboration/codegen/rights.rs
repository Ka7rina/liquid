// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    collaboration::{
        codegen::{path_visitor::PathVisitor, utils},
        ir::*,
    },
    traits::GenerateCode,
    utils::filter_non_liquid_attributes,
};
use derive_more::From;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[derive(From)]
pub struct Rights<'a> {
    collaboration: &'a Collaboration,
}

impl<'a> GenerateCode for Rights<'a> {
    fn generate_code(&self) -> TokenStream2 {
        let all_item_rights = &self.collaboration.all_item_rights;
        let impls = all_item_rights.iter().map(|item_rights| {
            let contract_ident = &item_rights.ty;
            let rights = &item_rights.rights;
            let fns = rights.iter().map(|right| {
                let owners = &right.owners;
                let selectors = owners.iter().map(|owner| {
                    let from = &owner.from;
                    let with = &owner.with;
                    let span = from.span();
                    let ident = match from {
                        SelectFrom::This(ident) => quote! { self.#ident },
                        SelectFrom::Argument(ident) => quote! { #ident },
                    };

                    match with {
                        None => {
                            quote_spanned! { span => &#ident }
                        }
                        Some(SelectWith::Func(path)) => {
                            quote_spanned! { path.span() =>
                                #path(#ident)
                            }
                        }
                        Some(SelectWith::Obj(ast)) => {
                            let mut path_visitor =
                                PathVisitor::new(Some(ident), &ast.arena);
                            let stmts = path_visitor.eval(ast.root);
                            quote_spanned! { span =>
                                #stmts
                            }
                        }
                    }
                });
                let attrs = filter_non_liquid_attributes(&right.attrs);
                let sig = &right.sig;
                let fn_ident = &sig.ident;
                let inputs = &sig.inputs;
                let output = &sig.output;
                let body = &right.body;
                let stmts = &body.stmts;
                let clone_error = format!(
                    "the exercising of right `{}` must be based on an existing `{}` \
                     contract, not a cloned one",
                    fn_ident, contract_ident
                );
                let abolish_owners = if sig.is_self_ref() {
                    quote! {}
                } else {
                    quote! { owners.extend(signers.clone()); }
                };

                quote_spanned! { right.span =>
                    #(#attrs)*
                    pub fn #fn_ident (#inputs) #output {
                        if self.__liquid_forbids_constructing_contract.0 {
                            liquid_lang::env::revert(&#clone_error.to_owned())
                        }

                        {
                            #[allow(unused_imports)]
                            use liquid_lang::Can_Not_Select_Any_Account_Address_From_It;
                            use liquid_lang::AcquireSigners;

                            let signers = self.acquire_signers();
                            #[allow(unused_mut)]
                            let mut owners = liquid_prelude::collections::BTreeSet::<address>::new();
                            #abolish_owners
                            #(owners.extend((#selectors).acquire_addrs());)*
                            let storage = __liquid_acquire_storage_instance();
                            let authorizers = &mut storage.__liquid_authorizers;
                            for owner in &owners {
                                if !authorizers.contains(owner) {
                                    liquid_lang::env::revert(&#contract_ident::__LIQUID_UNAUTHORIZED_CREATE_ERROR.to_owned());
                                }
                            }
                            authorizers.extend(signers);
                        }

                        #(#stmts)*
                    }
                }
            });

            quote! {
                impl #contract_ident {
                    #(#fns)*
                }
            }
        });

        let contracts = &self.collaboration.contracts;
        let envs = contracts.iter().map(|contract| {
            let ident = &contract.ident;

            quote! {
                impl #ident {
                    #[allow(unused)]
                    pub fn env(&self) -> liquid_lang::EnvAccess {
                        liquid_lang::EnvAccess {}
                    }
                }
            }
        });

        quote! {
            #(#impls)*
            #(#envs)*
        }
    }
}
