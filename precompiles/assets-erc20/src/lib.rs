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

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

// std
use core::marker::PhantomData;
// substrate
use sp_core::{H160, H256, U256};
use sp_std::convert::{TryFrom, TryInto};
// moonbeam
use precompile_utils::prelude::*;

pub struct ERC20Assets<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
#[precompile::precompile_set]
impl<Runtime> ERC20Assets<Runtime>
where
	Runtime: pallet_assets::Config,
{
	#[precompile::discriminant]
	fn discriminant(address: H160) -> Option<u32> {
		None
	}

	#[precompile::public("totalSupply()")]
	#[precompile::view]
	fn total_supply(asset_id: u32, handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("balanceOf(address)")]
	#[precompile::view]
	fn balance_of(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		who: Address,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("allowance(address,address)")]
	#[precompile::view]
	fn allowance(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		owner: Address,
		spender: Address,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("approve(address,uint256)")]
	fn approve(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		spender: Address,
		value: U256,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("transfer(address,uint256)")]
	fn transfer(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		to: Address,
		value: U256,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("transferFrom(address,address,uint256)")]
	fn transfer_from(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		from: Address,
		to: Address,
		value: U256,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("name()")]
	#[precompile::view]
	fn name(asset_id: u32, handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("symbol()")]
	#[precompile::view]
	fn symbol(asset_id: u32, handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("decimals()")]
	#[precompile::view]
	fn decimals(asset_id: u32, handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("mint(address,uint256)")]
	fn mint(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		to: Address,
		value: U256,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("burn(address,uint256)")]
	fn burn(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		from: Address,
		value: U256,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("transferOwnership(address)")]
	#[precompile::public("transfer_ownership(address)")]
	fn transfer_ownership(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		owner: Address,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("freeze(address)")]
	fn freeze(
		asset_id: u32,
		handle: &mut impl PrecompileHandle,
		account: Address,
	) -> EvmResult<()> {
		Ok(())
	}

	#[precompile::public("thaw(address)")]
	fn thaw(asset_id: u32, handle: &mut impl PrecompileHandle, account: Address) -> EvmResult<()> {
		Ok(())
	}
}
