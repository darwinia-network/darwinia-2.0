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

#![cfg(test)]

mod mock;
use frame_system::AccountInfo;
use mock::*;

// darwinia
use dc_primitives::AccountId;
use pangolin_runtime::{AccountMigration, Runtime, RuntimeCall, RuntimeOrigin};
// substrate
use frame_support::{
	assert_err, assert_ok, migration, traits::Get, Blake2_128Concat, StorageHasher,
};
use pallet_balances::AccountData;
use sp_core::{sr25519::Pair, Encode, Pair as PairT, H160, H256, U256};
use sp_io::hashing::blake2_256;
use sp_runtime::{
	traits::ValidateUnsigned,
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	AccountId32, DispatchError, DispatchResult,
};
use sp_version::RuntimeVersion;

fn migrate(pair: Pair, to: AccountId, chain_id: u64, spec_name: &[u8]) -> DispatchResult {
	// let spec_name = b"Pangolin2".as_slice();
	let account_id = AccountId32::new(pair.public().0);

	let message = blake2_256(
		&[
			&blake2_256(&[&chain_id.to_le_bytes(), spec_name, b"::account-migration"].concat()),
			to.0.as_slice(),
		]
		.concat(),
	);
	let sig = pair.sign(&message);

	AccountMigration::pre_dispatch(&darwinia_account_migration::Call::migrate {
		from: account_id.clone(),
		to: to.into(),
		signature: sig.clone(),
	})
	.map_err(|e| match e {
		TransactionValidityError::Invalid(InvalidTransaction::Custom(e)) =>
			Box::leak(format!("err code: {}", e).into_boxed_str()),
		e @ _ => <&'static str>::from(e),
	})?;
	AccountMigration::migrate(RuntimeOrigin::none(), account_id, to.into(), sig)
}

#[test]
fn validate_substrate_account_not_found() {
	ExtBuilder::default().build().execute_with(|| {
		let pair = Pair::from_seed(b"00000000000000000000000000000001");
		let to = H160::default();

		assert_err!(
			migrate(
				pair,
				to.into(),
				<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
				<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
					.spec_name
					.as_bytes()
			),
			DispatchError::Other("err code: 1") // The migration source not exist.
		);
	});
}

#[test]
fn validate_evm_account_already_exist() {
	let to = H160::from_low_u64_be(33).into();
	ExtBuilder::default().with_balances(vec![(to, 100)]).build().execute_with(|| {
		let pair = Pair::from_seed(b"00000000000000000000000000000001");

		// Mocked account data
		let account = AccountInfo {
			nonce: 100,
			consumers: 1,
			providers: 1,
			sufficients: 1,
			data: AccountData { free: 100_000, reserved: 100, ..Default::default() },
		};
		migration::put_storage_value(
			b"AccountMigration",
			b"Accounts",
			&Blake2_128Concat::hash(&pair.public().0),
			account.encode(),
		);

		assert_err!(
			migrate(
				pair,
				to,
				<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
				<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
					.spec_name
					.as_bytes()
			),
			DispatchError::Other("err code: 0") // To account has been used.
		);

		assert_err!(
			migrate(
				pair,
				to,
				<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get() + 1,
				<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
					.spec_name
					.as_bytes()
			),
			DispatchError::Other("err code: 2") // Invalid signature
		);
	});
}