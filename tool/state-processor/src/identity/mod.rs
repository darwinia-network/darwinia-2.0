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
use pallet_identity::{Data, Judgement, RegistrarInfo, Registration};
use sp_core::{blake2_128, crypto::AccountId32};
use subhasher::blake2_128_concat;

type Balance = u128;

// TODO, check if all network have the same value
const SubAccountDeposit: Balance = 2 * GWEI;

impl Processor {
	/// Only care about the solo chain, since parachains don't have identity now.
	pub fn process_identity(&mut self) {
		let mut identities = <Map<Registration<Balance, ConstU32<20>, ConstU32<100>>>>::default();
		let mut registrars =
			BoundedVec::<Option<RegistrarInfo<Balance, AccountId32>>, ConstU32<20>>::default();
		let mut super_of = Map::<(AccountId32, Data)>::default();
		let mut subs_of = Map::<(Balance, BoundedVec<AccountId32, ConstU32<100>>)>::default();

		self.solo_state
			.take_map(b"Identity", b"IdentityOf", &mut identities, get_hashed_key)
			.take_value(b"Identity", b"Registrars", &mut registrars)
			.take_map(b"Identity", b"SuperOf", &mut super_of, get_hashed_key)
			.take_map(b"Identity", b"SubsOf", &mut subs_of, get_hashed_key);

		log::info!("update identity deposit and judgements decimal.");
		identities.iter_mut().for_each(|(_k, v)| {
			v.deposit = v.deposit * GWEI;
			v.judgements.iter_mut().for_each(|(index, judgement)| {
				if let Judgement::FeePaid(amount) = judgement {
					*amount = *amount * GWEI;
				}
			});
		});

		log::info!("update registrars fee decimal.");
		registrars.iter_mut().for_each(|registar_info| match registar_info {
			Some(info) => {
				info.fee = info.fee * GWEI;
			},
			None => {
				log::error!("This gonna not matched, no request_judgement or provide_judgement in this case.");
			},
		});

		log::info!("remove subsOf and superOf");
		let mut account_infos = <Map<AccountInfo>>::default();
		self.shell_state.take_map(
			b"AccountMigration",
			b"Accounts",
			&mut account_infos,
			get_hashed_key,
		);
		subs_of.iter_mut().for_each(|(k, (subs_deposit, sub_ids))| {
			for id in sub_ids {
				let key = &array_bytes::bytes2hex("", &blake2_128_concat(&id.encode()));
				if let Some((super_id, _)) = super_of.get(key) {
					let deposit = SubAccountDeposit.min(*subs_deposit);
					*subs_deposit -= deposit;

					let super_key = &array_bytes::bytes2hex("", &blake2_128_concat(&super_id.encode()));
					if let Some(super_info) = account_infos.get_mut(super_key) {
						log::info!("super_id: {:?}, super_info: {:?}", super_id, super_info);
						super_info.data.reserved -= deposit * GWEI;
						log::info!("after super_id: {:?}, super_info: {:?}", super_id, super_info);
					}
				}
			}
		});
	}
}
