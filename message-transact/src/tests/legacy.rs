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
use codec::Encode;
// --- darwinia-network ---
use crate::mock::{
	Account::{Alice, Bob},
	*,
};
use bp_message_dispatch::{CallOrigin, MessageDispatch, MessagePayload, SpecVersion};
use bp_runtime::messages::DispatchFeePayment;
// --- paritytech ---
use frame_support::pallet_prelude::Weight;
use sp_core::U256;

const TEST_SPEC_VERSION: SpecVersion = 0;
pub(crate) type SubChainId = [u8; 4];
pub(crate) const SOURCE_CHAIN_ID: SubChainId = *b"srce";
pub(crate) const TARGET_CHAIN_ID: SubChainId = *b"trgt";
const TEST_WEIGHT: Weight = Weight::from_ref_time(1_000_000_000_000);

// This ERC-20 contract mints the maximum amount of tokens to the contract creator.
// pragma solidity ^0.5.0;
// import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v2.5.1/contracts/token/ERC20/ERC20.sol";
// contract MyToken is ERC20 {
//	 constructor() public { _mint(msg.sender, 2**256 - 1); }
// }
pub const ERC20_CONTRACT_BYTECODE: &str = "608060405234801561001057600080fd5b50610041337fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61004660201b60201c565b610291565b600073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1614156100e9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f45524332303a206d696e7420746f20746865207a65726f20616464726573730081525060200191505060405180910390fd5b6101028160025461020960201b610c7c1790919060201c565b60028190555061015d816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461020960201b610c7c1790919060201c565b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508173ffffffffffffffffffffffffffffffffffffffff16600073ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040518082815260200191505060405180910390a35050565b600080828401905083811015610287576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b610e3a806102a06000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c806370a082311161005b57806370a08231146101fd578063a457c2d714610255578063a9059cbb146102bb578063dd62ed3e1461032157610088565b8063095ea7b31461008d57806318160ddd146100f357806323b872dd146101115780633950935114610197575b600080fd5b6100d9600480360360408110156100a357600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610399565b604051808215151515815260200191505060405180910390f35b6100fb6103b7565b6040518082815260200191505060405180910390f35b61017d6004803603606081101561012757600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803590602001909291905050506103c1565b604051808215151515815260200191505060405180910390f35b6101e3600480360360408110156101ad57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291908035906020019092919050505061049a565b604051808215151515815260200191505060405180910390f35b61023f6004803603602081101561021357600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505061054d565b6040518082815260200191505060405180910390f35b6102a16004803603604081101561026b57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610595565b604051808215151515815260200191505060405180910390f35b610307600480360360408110156102d157600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610662565b604051808215151515815260200191505060405180910390f35b6103836004803603604081101561033757600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610680565b6040518082815260200191505060405180910390f35b60006103ad6103a6610707565b848461070f565b6001905092915050565b6000600254905090565b60006103ce848484610906565b61048f846103da610707565b61048a85604051806060016040528060288152602001610d7060289139600160008b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206000610440610707565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610bbc9092919063ffffffff16565b61070f565b600190509392505050565b60006105436104a7610707565b8461053e85600160006104b8610707565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008973ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610c7c90919063ffffffff16565b61070f565b6001905092915050565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b60006106586105a2610707565b8461065385604051806060016040528060258152602001610de160259139600160006105cc610707565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008a73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610bbc9092919063ffffffff16565b61070f565b6001905092915050565b600061067661066f610707565b8484610906565b6001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b600033905090565b600073ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff161415610795576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526024815260200180610dbd6024913960400191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16141561081b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526022815260200180610d286022913960400191505060405180910390fd5b80600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508173ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925836040518082815260200191505060405180910390a3505050565b600073ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16141561098c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526025815260200180610d986025913960400191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff161415610a12576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401808060200182810382526023815260200180610d056023913960400191505060405180910390fd5b610a7d81604051806060016040528060268152602001610d4a602691396000808773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610bbc9092919063ffffffff16565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610b10816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610c7c90919063ffffffff16565b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508173ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040518082815260200191505060405180910390a3505050565b6000838311158290610c69576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b83811015610c2e578082015181840152602081019050610c13565b50505050905090810190601f168015610c5b5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5060008385039050809150509392505050565b600080828401905083811015610cfa576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b809150509291505056fe45524332303a207472616e7366657220746f20746865207a65726f206164647265737345524332303a20617070726f766520746f20746865207a65726f206164647265737345524332303a207472616e7366657220616d6f756e7420657863656564732062616c616e636545524332303a207472616e7366657220616d6f756e74206578636565647320616c6c6f77616e636545524332303a207472616e736665722066726f6d20746865207a65726f206164647265737345524332303a20617070726f76652066726f6d20746865207a65726f206164647265737345524332303a2064656372656173656420616c6c6f77616e63652062656c6f77207a65726fa265627a7a72315820c7a5ffabf642bda14700b2de42f8c57b36621af020441df825de45fd2b3e1c5c64736f6c63430005100032";

fn prepare_message(
	call: RuntimeCall,
) -> <pallet_bridge_dispatch::Pallet<TestRuntime> as MessageDispatch<
	AccountId,
	<TestRuntime as pallet_bridge_dispatch::Config>::BridgeMessageId,
>>::Message {
	MessagePayload {
		spec_version: TEST_SPEC_VERSION,
		weight: TEST_WEIGHT,
		origin: CallOrigin::SourceAccount(AccountId::from_low_u64_be(1)),
		dispatch_fee_payment: DispatchFeePayment::AtSourceChain,
		call: MockEncodedCall(call.encode()),
	}
}

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
fn test_dispatch_basic_system_call_works() {
	let (pairs, mut ext) = new_test_ext(1);
	let relayer_account = &pairs[0];

	ext.execute_with(|| {
		let mock_message_id = [0; 4];
		let call = RuntimeCall::System(frame_system::Call::remark { remark: vec![] });
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
		assert!(!result.dispatch_fee_paid_during_dispatch);
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
fn test_dispatch_legacy_ethereum_transaction_works() {
	let (pairs, mut ext) = new_test_ext(2);
	let alice = &pairs[0];
	let relayer_account = &pairs[1];

	ext.execute_with(|| {
		let mock_message_id = [0; 4];
		let unsigned_tx = legacy_erc20_creation_unsigned_transaction();
		let t = unsigned_tx.sign(&alice.private_key);
		let call = RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: t });
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
	let (pairs, mut ext) = new_test_ext(2);
	let alice = &pairs[0];
	let relayer_account = &pairs[1];

	ext.execute_with(|| {
		let mock_message_id = [0; 4];
		let mut unsigned_tx = legacy_erc20_creation_unsigned_transaction();
		// 62500001 * 16000 > 1_000_000_000_000
		unsigned_tx.gas_limit = U256::from(62500001);
		let t = unsigned_tx.sign(&alice.private_key);
		let call = RuntimeCall::MessageTransact(crate::Call::message_transact { transaction: t });
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
		System::assert_has_event(RuntimeEvent::Dispatch(
			pallet_bridge_dispatch::Event::MessageWeightMismatch(
				SOURCE_CHAIN_ID,
				mock_message_id,
				Weight::from_ref_time(1000000016000),
				Weight::from_ref_time(1000000000000),
			),
		));
	});
}
