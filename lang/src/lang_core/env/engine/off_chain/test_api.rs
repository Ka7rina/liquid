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

use super::{EnvInstance, Event, ExecContext};
use crate::lang_core::env::engine::OnInstance;
use liquid_primitives::types::address::*;

/// Pushes a contract execution context.
///
/// This is the data behind a single instance of a contract call.
///
/// # Note
///
/// Together with [`pop_execution_context`] this can be used to emulated
/// nested calls.
pub fn set_caller(caller: Address) {
    <EnvInstance as OnInstance>::on_instance(|instance| {
        instance.exec_contexts.push(ExecContext::new(caller, Default::default()))
    });
}

/// Pushes a contract execution context.
///
/// This is the data behind a single instance of a contract call.
///
/// # Note
///
/// Together with [`pop_execution_context`] this can be used to emulated
/// nested calls.
pub fn set_caller_callee(caller: Address, callee: Address) {
    <EnvInstance as OnInstance>::on_instance(|instance| {
        instance.exec_contexts.push(ExecContext::new(caller, callee))
    });
}

/// Pops the top contract execution context.
///
/// # Note
///
/// Together with [`set_caller`] this can be used to emulated
/// nested calls.
pub fn pop_execution_context() {
    <EnvInstance as OnInstance>::on_instance(|instance| {
        instance.exec_contexts.pop();
    })
}

/// The default accounts.
pub struct DefaultAccounts {
    pub alice: Address,
    pub bob: Address,
    pub charlie: Address,
    pub david: Address,
    pub eve: Address,
    pub frank: Address,
}

/// Returns the default accounts for testing purposes:
/// Alice, Bob, Charlie, David, Eve and Frank
/// https://en.wikipedia.org/wiki/Alice_and_Bob
pub fn default_accounts() -> DefaultAccounts {
    DefaultAccounts {
        alice: [0xffu8; ADDRESS_LENGTH].into(),
        bob: [0x01u8; ADDRESS_LENGTH].into(),
        charlie: [0x02u8; ADDRESS_LENGTH].into(),
        david: [0x03u8; ADDRESS_LENGTH].into(),
        eve: [0x04u8; ADDRESS_LENGTH].into(),
        frank: [0x05u8; ADDRESS_LENGTH].into(),
    }
}

/// Returns the recorded emitted events in order.
pub fn get_events() -> Vec<Event> {
    <EnvInstance as OnInstance>::on_instance(|instance| {
        instance.get_events().cloned().collect::<Vec<_>>()
    })
}
