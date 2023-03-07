// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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
#[allow(unused_imports)]
use crate::*;
// substrate
#[allow(unused_imports)]
use frame_support::log;

#[cfg(feature = "try-runtime")]
const ERROR_ACCOUNT: &str = "0x48900703a1bce72568051075f8e9dcf1d8ba61a2bab3cdfe96de1e701f891c2f";

pub struct CustomOnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		log::info!("Pre-check account: {ERROR_ACCOUNT}");

		let a = array_bytes::hex_n_into_unchecked::<_, sp_runtime::AccountId32, 32>(ERROR_ACCOUNT);

		assert_eq!(
			AccountMigration::ledger_of(&a).unwrap().staked_ring,
			52_500_000_000_000_000_000_u128
		);

		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		log::info!("Post-check account: {ERROR_ACCOUNT}");

		let a = array_bytes::hex_n_into_unchecked::<_, sp_runtime::AccountId32, 32>(ERROR_ACCOUNT);

		assert_eq!(
			AccountMigration::ledger_of(&a).unwrap().staked_ring,
			2_500_000_000_000_000_000_u128
		);

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		migrate()
	}
}

fn migrate() -> frame_support::weights::Weight {
	<darwinia_account_migration::Ledgers<Runtime>>::translate(
		|k, mut v: Option<darwinia_staking::Ledger<Runtime>>| {
			if let Some(v) = v.as_mut() {
				if let Some(ds) = <darwinia_account_migration::Deposits<Runtime>>::get(k) {
					v.staked_ring -= ds.into_iter().map(|d| d.value).sum::<Balance>();
				}

				v.unstaking_ring.retain(|u| u.1 != 0);
				v.unstaking_kton.retain(|u| u.1 != 0);
			}

			v
		},
	);

	// frame_support::weights::Weight::zero()
	RuntimeBlockWeights::get().max_block
}
