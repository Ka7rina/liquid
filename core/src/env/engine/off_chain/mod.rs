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

mod db;
pub mod test_api;

use self::db::{Block, ContractStorage, ExecContext};
use crate::env::{
    engine::OnInstance,
    types::{Address, BlockNumber, Timestamp},
    CallData, CallMode, Env, Result,
};
use core::cell::RefCell;

pub struct EnvInstance {
    contract_storage: ContractStorage,
    blocks: Vec<Block>,
    exec_contexts: Vec<ExecContext>,
}

impl EnvInstance {
    pub fn new() -> Self {
        let mut blocks = Vec::new();
        blocks.push(Block::new(0));

        Self {
            contract_storage: ContractStorage::new(),
            blocks,
            exec_contexts: Vec::new(),
        }
    }

    pub fn current_exec_context(&self) -> &ExecContext {
        self.exec_contexts
            .last()
            .expect("there must be at least one execution context in test environment")
    }

    pub fn current_block(&self) -> &Block {
        self.blocks
            .last()
            .expect("there must be at least one block in test environment")
    }
}

impl Env for EnvInstance {
    fn set_storage<V>(&mut self, key: &[u8], value: &V)
    where
        V: scale::Encode,
    {
        self.contract_storage.set_storage(key, value);
    }

    fn get_storage<R>(&mut self, key: &[u8]) -> Result<R>
    where
        R: scale::Decode,
    {
        self.contract_storage.get_storage::<R>(key)
    }

    fn remove_storage(&mut self, key: &[u8]) {
        self.contract_storage.remove_storage(key);
    }

    fn get_call_data(&mut self, _: CallMode) -> Result<CallData> {
        unimplemented!();
    }

    fn finish<V>(&mut self, _: &V)
    where
        V: liquid_abi_codec::Encode,
    {
        unimplemented!();
    }

    fn revert<V>(&mut self, msg: &V)
    where
        V: liquid_abi_codec::Encode,
    {
        // Ensure that the type of `V` can only be String.
        panic!(<String as liquid_abi_codec::Decode>::decode(
            &mut msg.encode().as_slice()
        )
        .unwrap());
    }

    fn get_caller(&mut self) -> Address {
        self.current_exec_context().caller()
    }

    fn now(&mut self) -> Timestamp {
        self.current_block().timestamp()
    }

    fn get_block_number(&mut self) -> BlockNumber {
        self.current_block().block_number()
    }
}

impl OnInstance for EnvInstance {
    fn on_instance<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        thread_local!(
            static INSTANCE: RefCell<EnvInstance> = RefCell::new(
                EnvInstance::new()
            )
        );

        INSTANCE.with(|instance| f(&mut instance.borrow_mut()))
    }
}
