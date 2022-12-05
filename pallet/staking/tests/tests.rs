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

mod mock;
use mock::*;

// darwinia
use darwinia_staking::*;
use dc_types::UNIT;
// substrate
use frame_support::{assert_ok, BoundedVec};

#[test]
fn stake_should_work() {
	new_test_ext().execute_with(|| {
		assert!(Staking::ledger_of(1).is_none());
		assert_ok!(Staking::stake(RuntimeOrigin::signed(1), UNIT, 0, vec![]));
		assert_eq!(
			Staking::ledger_of(1).unwrap(),
			Ledger {
				account: 1,
				staked_ring: UNIT,
				staked_kton: 0,
				staked_deposits: Default::default(),
				unstaking_ring: Default::default(),
				unstaking_kton: Default::default()
			}
		);

		assert_ok!(Staking::stake(RuntimeOrigin::signed(1), 0, UNIT, vec![]));
		assert_eq!(
			Staking::ledger_of(1).unwrap(),
			Ledger {
				account: 1,
				staked_ring: UNIT,
				staked_kton: UNIT,
				staked_deposits: Default::default(),
				unstaking_ring: Default::default(),
				unstaking_kton: Default::default()
			}
		);

		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), UNIT, 1));
		assert_ok!(Staking::stake(RuntimeOrigin::signed(1), 0, 0, vec![0]));
		assert_eq!(
			Staking::ledger_of(1).unwrap(),
			Ledger {
				account: 1,
				staked_ring: UNIT,
				staked_kton: UNIT,
				staked_deposits: BoundedVec::truncate_from(vec![0]),
				unstaking_ring: Default::default(),
				unstaking_kton: Default::default()
			}
		);

		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), 200 * UNIT, 1));
		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), 200 * UNIT, 1));
		assert_ok!(Staking::stake(RuntimeOrigin::signed(1), 500 * UNIT, 500 * UNIT, vec![1, 2]));
		assert_eq!(
			Staking::ledger_of(1).unwrap(),
			Ledger {
				account: 1,
				staked_ring: 501 * UNIT,
				staked_kton: 501 * UNIT,
				staked_deposits: BoundedVec::truncate_from(vec![0, 1, 2]),
				unstaking_ring: Default::default(),
				unstaking_kton: Default::default()
			}
		);
	});
}

#[test]
fn staking_pool_should_work() {
	new_test_ext().execute_with(|| {
		// assert_ok!(Darwinia::stake(Origin::signed(1), 1));
	});
}
