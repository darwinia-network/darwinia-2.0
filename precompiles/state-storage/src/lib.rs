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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

// std
use core::marker::PhantomData;
// moonbeam
use precompile_utils::prelude::*;

const PALLET_PREFIX_LENGTH: usize = 16;

pub trait StorageFilterT {
	fn allow(prefix: &[u8]) -> bool;
}

pub struct StateStorage<Runtime, Filter> {
	_marker: PhantomData<(Runtime, Filter)>,
}

#[precompile_utils::precompile]
impl<Runtime, Filter> StateStorage<Runtime, Filter>
where
	Runtime: pallet_evm::Config,
	Filter: StorageFilterT,
{
	#[precompile::public("state_storage(bytes)")]
	#[precompile::view]
	fn state_storage_at(
		handle: &mut impl PrecompileHandle,
		key: UnboundedBytes,
	) -> EvmResult<UnboundedBytes> {
		handle.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;

		let bytes = key.as_bytes();
		if bytes.len() < PALLET_PREFIX_LENGTH || !Filter::allow(&bytes[0..PALLET_PREFIX_LENGTH]) {
			return Err(revert("Read restriction"));
		}

		let output = frame_support::storage::unhashed::get_raw(&bytes);
		Ok(output.unwrap_or_default().as_slice().into())
	}
}
