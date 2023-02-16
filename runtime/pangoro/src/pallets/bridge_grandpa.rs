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

pub use pallet_bridge_grandpa::Instance1 as WithRococoGrandpa;

// darwinia
use crate::*;

pub type RococoHeadersToKeep = ConstU32<500>;

impl pallet_bridge_grandpa::Config<WithRococoGrandpa> for Runtime {
	type BridgedChain = bp_pangolin::DarwiniaLike;
	type HeadersToKeep = RococoHeadersToKeep;
	type MaxBridgedAuthorities = ConstU32<100_000>;
	type MaxBridgedHeaderSize = ConstU32<65536>;
	type MaxRequests = ConstU32<50>;
	type WeightInfo = ();
}