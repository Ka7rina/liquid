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
    codegen::GenerateCode,
    ir::{Contract, FnArg, Function, FunctionKind},
};
use core::cell::RefCell;
use liquid_prelude::{collections::BTreeSet, string::String};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub struct Dispatch<'a> {
    contract: &'a Contract,
    inputs_set: RefCell<BTreeSet<String>>,
}

impl<'a> From<&'a Contract> for Dispatch<'a> {
    fn from(contract: &'a Contract) -> Self {
        Self {
            contract,
            inputs_set: RefCell::new(Default::default()),
        }
    }
}

impl<'a> GenerateCode for Dispatch<'a> {
    fn generate_code(&self) -> TokenStream2 {
        let marker = self.generate_external_fn_marker();
        let traits = self.generate_external_fn_traits();
        let dispatch = self.generate_dispatch();
        let entry_point = self.generate_entry_point();

        quote! {
            #[cfg(not(test))]
            const _: () = {
                #marker
                #traits
                #dispatch
                #entry_point
            };
        }
    }
}

impl<'a> Dispatch<'a> {
    fn generate_external_fn_marker(&self) -> TokenStream2 {
        quote! {
            pub struct ExternalMarker<S> {
                marker: core::marker::PhantomData<fn() -> S>,
            }
        }
    }

    fn generate_external_fn_traits(&self) -> TokenStream2 {
        let traits = self
            .contract
            .functions
            .iter()
            .map(|func| self.generate_external_fn_trait(func));

        quote! {
            /// To evade the orphan rule in Rust.
            struct NotOrphan<T> {
                marker: core::marker::PhantomData<fn() -> T>,
            }

            impl liquid_lang::ty_mapping::SolTypeName for NotOrphan<()> {
                const NAME: &'static [u8] = <() as liquid_lang::ty_mapping::SolTypeName>::NAME;
            }

            impl liquid_lang::ty_mapping::SolTypeNameLen for NotOrphan<()> {
                const LEN: usize = <() as liquid_lang::ty_mapping::SolTypeNameLen>::LEN;
            }

            #(#traits)*
        }
    }

