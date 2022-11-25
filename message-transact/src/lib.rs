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
use ethereum::{
	AccessListItem, BlockV2 as Block, LegacyTransactionMessage, Log, ReceiptV3 as Receipt,
	TransactionAction, TransactionV2 as Transaction,
};
// frontier
use fp_ethereum::{TransactionData, ValidatedTransaction};
use fp_evm::{CheckEvmTransaction, CheckEvmTransactionConfig, InvalidEvmTransactionError};
use pallet_ethereum::ensure_ethereum_transaction;
use pallet_evm::FeeCalculator;

pub use pallet::*;

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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]

		pub fn message_transact(
			origin: OriginFor<T>,
			transaction: Transaction,
		) -> DispatchResultWithPostInfo {
			let source = ensure_ethereum_transaction(origin)?;

			let extracted_transaction = match transaction {
				Transaction::Legacy(t) => Ok(Transaction::Legacy(ethereum::LegacyTransaction {
					nonce: pallet_evm::Pallet::<T>::account_basic(&source).0.nonce, // auto set
					gas_price: T::FeeCalculator::min_gas_price().0,                 // auto set
					gas_limit: t.gas_limit,
					action: t.action,
					value: t.value,
					input: t.input,
					signature: t.signature, // not used.
				})),
				_ => todo!(),
			}?;

			// Validate the transaction before apply

			T::ValidatedTransaction::apply(source, transaction)
		}
	}
}
