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

use frame_support::{traits::ConstU32, BoundedVec};
use pallet_identity::{Data, RegistrarInfo, Registration};
use sp_core::crypto::AccountId32;

type Balance = u128;

impl Processor {
	/// Only care about the solo chain, since parachains don't have identity now.
	pub fn process_identity(&mut self) {
		let mut identities = Map::default();
		let mut registrars = Map::default();
		let mut super_of = Map::default();
		let mut sub_of = Map::default();

		self.solo_state.take::<Registration<Balance, ConstU32<20>, ConstU32<100>>, _>(
			b"Identity",
			b"IdentityOf",
			&mut identities,
			get_blake2_128_concat_suffix,
		);

		// TODO: Need more tests
		self.solo_state
			.take::<BoundedVec<Option<RegistrarInfo<Balance, AccountId32>>, ConstU32<20>>, _>(
				b"Identity",
				b"Registrars",
				&mut registrars,
				untouched_key,
			);

		self.solo_state.take::<(AccountId32, Data), _>(
			b"Identity",
			b"SuperOf",
			&mut super_of,
			get_blake2_128_concat_suffix,
		);

		self.solo_state.take::<(Balance, BoundedVec<AccountId32, ConstU32<100>>), _>(
			b"Identity",
			b"SubsOf",
			&mut sub_of,
			get_blake2_128_concat_suffix,
		);
	}
}