    fn generate_external_fn_trait(&self, func: &Function) -> TokenStream2 {
        let fn_id = match &func.kind {
            FunctionKind::External(fn_id) => fn_id,
            _ => return quote! {},
        };

        let span = func.span();
        let external_marker = quote! { ExternalMarker<[(); #fn_id]> };
        let sig = &func.sig;

        let inputs = &sig.inputs;
        let input_tys = &inputs
            .iter()
            .skip(1)
            .map(|arg| match arg {
                FnArg::Typed(ident_type) => &ident_type.ty,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let output = &sig.output;
        let output_ty = {
            match output {
                syn::ReturnType::Default => quote! {()},
                syn::ReturnType::Type(_, ty) => quote! { #ty },
            }
        };

        let fn_input = quote_spanned! { inputs.span() =>
            impl liquid_lang::FnInput for #external_marker  {
                type Input = (#(<#input_tys as liquid_lang::ValidLiquidInputType>::T,)*);
            }
        };

        let fn_output = quote_spanned! { output.span() =>
            impl liquid_lang::FnOutput for #external_marker {
                type Output = <#output_ty as liquid_lang::ValidLiquidOutputType>::T;
            }
        };

        let mut selectors = quote_spanned! { span =>
        };
        for i in 1..=input_tys.len() {
            let tys = &input_tys[..i];

            let tys_str = quote! { #(#tys)* }.to_string();
            if self.inputs_set.borrow().contains(&tys_str) {
                continue;
            } else {
                self.inputs_set.borrow_mut().insert(tys_str);
            }
            let first_tys = &tys[0..i - 1];
            let rest_ty = &tys[i - 1];
            if i > 1 {
                selectors.extend(quote_spanned! { span =>
                    impl liquid_lang::ty_mapping::SolTypeName for NotOrphan<(#(#tys,)*)> {
                        const NAME: &'static [u8] = {
                            const LEN: usize =
                                <(#(#first_tys,)*) as liquid_lang::ty_mapping::SolTypeNameLen>::LEN
                                + <#rest_ty as liquid_lang::ty_mapping::SolTypeNameLen>::LEN
                                + 1;
                            &liquid_lang::ty_mapping::concat::<NotOrphan<(#(#first_tys,)*)>, #rest_ty, LEN>()
                        };
                    }
                });
            } else {
                selectors.extend(quote_spanned! { span =>
                    impl liquid_lang::ty_mapping::SolTypeName for NotOrphan<(#rest_ty,)> {
                        const NAME: &'static [u8] = <#rest_ty as liquid_lang::ty_mapping::SolTypeName>::NAME;
                    }
                });
            }
        }

        let fn_name = sig.ident.to_string();
        let fn_name_bytes = fn_name.as_bytes();
        let fn_name_len = fn_name_bytes.len();
        let composite_sig = quote! {
            const SIG_LEN: usize =
                <(#(#input_tys,)*) as liquid_lang::ty_mapping::SolTypeNameLen>::LEN + #fn_name_len
                + 2;
            const SIG: [u8; SIG_LEN] =
                liquid_lang::ty_mapping::composite::<SIG_LEN>(
                    &[#(#fn_name_bytes),*],
                    <NotOrphan<(#(#input_tys,)*)> as liquid_lang::ty_mapping::SolTypeName>::NAME);
        };
        selectors.extend(quote_spanned! { span =>
            impl liquid_lang::FnSelectors for #external_marker {
                const KECCAK256_SELECTOR: liquid_primitives::Selector = {
                    #composite_sig
                    liquid_primitives::hash::keccak::keccak256(&SIG)
                };
                const SM3_SELECTOR: liquid_primitives::Selector = {
                    #composite_sig
                    liquid_primitives::hash::sm3::sm3(&SIG)
                };
            }
        });

        let is_mut = sig.is_mut();
        let mutability = quote_spanned! { span =>
            impl liquid_lang::FnMutability for #external_marker {
                const IS_MUT: bool = #is_mut;
            }
        };

        quote_spanned! { span =>
            #fn_input
            #fn_output
            #selectors
            #mutability
            impl liquid_lang::ExternalFn for #external_marker {}
        }
    }

    fn generate_dispatch_fragment(&self, func: &Function) -> TokenStream2 {
        let fn_id = match &func.kind {
            FunctionKind::External(fn_id) => fn_id,
            _ => return quote! {},
        };
        let namespace = quote! { ExternalMarker<[(); #fn_id]> };

        let sig = &func.sig;
        let fn_name = &sig.ident;
        let inputs = &sig.inputs;
        let input_idents = inputs
            .iter()
            .skip(1)
            .map(|arg| match arg {
                FnArg::Typed(ident_type) => &ident_type.ident,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let pat_idents = if input_idents.is_empty() {
            quote! { _ }
        } else {
            quote! { (#(#input_idents,)*) }
        };

        let builder_name = if sig.is_mut() {
            quote! { on_external_mut }
        } else {
            quote! { on_external }
        };

        quote! {
            .#builder_name::<#namespace>(|storage, #pat_idents| {
                storage.#fn_name(#(#input_idents,)*)
            })
        }
    }

    fn generate_dispatch(&self) -> TokenStream2 {
        let fragments = self
            .contract
            .functions
            .iter()
            .map(|func| self.generate_dispatch_fragment(func));
        let constr = &self.contract.constructor;
        let constr_sig = &constr.sig;
        let constr_ident = &constr_sig.ident;
        let constr_inputs = &constr_sig.inputs;

        let constr_input_idents = constr_sig
            .inputs
            .iter()
            .skip(1)
            .map(|arg| match arg {
                FnArg::Typed(ident_type) => &ident_type.ident,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let constr_pat_idents = if constr_input_idents.is_empty() {
            quote! { _ }
        } else {
            quote! { (#(#constr_input_idents,)*) }
        };

        let constr_input_tys = constr_inputs.iter().skip(1).map(|arg| match arg {
            FnArg::Typed(ident_type) => &ident_type.ty,
            _ => unreachable!(),
        });

        quote! {
            impl Storage {
                pub fn dispatch_using_mode(mode: liquid_lang::CallMode) -> liquid_lang::DispatchResult {
                    liquid_lang::Contract::new_builder::<Storage, (#(#constr_input_tys,)*)>(|storage, #constr_pat_idents| {
                        storage.#constr_ident(#(#constr_input_idents,)*);
                    })
                    #(
                        #fragments
                    )*
                    .done()
                    .dispatch(mode)
                }
            }
        }
    }

    fn generate_entry_point(&self) -> TokenStream2 {
        quote! {
            #[no_mangle]
            fn deploy() -> u32{
                liquid_lang::DispatchRetCode::from(Storage::dispatch_using_mode(liquid_lang::CallMode::Deploy)).to_u32()
            }

            #[no_mangle]
            fn call() -> u32 {
                liquid_lang::DispatchRetCode::from(Storage::dispatch_using_mode(liquid_lang::CallMode::Call)).to_u32()
            }
        }
    }
}
