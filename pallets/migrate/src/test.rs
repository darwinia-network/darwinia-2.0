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
use crate::{mock::*, ClaimMessage, Error};
// substrate
use frame_support::{assert_err, assert_ok};
use sp_core::{blake2_256, Pair, H160};

#[test]
fn claim_to_new_account() {
	let (pair, charilie) = SubAccounts::Charlie.to_pair();
	let alice: H160 = EthAccounts::Alice.into();

	ExtBuilder::default()
		.with_migrated_accounts(vec![(charilie.clone(), 1000)])
		.build()
		.execute_with(|| {
			let message = ClaimMessage::new(42, &charilie, &alice);
			let sig = pair.sign(&blake2_256(&message.raw_bytes())[..]);

			assert_eq!(Migrate::balance_of(charilie.clone()), Some(1000));
			assert_eq!(Balances::free_balance(alice), 0);
			assert_ok!(Migrate::claim_to(RuntimeOrigin::none(), 42, charilie.clone(), alice, sig));
			assert!(Migrate::balance_of(charilie).is_none());
			assert_eq!(Balances::free_balance(alice), 1000);
		});
}

#[test]
fn claim_with_not_exist_old_pub_key() {
	let (pair, charilie) = SubAccounts::Charlie.to_pair();
	let alice: H160 = EthAccounts::Alice.into();

	ExtBuilder::default().build().execute_with(|| {
		let message = ClaimMessage::new(42, &charilie, &alice);
		let sig = pair.sign(&blake2_256(&message.raw_bytes())[..]);

		assert_err!(
			Migrate::claim_to(RuntimeOrigin::none(), 42, charilie.clone(), alice, sig),
			Error::<TestRuntime>::AccountNotExist
		);
	});
}

#[test]
fn claim_to_existed_account() {
	let (pair, bogus) = SubAccounts::Bogus.to_pair();
	let bob: H160 = EthAccounts::Bob.into();

	ExtBuilder::default()
		.with_migrated_accounts(vec![(bogus.clone(), 1000)])
		.with_balances(vec![(bob, 500)])
		.build()
		.execute_with(|| {
			let message = ClaimMessage::new(42, &bogus, &bob);
			let sig = pair.sign(&blake2_256(&message.raw_bytes())[..]);

			assert_eq!(Migrate::balance_of(bogus.clone()), Some(1000));
			assert_eq!(Balances::free_balance(bob), 500);
			assert_ok!(Migrate::claim_to(RuntimeOrigin::none(), 42, bogus.clone(), bob, sig));
			assert!(Migrate::balance_of(bogus).is_none());
			assert_eq!(Balances::free_balance(bob), 1000 + 500);
		});
}

#[test]
fn claim_event() {
	let (pair, charilie) = SubAccounts::Charlie.to_pair();
	let alice: H160 = EthAccounts::Alice.into();

	ExtBuilder::default()
		.with_migrated_accounts(vec![(charilie.clone(), 1000)])
		.build()
		.execute_with(|| {
			let message = ClaimMessage::new(42, &charilie, &alice);
			let sig = pair.sign(&blake2_256(&message.raw_bytes())[..]);

			assert_ok!(Migrate::claim_to(RuntimeOrigin::none(), 42, charilie.clone(), alice, sig));
			System::assert_has_event(RuntimeEvent::Migrate(crate::Event::Claim {
				old_pub_key: charilie,
				new_pub_key: alice,
				amount: 1000,
			}))
		});
}
