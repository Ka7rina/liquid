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

use liquid_abi_codec::{Decode, Encode};
use liquid_macro::seq;
use liquid_prelude::{string::String, vec::Vec};
use liquid_primitives::{types::*, Selector};

pub trait FnInput {
    type Input: Decode + 'static;
}

pub trait FnOutput {
    type Output: Encode + 'static;
}

pub trait FnSelector {
    const SELECTOR: Selector;
}

pub trait FnMutability {
    const IS_MUT: bool;
}

pub trait ExternalFn: FnInput + FnOutput + FnSelector + FnMutability {}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Parameter_Type: Sized {
    type T = Self;
}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Return_Type: Sized {
    type T = Self;
}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Input_Type: Sized {
    type T = Self;
}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Event_Data_Type: Sized {
    type T = Self;
}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Event_Topic_Type: Sized {
    type T = Self;

    fn topic(&self) -> Hash
    where
        Self: liquid_abi_codec::Encode,
    {
        self.encode().into()
    }
}

#[allow(non_camel_case_types)]
pub trait You_Should_Use_An_Valid_Field_Data_Type: Sized {
    type T = Self;
}

macro_rules! gen_impls {
    ($t:ty) => {
        impl You_Should_Use_An_Valid_Parameter_Type for $t {}
        impl You_Should_Use_An_Valid_Return_Type for $t {}
        impl You_Should_Use_An_Valid_Input_Type for $t {}
        impl You_Should_Use_An_Valid_Event_Data_Type for $t {}
        impl You_Should_Use_An_Valid_Event_Topic_Type for $t {}
        impl You_Should_Use_An_Valid_Field_Data_Type for $t {}
    };
}

macro_rules! impl_for_primitives {
    ($($t:ty,)*) => {
        $(
            gen_impls!($t);
        )*
    };
}

impl_for_primitives!(
    u8, u16, u32, u64, u128, u256, i8, i16, i32, i64, i128, i256, bool, Address,
);

seq!(N in 1..=32 {
    #(
        gen_impls!(Bytes#N);
    )*
});

impl You_Should_Use_An_Valid_Parameter_Type for String {}
impl You_Should_Use_An_Valid_Return_Type for String {}
impl You_Should_Use_An_Valid_Input_Type for String {}
impl You_Should_Use_An_Valid_Event_Data_Type for String {}
impl You_Should_Use_An_Valid_Event_Topic_Type for String {
    type T = Self;

    fn topic(&self) -> Hash {
        liquid_primitives::hash::hash(self.as_bytes()).into()
    }
}
impl You_Should_Use_An_Valid_Field_Data_Type for String {}


impl You_Should_Use_An_Valid_Parameter_Type for Bytes {}
impl You_Should_Use_An_Valid_Return_Type for Bytes {}
impl You_Should_Use_An_Valid_Input_Type for Bytes {}
impl You_Should_Use_An_Valid_Event_Data_Type for Bytes {}
impl You_Should_Use_An_Valid_Field_Data_Type for Bytes {}

impl<T> You_Should_Use_An_Valid_Parameter_Type for Vec<T> where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T> You_Should_Use_An_Valid_Return_Type for Vec<T> where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T> You_Should_Use_An_Valid_Input_Type for Vec<T> where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T> You_Should_Use_An_Valid_Event_Data_Type for Vec<T> where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T> You_Should_Use_An_Valid_Field_Data_Type for Vec<T> where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T, const N: usize> You_Should_Use_An_Valid_Parameter_Type for [T; N] where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T, const N: usize> You_Should_Use_An_Valid_Return_Type for [T; N] where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T, const N: usize> You_Should_Use_An_Valid_Input_Type for [T; N] where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T, const N: usize> You_Should_Use_An_Valid_Event_Data_Type for [T; N] where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

impl<T, const N: usize> You_Should_Use_An_Valid_Field_Data_Type for [T; N] where
    T: You_Should_Use_An_Valid_Parameter_Type
{
}

/// `()` can be used to indicate taking nothing from call data.
impl You_Should_Use_An_Valid_Input_Type for () {}

/// `()` can be used to indicate returning nothing.
impl You_Should_Use_An_Valid_Return_Type for () {}

/// For tuple types, implement `You_Should_Use_An_Valid_Return_Type` only.
/// Due to that tuple types can only be used in return value of a contract's method.
macro_rules! impl_for_tuple {
    ($first:tt,) => {
        impl<$first: You_Should_Use_An_Valid_Parameter_Type>
            You_Should_Use_An_Valid_Return_Type for ($first,)
        {
        }
    };
    ($first:tt, $($rest:tt,)+) => {
        impl<$first: You_Should_Use_An_Valid_Parameter_Type, $($rest),+>
        You_Should_Use_An_Valid_Return_Type for ($first, $($rest),+)
        where
            $($rest: You_Should_Use_An_Valid_Parameter_Type),+
        {
        }

        impl_for_tuple!($($rest,)+);
    };
}

// The max number of outputs of a contract's method is 16.
seq! (N in 0..16 {
    impl_for_tuple!(#(T#N,)*);
});

#[cfg(feature = "liquid-abi-gen")]
pub trait GenerateABI {
    fn generate_abi() -> liquid_abi_gen::ContractABI;
}
