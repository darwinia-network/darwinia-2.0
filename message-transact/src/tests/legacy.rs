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

// --- crates.io ---
use array_bytes::hex2bytes;
// --- darwinia-network ---
use crate::{mock::*, tests::*};
// --- paritytech ---
use frame_support::pallet_prelude::Weight;
use sp_core::U256;

pub fn legacy_erc20_creation_unsigned_transaction() -> LegacyUnsignedTransaction {
	LegacyUnsignedTransaction {
		nonce: U256::zero(),
		gas_price: U256::from(1),
		gas_limit: U256::from(0x100000),
		action: ethereum::TransactionAction::Create,
		value: U256::zero(),
		input: hex2bytes(ERC20_CONTRACT_BYTECODE).unwrap(),
	}
}

#[test]
fn test_dispatch_legacy_ethereum_transaction_works() {
	let alice = address_build(1);
	let relayer_account = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![(alice.address, 1000), (relayer_account.address, 1000)])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			let t = unsigned_tx.sign(&alice.private_key);
			let call =
				RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: t });
			let message = prepare_message(call);

			System::set_block_number(1);
			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer_account.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);

			assert!(result.dispatch_result);
			System::assert_has_event(RuntimeEvent::Dispatch(
				pallet_bridge_dispatch::Event::MessageDispatched(
					SOURCE_CHAIN_ID,
					mock_message_id,
					Ok(()),
				),
			));
		});
}

#[test]
fn test_dispatch_legacy_ethereum_transaction_weight_mismatch() {
	let alice = address_build(1);
	let relayer_account = address_build(2);

	ExtBuilder::default()
		.with_balances(vec![(alice.address, 1000), (relayer_account.address, 1000)])
		.build()
		.execute_with(|| {
			let mock_message_id = [0; 4];
			let mut unsigned_tx = legacy_erc20_creation_unsigned_transaction();
			// 62500001 * 16000 > 1_000_000_000_000
			unsigned_tx.gas_limit = U256::from(62500001);
			let t = unsigned_tx.sign(&alice.private_key);
			let call =
				RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: t });
			let message = prepare_message(call);

			System::set_block_number(1);
			let result = Dispatch::dispatch(
				SOURCE_CHAIN_ID,
				TARGET_CHAIN_ID,
				&relayer_account.address,
				mock_message_id,
				Ok(message),
				|_, _| Ok(()),
			);

			assert!(!result.dispatch_result);
			println!("{:?}", System::events());
			System::assert_has_event(RuntimeEvent::Dispatch(
				pallet_bridge_dispatch::Event::MessageWeightMismatch(
					SOURCE_CHAIN_ID,
					mock_message_id,
					Weight::from_ref_time(1249913722000),
					Weight::from_ref_time(1000000000000),
				),
			));
		});
}
