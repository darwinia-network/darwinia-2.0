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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Debian`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("crab-local"), DB CACHE: 1024

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
// crab-local
// --output
// runtime/crab/src/weights/
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	// Storage: Assets Asset (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 23_393 nanoseconds.
		Weight::from_ref_time(23_393_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_create() -> Weight {
		// Minimum execution time: 38_361 nanoseconds.
		Weight::from_ref_time(38_361_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn start_destroy() -> Weight {
		// Minimum execution time: 53_286 nanoseconds.
		Weight::from_ref_time(53_286_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:0)
	// Storage: System Account (r:1000 w:1000)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(_c: u32, ) -> Weight {
		// Minimum execution time: 28_953 nanoseconds.
		Weight::from_ref_time(11_140_037_000)
			.saturating_add(T::DbWeight::get().reads(2002))
			.saturating_add(T::DbWeight::get().writes(2001))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:0)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(_a: u32, ) -> Weight {
		// Minimum execution time: 25_818 nanoseconds.
		Weight::from_ref_time(5_856_860_000)
			.saturating_add(T::DbWeight::get().reads(1002))
			.saturating_add(T::DbWeight::get().writes(1001))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn finish_destroy() -> Weight {
		// Minimum execution time: 108_431 nanoseconds.
		Weight::from_ref_time(108_431_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn mint() -> Weight {
		// Minimum execution time: 47_200 nanoseconds.
		Weight::from_ref_time(47_200_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 42_744 nanoseconds.
		Weight::from_ref_time(42_744_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 52_584 nanoseconds.
		Weight::from_ref_time(52_584_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 45_738 nanoseconds.
		Weight::from_ref_time(45_738_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		// Minimum execution time: 54_208 nanoseconds.
		Weight::from_ref_time(54_208_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn freeze() -> Weight {
		// Minimum execution time: 24_619 nanoseconds.
		Weight::from_ref_time(24_619_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn thaw() -> Weight {
		// Minimum execution time: 24_948 nanoseconds.
		Weight::from_ref_time(24_948_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn freeze_asset() -> Weight {
		// Minimum execution time: 21_495 nanoseconds.
		Weight::from_ref_time(21_495_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn thaw_asset() -> Weight {
		// Minimum execution time: 23_834 nanoseconds.
		Weight::from_ref_time(23_834_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 24_822 nanoseconds.
		Weight::from_ref_time(24_822_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn set_team() -> Weight {
		// Minimum execution time: 22_826 nanoseconds.
		Weight::from_ref_time(22_826_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Minimum execution time: 22_897 nanoseconds.
		Weight::from_ref_time(30_287_500)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 25_979 nanoseconds.
		Weight::from_ref_time(25_979_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 24_245 nanoseconds.
		Weight::from_ref_time(21_384_000)
			// Standard Error: 283_571
			.saturating_add(Weight::from_ref_time(57_220).saturating_mul(n.into()))
			// Standard Error: 283_571
			.saturating_add(Weight::from_ref_time(179_140).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_clear_metadata() -> Weight {
		// Minimum execution time: 22_005 nanoseconds.
		Weight::from_ref_time(22_005_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_asset_status() -> Weight {
		// Minimum execution time: 32_983 nanoseconds.
		Weight::from_ref_time(32_983_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 25_652 nanoseconds.
		Weight::from_ref_time(25_652_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_approved() -> Weight {
		// Minimum execution time: 65_233 nanoseconds.
		Weight::from_ref_time(65_233_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 25_381 nanoseconds.
		Weight::from_ref_time(25_381_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn force_cancel_approval() -> Weight {
		// Minimum execution time: 26_033 nanoseconds.
		Weight::from_ref_time(26_033_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
