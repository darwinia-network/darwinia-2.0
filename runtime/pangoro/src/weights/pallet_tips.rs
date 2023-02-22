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

//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Debian`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pangoro-local"), DB CACHE: 1024

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
// pangoro-local
// --output
// runtime/pangoro/src/weights/
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(_r: u32, ) -> Weight {
		// Minimum execution time: 46_857 nanoseconds.
		Weight::from_ref_time(70_025_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		// Minimum execution time: 55_255 nanoseconds.
		Weight::from_ref_time(55_255_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 7]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Minimum execution time: 40_286 nanoseconds.
		Weight::from_ref_time(39_133_916)
			// Standard Error: 173
			.saturating_add(Weight::from_ref_time(1_052).saturating_mul(r.into()))
			// Standard Error: 474_437
			.saturating_add(Weight::from_ref_time(164_583).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn tip(_t: u32, ) -> Weight {
		// Minimum execution time: 25_656 nanoseconds.
		Weight::from_ref_time(27_637_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn close_tip(_t: u32, ) -> Weight {
		// Minimum execution time: 59_909 nanoseconds.
		Weight::from_ref_time(73_823_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(_t: u32, ) -> Weight {
		// Minimum execution time: 30_181 nanoseconds.
		Weight::from_ref_time(32_038_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
