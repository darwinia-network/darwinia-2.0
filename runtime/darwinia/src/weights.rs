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

//! Expose the auto generated weight files.

#![allow(clippy::unnecessary_cast)]

pub mod block_weights;
pub use block_weights::constants::BlockExecutionWeight;

pub mod extrinsic_weights;
pub use extrinsic_weights::constants::ExtrinsicBaseWeight;

pub mod paritydb_weights;
pub use paritydb_weights::constants::ParityDbWeight;

pub mod rocksdb_weights;
pub use rocksdb_weights::constants::RocksDbWeight;

pub mod cumulus_pallet_xcmp_queue;
pub mod darwinia_deposit;
pub mod frame_system;
pub mod pallet_assets;
pub mod pallet_balances;
pub mod pallet_bridge_grandpa;
pub mod pallet_collective_council;
pub mod pallet_collective_technical_committee;
pub mod pallet_democracy;
pub mod pallet_elections_phragmen;
pub mod pallet_fee_market;
pub mod pallet_identity;
pub mod pallet_membership;
pub mod pallet_preimage;
pub mod pallet_proxy;
pub mod pallet_scheduler;
pub mod pallet_session;
pub mod pallet_timestamp;
pub mod pallet_tips;
pub mod pallet_treasury;
pub mod pallet_utility;
pub mod pallet_vesting;
