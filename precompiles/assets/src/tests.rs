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
use crate::mock::{
	Account::{Alice, Bob, Precompile},
	Assets, ExtBuilder, InternalCall, PrecompilesValue, RuntimeOrigin, System, TestPrecompiles,
	TestRuntime, TEST_ID,
};
// moonbeam
use precompile_utils::{
	prelude::{Address, RuntimeHelper, UnboundedBytes},
	testing::{PrecompileTesterExt, PrecompilesModifierTester},
	EvmDataWriter,
};
// substrate
use frame_support::{assert_ok, StorageHasher, Twox128};
use sha3::{Digest, Keccak256};
use sp_core::U256;

fn precompiles() -> TestPrecompiles<TestRuntime> {
	PrecompilesValue::get()
}

#[test]
fn selectors() {
	assert!(InternalCall::balance_of_selectors().contains(&0x70a08231));
	assert!(InternalCall::total_supply_selectors().contains(&0x18160ddd));
	assert!(InternalCall::approve_selectors().contains(&0x095ea7b3));
	assert!(InternalCall::allowance_selectors().contains(&0xdd62ed3e));
	assert!(InternalCall::transfer_selectors().contains(&0xa9059cbb));
	assert!(InternalCall::transfer_from_selectors().contains(&0x23b872dd));
	assert!(InternalCall::name_selectors().contains(&0x06fdde03));
	assert!(InternalCall::symbol_selectors().contains(&0x95d89b41));
	assert!(InternalCall::decimals_selectors().contains(&0x313ce567));

	assert!(InternalCall::mint_selectors().contains(&0x40c10f19));
	assert!(InternalCall::burn_selectors().contains(&0x9dc29fac));
	assert!(InternalCall::freeze_selectors().contains(&0x8d1fdf2f));
	assert!(InternalCall::thaw_selectors().contains(&0x5ea20216));
	assert!(InternalCall::transfer_ownership_selectors().contains(&0xf0350c04));

	assert_eq!(
		crate::SELECTOR_LOG_TRANSFER,
		&Keccak256::digest(b"Transfer(address,address,uint256)")[..]
	);

	assert_eq!(
		crate::SELECTOR_LOG_APPROVAL,
		&Keccak256::digest(b"Approval(address,address,uint256)")[..]
	);
}

#[test]
fn selector_less_than_four_bytes() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));
		// This selector is only three bytes long when four are required.
		precompiles()
			.prepare_test(Alice, Precompile, vec![1u8, 2u8, 3u8])
			.execute_reverts(|output| output == b"Tried to read selector out of bounds");
	});
}

#[test]
fn no_selector_exists_but_length_is_right() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));

		precompiles()
			.prepare_test(Alice, Precompile, vec![1u8, 2u8, 3u8, 4u8])
			.execute_reverts(|output| output == b"Unknown selector");
	});
}

#[test]
fn modifiers() {
	ExtBuilder::default().with_balances(vec![(Alice.into(), 1000)]).build().execute_with(|| {
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));
		let mut tester = PrecompilesModifierTester::new(precompiles(), Alice, Precompile);

		tester.test_view_modifier(InternalCall::balance_of_selectors());
		tester.test_view_modifier(InternalCall::total_supply_selectors());
		tester.test_default_modifier(InternalCall::approve_selectors());
		tester.test_view_modifier(InternalCall::allowance_selectors());
		tester.test_default_modifier(InternalCall::transfer_selectors());
		tester.test_default_modifier(InternalCall::transfer_from_selectors());
		tester.test_view_modifier(InternalCall::name_selectors());
		tester.test_view_modifier(InternalCall::symbol_selectors());
		tester.test_view_modifier(InternalCall::decimals_selectors());

		tester.test_default_modifier(InternalCall::mint_selectors());
		tester.test_default_modifier(InternalCall::burn_selectors());
		tester.test_default_modifier(InternalCall::freeze_selectors());
		tester.test_default_modifier(InternalCall::thaw_selectors());
		tester.test_default_modifier(InternalCall::transfer_ownership_selectors());
	});
}

#[test]
fn get_total_supply() {
	ExtBuilder::default()
		.with_balances(vec![(Alice.into(), 1000), (Bob.into(), 2500)])
		.build()
		.execute_with(|| {
			assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));
			assert_ok!(Assets::mint(
				RuntimeOrigin::signed(Alice.into()),
				TEST_ID,
				Alice.into(),
				1000
			));

			precompiles()
				.prepare_test(Alice, Precompile, InternalCall::total_supply {})
				.expect_no_logs()
				.execute_returns_encoded(U256::from(1000u64));
		});
}

#[test]
fn get_balances_known_user() {
	ExtBuilder::default().with_balances(vec![(Alice.into(), 1000)]).build().execute_with(|| {
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(Alice.into()), TEST_ID, Alice.into(), 1000));

		precompiles()
			.prepare_test(
				Alice,
				Precompile,
				InternalCall::balance_of { who: Address(Alice.into()) },
			)
			.expect_no_logs()
			.execute_returns_encoded(U256::from(1000u64));
	});
}

#[test]
fn get_balances_unknown_user() {
	ExtBuilder::default().with_balances(vec![(Alice.into(), 1000)]).build().execute_with(|| {
		assert_ok!(Assets::force_create(RuntimeOrigin::root(), TEST_ID, Alice.into(), true, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(Alice.into()), TEST_ID, Alice.into(), 1000));

		precompiles()
			.prepare_test(Alice, Precompile, InternalCall::balance_of { who: Address(Bob.into()) })
			.expect_no_logs()
			.execute_returns_encoded(U256::from(0u64));
	});
}
