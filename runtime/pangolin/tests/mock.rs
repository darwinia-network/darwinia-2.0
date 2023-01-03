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
use dc_primitives::{AccountId, Balance};
use pangolin_runtime::{Runtime, System};
// parity
use sp_io::TestExternalities;

#[derive(Default, Clone)]
pub struct ExtBuilder {
	balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
	pub fn build(&mut self) -> TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		pallet_balances::GenesisConfig::<Runtime> { balances: self.balances.clone() }
			.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	pub fn with_balances(&mut self, balances: Vec<(AccountId, Balance)>) -> &mut Self {
		self.balances = balances;
		self
	}
}