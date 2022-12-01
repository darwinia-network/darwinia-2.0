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
use crate::{
	mock::{Deposit, *},
	Deposit as DepositS, *,
};
use darwinia_staking::Stake;
use dc_types::UNIT;
// substrate
use frame_support::{assert_noop, assert_ok};

#[test]
fn lock_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(&Deposit::account_id()), 0);
		assert_eq!(Balances::free_balance(&1), 1_000 * UNIT);
		assert_eq!(Kton::free_balance(&1), 0);
		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), 10 * UNIT, 1));
		assert_eq!(Balances::free_balance(&Deposit::account_id()), 10 * UNIT);
		assert_eq!(Balances::free_balance(&1), 990 * UNIT);
		assert_eq!(Kton::free_balance(&1), 76_142_131_979_695);
	});
}

#[test]
fn lock_should_fail() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Deposit::lock(RuntimeOrigin::signed(1), 0, 0),
			<Error<Runtime>>::LockAtLeastSome
		);

		assert_noop!(
			Deposit::lock(RuntimeOrigin::signed(1), UNIT, 0),
			<Error<Runtime>>::LockAtLeastOneMonth
		);

		(0..<<Runtime as Config>::MaxDeposits as Get<_>>::get()).for_each(|_| {
			assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), UNIT, 1));
		});
		assert_noop!(
			Deposit::lock(RuntimeOrigin::signed(1), UNIT, 1),
			<Error<Runtime>>::ExceedMaxDeposits
		);

		assert_noop!(
			Deposit::lock(RuntimeOrigin::signed(2), 2_001 * UNIT, 1),
			<pallet_balances::Error<Runtime>>::InsufficientBalance
		);
	});
}

#[test]
fn claim_should_work() {
	new_test_ext().execute_with(|| {
		assert!(Deposit::deposit_of(&1).is_empty());
		assert_ok!(Deposit::claim(RuntimeOrigin::signed(1)));
		assert!(Deposit::deposit_of(&1).is_empty());

		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), UNIT, 1));
		assert!(!Deposit::deposit_of(&1).is_empty());

		Time::run(MILLISECS_PER_YEAR / 12 - 1);
		assert_ok!(Deposit::claim(RuntimeOrigin::signed(1)));
		assert!(!Deposit::deposit_of(&1).is_empty());

		Time::run(MILLISECS_PER_YEAR / 12);
		assert_ok!(Deposit::claim(RuntimeOrigin::signed(1)));
		assert!(Deposit::deposit_of(&1).is_empty());

		assert_ok!(Deposit::lock(RuntimeOrigin::signed(1), UNIT, 1));
		assert_ok!(Deposit::stake(&1, 0));
		Time::run(MILLISECS_PER_YEAR / 12);
		assert_ok!(Deposit::claim(RuntimeOrigin::signed(1)));
		assert!(!Deposit::deposit_of(&1).is_empty());

		assert_ok!(Deposit::unstake(&1, 0));
		assert_ok!(Deposit::claim(RuntimeOrigin::signed(1)));
		assert!(Deposit::deposit_of(&1).is_empty());
	});
}
