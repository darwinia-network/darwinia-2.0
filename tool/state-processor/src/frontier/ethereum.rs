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

// darwinia
use crate::*;

/// Copied from frontier. https://github.com/paritytech/frontier/blob/polkadot-v0.9.30/primitives/storage/src/lib.rs#L23
pub const PALLET_ETHEREUM_SCHEMA: &[u8] = b":ethereum_schema";

impl Processor {
	pub fn process_ethereum(&mut self) {
		log::info!("set PALLET_ETHEREUM_SCHEMA");
		let state = &mut self.shell_chain_spec.genesis.raw.top;
		state.insert(array_bytes::bytes2hex("0x", PALLET_ETHEREUM_SCHEMA), "0x3".into());
	}
}

#[test]
fn test_schema_key() {
	assert_eq!(
		array_bytes::bytes2hex("0x", PALLET_ETHEREUM_SCHEMA),
		"0x3a657468657265756d5f736368656d61"
	);
}
