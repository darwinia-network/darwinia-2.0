// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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

//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Debian`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("darwinia-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --execution
// wasm
// --heap-pages
// 4096
// --steps
// 2
// --repeat
// 1
// --chain
// darwinia-local
// --output
// runtime/darwinia/src/weights/
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	fn spend() -> Weight {
		// Minimum execution time: 591 nanoseconds.
		Weight::from_ref_time(591_000)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Storage: Treasury Proposals (r:0 w:1)
	fn propose_spend() -> Weight {
		// Minimum execution time: 52_473 nanoseconds.
		Weight::from_ref_time(52_473_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn reject_proposal() -> Weight {
		// Minimum execution time: 45_584 nanoseconds.
		Weight::from_ref_time(45_584_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Storage: Treasury Approvals (r:1 w:1)
	/// The range of component `p` is `[0, 99]`.
	fn approve_proposal(_p: u32, ) -> Weight {
		// Minimum execution time: 13_228 nanoseconds.
		Weight::from_ref_time(20_495_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	fn remove_approval() -> Weight {
		// Minimum execution time: 9_446 nanoseconds.
		Weight::from_ref_time(9_446_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury Deactivated (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Storage: Treasury Approvals (r:1 w:1)
	// Storage: Treasury Proposals (r:100 w:0)
	/// The range of component `p` is `[0, 100]`.
	fn on_initialize_proposals(_p: u32, ) -> Weight {
		// Minimum execution time: 48_789 nanoseconds.
		Weight::from_ref_time(227_996_000)
			.saturating_add(T::DbWeight::get().reads(104))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
