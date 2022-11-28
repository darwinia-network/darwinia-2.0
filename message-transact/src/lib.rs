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

// crates.io
use codec::{Decode, Encode, MaxEncodedLen};
use ethereum::{
	AccessListItem, BlockV2 as Block, LegacyTransactionMessage, Log, ReceiptV3 as Receipt,
	TransactionAction, TransactionV2 as Transaction,
};
use frame_support::sp_runtime::traits::UniqueSaturatedInto;
use scale_info::TypeInfo;
// frontier
use fp_ethereum::{TransactionData, ValidatedTransaction};
use fp_evm::{CheckEvmTransaction, CheckEvmTransactionConfig, InvalidEvmTransactionError};
use pallet_evm::{FeeCalculator, GasWeightMapping};
// substrate
use frame_support::{PalletError, RuntimeDebug};
use sp_core::{H160, U256};

pub use pallet::*;

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum LcmpEthOrigin {
	MessageTransact(H160),
}

pub fn ensure_message_transact<OuterOrigin>(o: OuterOrigin) -> Result<H160, &'static str>
where
	OuterOrigin: Into<Result<LcmpEthOrigin, OuterOrigin>>,
{
	match o.into() {
		Ok(LcmpEthOrigin::MessageTransact(n)) => Ok(n),
		_ => Err("bad origin: expected to be an Lcmp Ethereum transaction"),
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config + pallet_ethereum::Config {
		/// Invalid transaction error
		type InvalidEvmTransactionError: From<InvalidEvmTransactionError>;
		/// Handler for applying an already validated transaction
		type ValidatedTransaction: ValidatedTransaction;
	}

	/// Ethereum pallet errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Message validate invalid
		InvalidEvmTransactionError(InvalidTransactionWrapper),
		MessageTransactionError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		OriginFor<T>: Into<Result<LcmpEthOrigin, OriginFor<T>>>,
	{
		/// This call only comes from LCMP message layer.
		#[pallet::weight({
			let without_base_extrinsic_weight = true;
			<T as pallet_evm::Config>::GasWeightMapping::gas_to_weight({
				let transaction_data: TransactionData = transaction.into();
				transaction_data.gas_limit.unique_saturated_into()
			}, without_base_extrinsic_weight)
		})]
		pub fn message_transact(
			origin: OriginFor<T>,
			transaction: Transaction,
		) -> DispatchResultWithPostInfo {
			let source = ensure_message_transact(origin)?;
			let (who, _) = pallet_evm::Pallet::<T>::account_basic(&source);

			let extracted_transaction = match transaction {
				Transaction::Legacy(ref t) =>
					Ok(Transaction::Legacy(ethereum::LegacyTransaction {
						nonce: who.nonce,                               // auto set
						gas_price: T::FeeCalculator::min_gas_price().0, // auto set
						gas_limit: t.gas_limit,
						action: t.action,
						value: t.value,
						input: t.input.clone(),
						signature: t.signature.clone(), // not used.
					})),
				_ => Err(Error::<T>::MessageTransactionError),
			}?;

			let transaction_data: TransactionData = (&extracted_transaction).into();
			let _ = CheckEvmTransaction::<InvalidTransactionWrapper>::new(
				CheckEvmTransactionConfig {
					evm_config: T::config(),
					block_gas_limit: T::BlockGasLimit::get(),
					base_fee: U256::default(), // TODO: FIX ME,
					chain_id: T::ChainId::get(),
					is_transactional: true,
				},
				transaction_data.clone().into(),
			)
			.validate_in_block_for(&who)
			.and_then(|v| v.with_chain_id())
			.and_then(|v| v.with_base_fee())
			.and_then(|v| v.with_balance_for(&who))
			.map_err(|e| Error::<T>::InvalidEvmTransactionError(e))?;

			T::ValidatedTransaction::apply(source, extracted_transaction)
		}
	}
}

#[derive(Encode, Decode, TypeInfo, PalletError)]
pub enum InvalidTransactionWrapper {
	GasLimitTooLow,
	GasLimitTooHigh,
	GasPriceTooLow,
	PriorityFeeTooHigh,
	BalanceTooLow,
	TxNonceTooLow,
	TxNonceTooHigh,
	InvalidPaymentInput,
	InvalidChainId,
}

impl From<InvalidEvmTransactionError> for InvalidTransactionWrapper {
	fn from(validation_error: InvalidEvmTransactionError) -> Self {
		match validation_error {
			InvalidEvmTransactionError::GasLimitTooLow => InvalidTransactionWrapper::GasLimitTooLow,
			InvalidEvmTransactionError::GasLimitTooHigh =>
				InvalidTransactionWrapper::GasLimitTooHigh,
			InvalidEvmTransactionError::GasPriceTooLow => InvalidTransactionWrapper::GasPriceTooLow,
			InvalidEvmTransactionError::PriorityFeeTooHigh =>
				InvalidTransactionWrapper::PriorityFeeTooHigh,
			InvalidEvmTransactionError::BalanceTooLow => InvalidTransactionWrapper::BalanceTooLow,
			InvalidEvmTransactionError::TxNonceTooLow => InvalidTransactionWrapper::TxNonceTooLow,
			InvalidEvmTransactionError::TxNonceTooHigh => InvalidTransactionWrapper::TxNonceTooHigh,
			InvalidEvmTransactionError::InvalidPaymentInput =>
				InvalidTransactionWrapper::InvalidPaymentInput,
			InvalidEvmTransactionError::InvalidChainId => InvalidTransactionWrapper::InvalidChainId,
		}
	}
}
