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
use pangolin_runtime::{AccountMigration, RuntimeCall, RuntimeOrigin};
// substrate
use frame_support::assert_ok;
use sp_core::{sr25519::Pair, Pair as PairT, H160, H256, U256};
use sp_io::hashing::blake2_256;
use sp_runtime::{
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	AccountId32, DispatchResult,
};

fn migrate(pair: Pair, to: AccountId) -> DispatchResult {
	use sp_runtime::traits::ValidateUnsigned;
	let chain_id = 44u64;
	let spec_name = b"Pangolin2".as_slice();
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
			Box::leak(format!("code error: {}", e).into_boxed_str()),
		e @ _ => <&'static str>::from(e),
	})?;
	AccountMigration::migrate(RuntimeOrigin::none(), account_id, to.into(), sig)
}

#[test]
fn test_validate_unsigned() {
	ExtBuilder::default().build().execute_with(|| {
		let pair = Pair::from_seed(b"12345678901234567890123456789012");

		let to = H160::zero();
		assert_ok!(migrate(pair, to.into()));
	});
}
