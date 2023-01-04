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
use mock::*;

// darwinia
use dc_primitives::AccountId;
use pangolin_runtime::*;
// substrate
use frame_support::{
	assert_err, assert_ok, migration, traits::Get, Blake2_128Concat, StorageHasher,
};
use frame_system::AccountInfo;
use pallet_assets::ExistenceReason;
use pallet_balances::AccountData;
use sp_core::{sr25519::Pair, Encode, Pair as PairT, H160};
use sp_io::hashing::blake2_256;
use sp_runtime::{
	traits::ValidateUnsigned,
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	AccountId32, DispatchError, DispatchResult,
};
use sp_version::RuntimeVersion;

fn migrate(pair: Pair, to: AccountId, chain_id: u64, spec_name: &[u8]) -> DispatchResult {
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
		to,
		signature: sig.clone(),
	})
	.map_err(|e| match e {
		TransactionValidityError::Invalid(InvalidTransaction::Custom(e)) =>
			Box::leak(format!("err code: {}", e).into_boxed_str()),
		e => <&'static str>::from(e),
	})?;
	AccountMigration::migrate(RuntimeOrigin::none(), account_id, to, sig)
}

fn prepare_accounts(storage: bool) -> Pair {
	let pair = Pair::from_seed(b"00000000000000000000000000000001");
	let account_id = AccountId32::new(pair.public().0);

	if storage {
		<darwinia_account_migration::Accounts<Runtime>>::insert(
			account_id.clone(),
			AccountInfo {
				nonce: 100,
				consumers: 1,
				providers: 1,
				sufficients: 1,
				data: AccountData { free: 100, reserved: 100, ..Default::default() },
			},
		);
		assert!(AccountMigration::account_of(account_id).is_some());
	}
	pair
}

#[test]
fn validate_substrate_account_not_found() {
	ExtBuilder::default().build().execute_with(|| {
		let to = H160::default();
		let pair = prepare_accounts(false);

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
		let pair = prepare_accounts(true);

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
	});
}

#[test]
fn validate_invalid_sig() {
	let to = H160::from_low_u64_be(33).into();
	ExtBuilder::default().build().execute_with(|| {
		let pair = prepare_accounts(true);

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

#[test]
fn migrate_accounts() {
	let to = H160::from_low_u64_be(255).into();
	ExtBuilder::default().build().execute_with(|| {
		let pair = prepare_accounts(true);
		let account_id = AccountId32::new(pair.public().0);

		assert_ok!(migrate(
			pair,
			to,
			<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
			<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
				.spec_name
				.as_bytes()
		));
		assert_eq!(AccountMigration::account_of(account_id), None);
		assert_eq!(
			System::account(to),
			AccountInfo {
				nonce: 100,
				consumers: 1,
				providers: 1,
				sufficients: 1,
				data: AccountData { free: 100, reserved: 100, ..Default::default() },
			}
		);
	});
}

#[test]
fn migrate_kton_accounts() {
	let to = H160::from_low_u64_be(255).into();
	ExtBuilder::default().build().execute_with(|| {
		let pair = prepare_accounts(true);
		let account_id = AccountId32::new(pair.public().0);

		// The struct in the upstream repo is not accessible due to viable.
		#[derive(Encode)]
		pub struct AssetAccount {
			pub balance: u128,
			pub is_frozen: bool,
			pub reason: ExistenceReason<u128>,
			pub extra: (),
		}

		let asset_account = AssetAccount {
			balance: 100,
			is_frozen: false,
			reason: ExistenceReason::<u128>::Sufficient,
			extra: (),
		};
		assert_eq!(asset_account.encode().len(), 18);
		migration::put_storage_value(
			b"AccountMigration",
			b"KtonAccounts",
			&Blake2_128Concat::hash(account_id.as_ref()),
			asset_account.clone(),
		);

		assert_ok!(migrate(
			pair,
			to,
			<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
			<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
				.spec_name
				.as_bytes()
		));

		assert_eq!(
			Assets::maybe_balance(AssetIds::PKton as u64, to).unwrap(),
			asset_account.balance
		);
	});
}

#[test]
fn vesting() {
	let to = H160::from_low_u64_be(255).into();
	ExtBuilder::default().build().execute_with(|| {
		let pair = prepare_accounts(true);
		let account_id = AccountId32::new(pair.public().0);

		// The struct in the upstream repo is not accessible due to viable.
		#[derive(Encode)]
		pub struct VestingInfo {
			locked: u128,
			per_block: u128,
			starting_block: u32,
		}

		let vests = vec![
			VestingInfo { locked: 100, per_block: 5, starting_block: 0 },
			VestingInfo { locked: 100, per_block: 5, starting_block: 0 },
		];
		migration::put_storage_value(
			b"AccountMigration",
			b"Vestings",
			&Blake2_128Concat::hash(account_id.as_ref()),
			vests,
		);

		assert_ok!(migrate(
			pair,
			to,
			<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
			<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
				.spec_name
				.as_bytes()
		));

		assert_eq!(Vesting::vesting(to).unwrap().len(), 2);
		assert_eq!(Balances::locks(to).len(), 1);
	});
}

#[test]
#[ignore]
fn staking() {
	let to = H160::from_low_u64_be(255).into();
	ExtBuilder::default().build().execute_with(|| {
		let pair1 = prepare_accounts(true);
		let pair2 = Pair::from_seed(b"00000000000000000000000000000002");
		let account_id1 = AccountId32::new(pair1.public().0);
		let account_id2 = AccountId32::new(pair2.public().0);

		<darwinia_account_migration::Bonded<Runtime>>::insert(
			account_id1.clone(),
			account_id2.clone(),
		);
		<darwinia_account_migration::Deposits<Runtime>>::insert(
			account_id1.clone(),
			account_id2.clone(),
		);

		assert_ok!(migrate(
			pair,
			to,
			<<Runtime as pallet_evm::Config>::ChainId as Get<u64>>::get(),
			<<Runtime as frame_system::Config>::Version as Get<RuntimeVersion>>::get()
				.spec_name
				.as_bytes()
		));
	});
}
