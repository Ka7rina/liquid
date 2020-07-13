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

mod buffer;
mod ext;

use self::buffer::StaticBuffer;
use super::OnInstance;
use crate::{CallData, Env, Result};
use liquid_primitives::Key;

/// The on-chain environment
pub struct EnvInstance {
    buffer: StaticBuffer,
}

impl OnInstance for EnvInstance {
    fn on_instance<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        static mut INSTANCE: EnvInstance = EnvInstance {
            buffer: StaticBuffer::new(),
        };

        f(unsafe { &mut INSTANCE })
    }
}

impl EnvInstance {
    fn reset_buffer(&mut self) {
        self.buffer.clear();
    }

    fn encode_into_buffer_scale<V>(&mut self, value: &V)
    where
        V: scale::Encode,
    {
        self.reset_buffer();
        scale::Encode::encode_to(value, &mut self.buffer);
    }

    fn encode_into_buffer_abi<V>(&mut self, value: &V)
    where
        V: liquid_abi_coder::Encode,
    {
        self.reset_buffer();
        liquid_abi_coder::Encode::encode_to(value, &mut self.buffer);
    }

    fn decode_from_buffer_scale<R>(&mut self) -> Result<R>
    where
        R: scale::Decode,
    {
        let len = self.buffer.len();
        scale::Decode::decode(&mut &self.buffer[..len]).map_err(Into::into)
    }

    fn decode_from_buffer_abi<R>(&mut self) -> Result<R>
    where
        R: liquid_abi_coder::Decode,
    {
        let len = self.buffer.len();
        liquid_abi_coder::Decode::decode(&mut &self.buffer[..len]).map_err(Into::into)
    }
}

impl Env for EnvInstance {
    fn set_storage<V>(&mut self, key: Key, value: &V)
    where
        V: scale::Encode,
    {
        self.encode_into_buffer_scale(value);
        ext::set_storage(key.as_bytes(), &self.buffer[..]);
    }

    fn get_storage<R>(&mut self, key: Key) -> Result<R>
    where
        R: scale::Decode,
    {
        let size = ext::get_storage(key.as_bytes(), &mut self.buffer[..])?;
        self.buffer.resize(size as usize);
        self.decode_from_buffer_scale()
    }

    fn get_call_data(&mut self) -> Result<CallData> {
        let size = ext::get_call_data(&mut self.buffer[..])?;
        self.buffer.resize(size as usize);
        self.decode_from_buffer_abi()
    }

    fn finish<V>(&mut self, return_value: &V)
    where
        V: liquid_abi_coder::Encode,
    {
        self.reset_buffer();
        self.encode_into_buffer_abi(return_value);
        ext::finish(&self.buffer[..self.buffer.len()]);
    }
}
