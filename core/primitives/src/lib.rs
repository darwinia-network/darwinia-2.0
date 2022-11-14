// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

//! Darwinia chain core primitives.
//!
//! # Opaque types.
//! These are used by the CLI to instantiate machinery that don't need to know
//! the specifics of the runtime. They can then be made to be agnostic over specific formats
//! of data like extrinsics, allowing for them to continue syncing the network through upgrades
//! to even the core data structures.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = sp_runtime::MultiSignature;

/// Some way of identifying an account on the chain.
/// We intentionally make it equivalent to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// Nonce of an account in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Hashing type
pub type Hashing = sp_runtime::traits::BlakeTwo256;

/// Block header type.
pub type Header = sp_runtime::generic::Header<BlockNumber, Hashing>;

/// Block type.
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;

/// Darwinia EVM addresses specific prefix.
pub const EVM_ADDR_PREFIX: &[u8] = b"dvm:";

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6_000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
/// Slot duration.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
/// 10 blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
/// 600 blocks.
pub const HOURS: BlockNumber = MINUTES * 60;
/// 14,400 blocks.
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
/// 1e18 wei — 1,000,000,000,000,000,000
pub const UNIT: Balance = 1_000 * MILLIUNIT;
/// 1e15 wei — 1,000,000,000,000,000
pub const MILLIUNIT: Balance = 1_000 * MICROUNIT;
/// 1e12 wei — 1,000,000,000,000
pub const MICROUNIT: Balance = 1_000 * GWEI;
/// 1e9 wei — 1,000,000,000
pub const GWEI: Balance = 1_000 * MWEI;
/// 1e6 wei — 1,000,000
pub const MWEI: Balance = 1_000 * KWEI;
/// 1e3 wei — 1,000
pub const KWEI: Balance = 1_000 * WEI;
/// 1 wei — 1
pub const WEI: Balance = 1;
