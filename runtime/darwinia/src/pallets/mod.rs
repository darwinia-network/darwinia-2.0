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

mod shared_imports {
	pub use sp_runtime::traits::{ConstU128, ConstU16, ConstU32, ConstU64, ConstU8};
}
pub use shared_imports::*;

// System stuffs.
mod system;
pub use system::*;

mod parachain_system;

mod timestamp;

mod parachain_info_;

// Monetary stuff.
mod balances;

mod transaction_payment;

// Consensus stuff.
mod authorship;

mod collator_selection;

mod session;
pub use session::*;

mod aura;

mod aura_ext;

// Governance stuff.
// Democracy: pallet_democracy = 11,
// Council: pallet_collective::<Instance1> = 12,
// TechnicalCommittee: pallet_collective::<Instance2> = 13,
// PhragmenElection: pallet_elections_phragmen = 14,
// TechnicalMembership: pallet_membership::<Instance1> = 15,
// Treasury: pallet_treasury = 16,
// Tips: pallet_tips = 17,

// Utility stuff.
mod sudo;

mod vesting;

mod utility;

// Identity: pallet_identity = 21,
// Scheduler: pallet_scheduler = 22,
// Preimage: pallet_preimage = 23,
// Proxy: pallet_proxy = 24,
// Multisig: pallet_multisig = 25,

// XCM stuff.
mod xcmp_queue;

mod polkadot_xcm;
pub use polkadot_xcm::*;

mod dmp_queue;

// EVM stuff.
mod ethereum;

mod evm;
pub use evm::*;

mod base_fee;
