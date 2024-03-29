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

//! Autogenerated weights for `frame_system`
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

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(_b: u32, ) -> Weight {
		// Minimum execution time: 21_369 nanoseconds.
		Weight::from_ref_time(910_391_000)
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(_b: u32, ) -> Weight {
		// Minimum execution time: 25_477 nanoseconds.
		Weight::from_ref_time(4_613_787_000)
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 15_737 nanoseconds.
		Weight::from_ref_time(15_737_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(_i: u32, ) -> Weight {
		// Minimum execution time: 16_295 nanoseconds.
		Weight::from_ref_time(555_614_000)
			.saturating_add(T::DbWeight::get().writes(1000))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(_i: u32, ) -> Weight {
		// Minimum execution time: 19_816 nanoseconds.
		Weight::from_ref_time(467_332_000)
			.saturating_add(T::DbWeight::get().writes(1000))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(_p: u32, ) -> Weight {
		// Minimum execution time: 14_907 nanoseconds.
		Weight::from_ref_time(822_938_000)
			.saturating_add(T::DbWeight::get().writes(1000))
	}
}
