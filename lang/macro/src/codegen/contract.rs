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
    codegen::{dispatch::Dispatch, env_types::EnvTypes, storage::Storage, GenerateCode},
    ir,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl GenerateCode for ir::Contract {
    fn generate_code(&self) -> TokenStream2 {
        let ident = &self.ident;
        let storage_ident = &self.storage.ident;
        let env_types = EnvTypes::from(self).generate_code();
        let storage = Storage::from(self).generate_code();
        let dispatch = Dispatch::from(self).generate_code();
        let rust_items = &self.rust_items;

        quote! {
            mod #ident {
                #env_types

                mod __liquid_private {
                    use super::*;

                    #storage
                    #dispatch
                }

                #[cfg(test)]
                pub type #storage_ident = self::__liquid_private::Storage;

                #(
                    #rust_items
                )*
            }
        }
    }
}
