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

use liquid_prelude::string::String;

pub enum DispatchError {
    UnknownSelector,
    InvalidParams,
    CouldNotReadInput,
}

pub struct DispatchRetInfo(bool, &'static str);

impl From<DispatchError> for DispatchRetInfo {
    fn from(err: DispatchError) -> Self {
        match err {
            DispatchError::UnknownSelector => Self(false, "unknown selector"),
            DispatchError::InvalidParams => Self(false, "invalid params"),
            DispatchError::CouldNotReadInput => Self(false, "could not read input"),
        }
    }
}

impl DispatchRetInfo {
    pub fn get_info_string(&self) -> String {
        String::from(self.1)
    }

    pub fn is_success(&self) -> bool {
        self.0
    }
}

pub type DispatchResult = core::result::Result<(), DispatchError>;

impl From<DispatchResult> for DispatchRetInfo {
    fn from(result: DispatchResult) -> Self {
        match result {
            Ok(_) => Self(true, ""),
            Err(error) => Self::from(error),
        }
    }
}
