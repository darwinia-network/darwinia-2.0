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

//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-21, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Debian`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pangolin-local"), DB CACHE: 1024

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
// pangolin-local
// --output
// runtime/pangolin/src/weights/
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	// Storage: BridgeMoonbaseGrandpa PalletOperatingMode (r:1 w:0)
	// Storage: BridgeMoonbaseGrandpa RequestCount (r:1 w:1)
	// Storage: BridgeMoonbaseGrandpa BestFinalized (r:1 w:1)
	// Storage: BridgeMoonbaseGrandpa ImportedHeaders (r:1 w:2)
	// Storage: BridgeMoonbaseGrandpa CurrentAuthoritySet (r:1 w:0)
	// Storage: BridgeMoonbaseGrandpa ImportedHashesPointer (r:1 w:1)
	// Storage: BridgeMoonbaseGrandpa ImportedHashes (r:1 w:1)
	/// The range of component `p` is `[51, 102]`.
	/// The range of component `v` is `[50, 100]`.
	fn submit_finality_proof(p: u32, _v: u32, ) -> Weight {
		// Minimum execution time: 1_848_880 nanoseconds.
		Weight::from_ref_time(1_848_880_000)
			// Standard Error: 5_778_241
			.saturating_add(Weight::from_ref_time(21_940_097).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
