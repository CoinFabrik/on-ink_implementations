// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
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

//! Types for the default environment.
//!
//! These are simple mirrored types from the default substrate FRAME configuration.
//! Their interfaces and functionality might not be complete.
//!
//! Users are required to provide their own type definitions and `Environment`
//! implementations in order to write ink! contracts for other chain configurations.
//!
//! # Note
//!
//! When authoring a contract, the concrete `Environment` are available via aliases
//! generated by the `lang` macro. Therefore all functionality of the concrete
//! types is accessible in the contract, not constrained by the required trait
//! bounds.
//!
//! Outside the contract and its tests (e.g. in the off-chain environment), where
//! there is no knowledge of the concrete types, the functionality is restricted to
//! the trait bounds on the `Environment` trait types.

use ink_primitives::{
    AccountId,
    Hash,
};
use ink_traits::Environment;
#[cfg(feature = "std")]
use scale_info::TypeInfo;

/// Placeholder for chains that have no defined chain extension.
pub enum NoChainExtension {}

/// The fundamental types of the default configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = 4;

    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Timestamp = Timestamp;
    type BlockNumber = BlockNumber;
    type ChainExtension = NoChainExtension;
}

/// The default balance type.
pub type Balance = u128;

/// The default timestamp type.
pub type Timestamp = u64;

/// The default gas type.
pub type Gas = u64;

/// The default block number type.
pub type BlockNumber = u32;
