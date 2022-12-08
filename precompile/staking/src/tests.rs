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
	ExtBuilder, PCall, PrecompilesValue, Staking, System, TestPrecompiles, TestRuntime,
};
use sp_runtime::Perbill;
// moonbeam
use precompile_utils::{testing::PrecompileTesterExt, EvmDataWriter};
// substrate
use sp_core::{H160, U256};

fn precompiles() -> TestPrecompiles<TestRuntime> {
	PrecompilesValue::get()
}

#[test]
fn selectors() {
	assert!(PCall::stake_selectors().contains(&0x757f9b3b));
	assert!(PCall::unstake_selectors().contains(&0xef20fcb3));
	assert!(PCall::claim_selectors().contains(&0x4e71d92d));
	assert!(PCall::nominate_selectors().contains(&0xb332180b));
	assert!(PCall::collect_selectors().contains(&0x10a66536));
	assert!(PCall::chill_selectors().contains(&0x2b8a3ae6));
}

#[test]
fn stake_and_unstake() {
	let alice: H160 = Alice.into();
	ExtBuilder::default().with_balances(vec![(alice, 300)]).build().execute_with(|| {
		// stake
		precompiles()
			.prepare_test(
				alice,
				Precompile,
				PCall::stake {
					ring_amount: 200.into(),
					kton_amount: U256::zero(),
					deposits: vec![],
				},
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert_eq!(Staking::ledger_of(alice).unwrap().staked_ring, 200);

		// unstake
		precompiles()
			.prepare_test(
				alice,
				Precompile,
				PCall::unstake {
					ring_amount: 200.into(),
					kton_amount: U256::zero(),
					deposits: vec![],
				},
			)
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert_eq!(Staking::ledger_of(alice).unwrap().staked_ring, 0);
	});
}

#[test]
fn claim() {
	let alice: H160 = Alice.into();
	ExtBuilder::default().with_balances(vec![(alice, 300)]).build().execute_with(|| {
		// stake
		precompiles()
			.prepare_test(
				alice,
				Precompile,
				PCall::stake {
					ring_amount: 200.into(),
					kton_amount: U256::zero(),
					deposits: vec![],
				},
			)
			.execute_returns(EvmDataWriter::new().write(true).build());

		// unstake
		precompiles()
			.prepare_test(
				alice,
				Precompile,
				PCall::unstake {
					ring_amount: 200.into(),
					kton_amount: U256::zero(),
					deposits: vec![],
				},
			)
			.execute_returns(EvmDataWriter::new().write(true).build());

		// You have to wait for MinStakingDuration to claim
		System::set_block_number(5);
		precompiles()
			.prepare_test(alice, Precompile, PCall::claim {})
			.execute_returns(EvmDataWriter::new().write(true).build());
		assert!(Staking::ledger_of(alice).is_none());
	});
}

#[test]
fn nominate() {
	let alice: H160 = Alice.into();
	let bob: H160 = Bob.into();
	ExtBuilder::default().with_balances(vec![(alice, 300), (bob, 300)]).build().execute_with(
		|| {
			// alice stake first as collator
			precompiles()
				.prepare_test(
					alice,
					Precompile,
					PCall::stake {
						ring_amount: 200.into(),
						kton_amount: U256::zero(),
						deposits: vec![],
					},
				)
				.execute_returns(EvmDataWriter::new().write(true).build());

			// check the collator commission
			precompiles()
				.prepare_test(alice, Precompile, PCall::collect { commission: 30 })
				.execute_returns(EvmDataWriter::new().write(true).build());
			assert_eq!(Staking::collator_of(alice).unwrap(), Perbill::from_percent(30));

			// bob stake as nominator
			precompiles()
				.prepare_test(
					bob,
					Precompile,
					PCall::stake {
						ring_amount: 200.into(),
						kton_amount: U256::zero(),
						deposits: vec![],
					},
				)
				.execute_returns(EvmDataWriter::new().write(true).build());

			// check nominate result
			precompiles()
				.prepare_test(bob, Precompile, PCall::nominate { target: alice.into() })
				.execute_returns(EvmDataWriter::new().write(true).build());
			assert_eq!(Staking::nominator_of(bob).unwrap(), alice);

			// check alice(collator) chill
			precompiles()
				.prepare_test(alice, Precompile, PCall::chill {})
				.execute_returns(EvmDataWriter::new().write(true).build());
			assert!(Staking::collator_of(alice).is_none());

			// check bob(nominator) chill
			precompiles()
				.prepare_test(bob, Precompile, PCall::chill {})
				.execute_returns(EvmDataWriter::new().write(true).build());
			assert!(Staking::nominator_of(bob).is_none());
		},
	);
}
