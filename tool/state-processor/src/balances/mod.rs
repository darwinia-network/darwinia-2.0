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
use crate::*;

impl Processor {
	pub fn process_balances(
		&mut self,
		ring_locks: &mut Map<Vec<BalanceLock>>,
		kton_locks: &mut Map<Vec<BalanceLock>>,
	) -> &mut Self {
		log::info!("take solo balance locks");
		self.solo_state
			.take::<Vec<BalanceLock>, _>(
				b"Balances",
				b"Locks",
				ring_locks,
				get_blake2_128_concat_suffix,
			)
			.take::<Vec<BalanceLock>, _>(
				b"Kton",
				b"Locks",
				kton_locks,
				get_blake2_128_concat_suffix,
			);

		// ---
		// Currently, there are only fee-market locks.
		// I suggest shutting down the fee-market before merging.
		// So, we could ignore the para balance locks migration.
		// ---

		log::info!("adjust solo balance lock decimals");
		ring_locks.iter_mut().for_each(|(_, v)| v.iter_mut().for_each(|l| l.amount *= GWEI));
		kton_locks.iter_mut().for_each(|(_, v)| v.iter_mut().for_each(|l| l.amount *= GWEI));

		self
	}
}
