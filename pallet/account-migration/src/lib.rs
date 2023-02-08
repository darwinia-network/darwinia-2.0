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

//! # Darwinia account migration pallet
//!
//! ## Overview
//!
//! Darwinia2 uses ECDSA as its signature algorithm instead of SR25519.
//! These two algorithm are not compatible.
//! Thus, an account migration is required.
//!
//! ## Technical detail
//!
//! Users must send an extrinsic themselves to migrate their account(s).
//! This extrinsic should be unsigned, the reason is the same as `pallet-claims`.
//! This extrinsic's payload must contain a signature to the new ECDSA address, signed by their
//! origin SR25519 key.
//!
//! This pallet will store all the account data from Darwinia1 and Darwinia Parachain.
//! This pallet's genesis will be write into the chain spec JSON directly.
//! The data will be processed off-chain(ly).
//! After the verification, simply perform a take & put operation.
//!
//! ```nocompile
//! user -> send extrinsic -> verify -> put(storages, ECDSA, take(storages, SR25519))
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

// darwinia
use darwinia_deposit::Deposit;
use darwinia_staking::Ledger;
use dc_primitives::{AccountId as AccountId20, AssetId, Balance, BlockNumber, Index};
// substrate
use frame_support::{
	log, migration,
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement::KeepAlive, LockableCurrency, WithdrawReasons},
	StorageHasher,
};
use frame_system::{pallet_prelude::*, AccountInfo, RawOrigin};
use pallet_balances::AccountData;
use pallet_identity::Registration;
use pallet_vesting::VestingInfo;
use sp_core::sr25519::{Public, Signature};
use sp_io::hashing;
use sp_runtime::{
	traits::{IdentityLookup, TrailingZeroInput, Verify},
	AccountId32, RuntimeDebug,
};
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	const KTON_ID: u64 = 1026;

	/// The migration destination was already taken by someone.
	pub const E_ACCOUNT_ALREADY_EXISTED: u8 = 0;
	/// The migration source was not exist.
	pub const E_ACCOUNT_NOT_FOUND: u8 = 1;
	/// Invalid signature.
	pub const E_INVALID_SIGNATURE: u8 = 2;
	/// The account is not a member of the multisig.
	pub const E_NOT_MULTISIG_MEMBER: u8 = 4;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config:
		frame_system::Config<
			Index = Index,
			BlockNumber = BlockNumber,
			AccountId = AccountId20,
			AccountData = AccountData<Balance>,
			Lookup = IdentityLookup<AccountId20>,
		> + pallet_assets::Config<Balance = Balance, AssetId = AssetId>
		+ pallet_balances::Config<Balance = Balance>
		+ pallet_vesting::Config<Currency = pallet_balances::Pallet<Self>>
		+ pallet_identity::Config<Currency = pallet_balances::Pallet<Self>>
		+ darwinia_deposit::Config
		+ darwinia_staking::Config
	{
		/// Override the [`frame_system::Config::RuntimeEvent`].
		type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[allow(missing_docs)]
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event {
		/// An account has been migrated.
		Migrated { from: AccountId32, to: AccountId20 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Exceed maximum vesting count.
		ExceedMaxVestings,
		/// Exceed maximum deposit count.
		ExceedMaxDeposits,
	}

	/// [`frame_system::Account`] data.
	///
	/// <https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/system/src/lib.rs#L545>
	#[pallet::storage]
	#[pallet::getter(fn account_of)]
	pub type Accounts<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountId32, AccountInfo<Index, AccountData<Balance>>>;

	/// [`pallet_asset::AssetAccount`] data.
	///
	/// https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L115
	#[pallet::storage]
	#[pallet::getter(fn kton_account_of)]
	pub type KtonAccounts<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, AssetAccount>;

	/// [`pallet_vesting::Vesting`] data.
	///
	/// <https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L188>
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn vesting_of)]
	pub type Vestings<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountId32, Vec<VestingInfo<Balance, BlockNumber>>>;

	/// [`darwinia_deposit::Deposits`] data.
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn deposit_of)]
	pub type Deposits<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, Vec<Deposit>>;

	/// [`pallet_identity::IdentityOf`] data.
	///
	/// <https://github.com/paritytech/substrate/blob/polkadot-v0.9.30/frame/identity/src/lib.rs#L163>
	#[pallet::storage]
	#[pallet::getter(fn identity_of)]
	pub type Identities<T: Config> = StorageMap<
		_,
		Twox64Concat,
		AccountId32,
		Registration<Balance, ConstU32<100>, ConstU32<100>>,
	>;

	/// [`darwinia_staking::Ledgers`] data.
	#[pallet::storage]
	#[pallet::getter(fn ledger_of)]
	pub type Ledgers<T: Config> = StorageMap<_, Blake2_128Concat, AccountId32, Ledger<T>>;

	/// Multisig migration caches.
	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn multisig_of)]
	pub type Multisigs<T: Config> = StorageMap<_, Identity, AccountId32, Multisig>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Migrate all the account data under the `from` to `to`.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn migrate(
			origin: OriginFor<T>,
			from: AccountId32,
			to: AccountId20,
			_signature: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			Self::migrate_inner(from, to)?;

			Ok(())
		}

		/// Similar to `migrate` but for multisig accounts.
		///
		/// The `_signature` should be provided by `who`.
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn migrate_multisig(
			origin: OriginFor<T>,
			who: AccountId32,
			others: Vec<AccountId32>,
			threshold: u16,
			to: AccountId20,
			_signature: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			let (members, multisig) = multisig_of(who, others, threshold);

			if threshold < 2 {
				Self::migrate_inner(multisig, to)?;
			} else {
				let mut members = members.into_iter().map(|m| (m, false)).collect::<Vec<_>>();

				// Set the status to `true`.
				//
				// Because the `_signature` was already been verified in `pre_dispatch`.
				members
					.last_mut()
					.expect("[pallet::account-migration] `members` will never be empty; qed")
					.1 = true;

				<Multisigs<T>>::insert(multisig, Multisig { migrate_to: to, members, threshold });
			}

			Ok(())
		}

		/// To complete the pending multisig migration.
		///
		/// The `_signature` should be provided by `submitter`.
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn complete_multisig_migration(
			origin: OriginFor<T>,
			multisig: AccountId32,
			submitter: AccountId32,
			_signature: Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			let mut multisig_info = <Multisigs<T>>::take(&multisig)
				.expect("[pallet::account-migration] already checked in `pre_dispatch`; qed");

			// Set the status to `true`.
			//
			// Because the `_signature` was already been verified in `pre_dispatch`.
			multisig_info
				.members
				.iter_mut()
				.find(|(a, _)| a == &submitter)
				.expect("[pallet::account-migration] already checked in `pre_dispatch`; qed")
				.1 = true;

			if multisig_info.members.iter().fold(0, |acc, (_, ok)| if *ok { acc + 1 } else { acc })
				>= multisig_info.threshold
			{
				Self::migrate_inner(multisig, multisig_info.migrate_to)?;
			} else {
				<Multisigs<T>>::insert(multisig, multisig_info);
			}

			Ok(())
		}
	}
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			match call {
				Call::migrate { from, to, signature } => {
					Self::pre_check_existing(from, to)?;

					Self::pre_check_signature(from, to, signature)
				},
				Call::migrate_multisig { who, others, threshold, to, signature } => {
					let (_, multisig) = multisig_of(who.to_owned(), others.to_owned(), *threshold);

					Self::pre_check_existing(&multisig, to)?;

					Self::pre_check_signature(who, to, signature)
				},
				Call::complete_multisig_migration { multisig, submitter, signature } => {
					let Some(multisig_info) = <Multisigs<T>>::get(multisig) else {
						return InvalidTransaction::Custom(E_ACCOUNT_NOT_FOUND).into();
					};

					if !multisig_info.members.iter().any(|(a, _)| a == submitter) {
						return InvalidTransaction::Custom(E_NOT_MULTISIG_MEMBER).into();
					}

					Self::pre_check_signature(submitter, &multisig_info.migrate_to, signature)
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
	impl<T> Pallet<T>
	where
		T: Config,
	{
		fn pre_check_existing(
			from: &AccountId32,
			to: &AccountId20,
		) -> Result<(), TransactionValidityError> {
			if !<Accounts<T>>::contains_key(from) {
				Err(InvalidTransaction::Custom(E_ACCOUNT_NOT_FOUND))?;
			}
			if <frame_system::Account<T>>::contains_key(to) {
				Err(InvalidTransaction::Custom(E_ACCOUNT_ALREADY_EXISTED))?;
			}

			Ok(())
		}

		fn pre_check_signature(
			from: &AccountId32,
			to: &AccountId20,
			signature: &Signature,
		) -> TransactionValidity {
			let message = sr25519_signable_message(T::Version::get().spec_name.as_ref(), to);

			if verify_sr25519_signature(from, &message, signature) {
				ValidTransaction::with_tag_prefix("account-migration")
					.and_provides(from)
					.priority(100)
					.longevity(TransactionLongevity::max_value())
					.propagate(true)
					.build()
			} else {
				InvalidTransaction::Custom(E_INVALID_SIGNATURE).into()
			}
		}

		fn migrate_inner(from: AccountId32, to: AccountId20) -> DispatchResult {
			let account = <Accounts<T>>::take(&from)
				.expect("[pallet::account-migration] already checked in `pre_dispatch`; qed");

			<frame_system::Account<T>>::insert(to, account);

			if let Some(a) = <KtonAccounts<T>>::take(&from) {
				migration::put_storage_value(
					b"Assets",
					b"Account",
					&[
						Blake2_128Concat::hash(&KTON_ID.encode()),
						Blake2_128Concat::hash(&to.encode()),
					]
					.concat(),
					a,
				);
			}
			if let Some(v) = <Vestings<T>>::take(&from) {
				let locked = v.iter().map(|v| v.locked()).sum();

				<pallet_vesting::Vesting<T>>::insert(
					to,
					BoundedVec::try_from(v).map_err(|_| <Error<T>>::ExceedMaxVestings)?,
				);

				// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L248
				let reasons = WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE;

				// https://github.dev/paritytech/substrate/blob/19162e43be45817b44c7d48e50d03f074f60fbf4/frame/vesting/src/lib.rs#L86
				<pallet_balances::Pallet<T>>::set_lock(*b"vesting ", &to, locked, reasons);
			}
			if let Some(i) = <Identities<T>>::take(&from) {
				migration::put_storage_value(
					b"Identity",
					b"IdentityOf",
					&Twox64Concat::hash(&to.encode()),
					i,
				);
			}
			{
				let mut rs = <pallet_identity::Pallet<T>>::registrars();

				for r in rs.iter_mut().flatten() {
					if r.account.0 == <AccountId32 as AsRef<[u8; 32]>>::as_ref(&from)[..20] {
						r.account = to;

						break;
					}
				}

				migration::put_storage_value(b"Identity", b"Registrars", &[], rs);
			}
			if let Some(l) = <Ledgers<T>>::take(&from) {
				if let Some(ds) = <Deposits<T>>::take(&from) {
					<pallet_balances::Pallet<T> as Currency<_>>::transfer(
						&to,
						&darwinia_deposit::account_id(),
						ds.iter().map(|d| d.value).sum(),
						KeepAlive,
					)?;
					<darwinia_deposit::Deposits<T>>::insert(
						to,
						BoundedVec::try_from(ds).map_err(|_| <Error<T>>::ExceedMaxDeposits)?,
					);
				}

				let staking_pot = darwinia_staking::account_id();
				<pallet_balances::Pallet<T> as Currency<_>>::transfer(
					&to,
					&staking_pot,
					l.staked_ring + l.unstaking_ring.iter().map(|(r, _)| r).sum::<Balance>(),
					KeepAlive,
				)?;

				let sum = l.staked_kton + l.unstaking_kton.iter().map(|(k, _)| k).sum::<Balance>();
				if let Some(amount) = <pallet_assets::Pallet<T>>::maybe_balance(KTON_ID, to) {
					if amount >= sum {
						<pallet_assets::Pallet<T>>::transfer(
							RawOrigin::Signed(to).into(),
							KTON_ID.into(),
							staking_pot,
							sum,
						)?;
					}
				}

				<darwinia_staking::Ledgers<T>>::insert(to, l);
			}

			Self::deposit_event(Event::Migrated { from, to });

			Ok(())
		}
	}
}
pub use pallet::*;

// Copy from <https://github.dev/paritytech/substrate/blob/polkadot-v0.9.30/frame/assets/src/types.rs#L115>.
// Due to its visibility.
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
pub struct AssetAccount {
	balance: Balance,
	is_frozen: bool,
	reason: ExistenceReason,
	extra: (),
}
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug)]
pub enum ExistenceReason {
	#[codec(index = 0)]
	Consumer,
	#[codec(index = 1)]
	Sufficient,
	#[codec(index = 2)]
	DepositHeld(Balance),
	#[codec(index = 3)]
	DepositRefunded,
}

#[allow(missing_docs)]
#[derive(Encode, Decode, TypeInfo, RuntimeDebug)]
pub struct Multisig {
	pub migrate_to: AccountId20,
	pub members: Vec<(AccountId32, bool)>,
	pub threshold: u16,
}

/// Build a Darwinia account migration message.
pub fn sr25519_signable_message(spec_name: &[u8], account_id_20: &AccountId20) -> Vec<u8> {
	[
		// https://github.com/polkadot-js/common/issues/1710
		b"<Bytes>I authorize the migration to ",
		// Ignore the EIP-55 here.
		//
		// Must call the `to_lowercase` on front end.
		array_bytes::bytes2hex("0x", account_id_20.0).as_bytes(),
		b", an unused address on ",
		spec_name,
		b". Sign this message to authorize using the Substrate key associated with the account on ",
		&spec_name[..spec_name.len() - 1],
		b" that you wish to migrate.</Bytes>",
	]
	.concat()
}

/// Verify the Sr25519 signature.
pub fn verify_sr25519_signature(
	public_key: &AccountId32,
	message: &[u8],
	signature: &Signature,
) -> bool {
	// Actually, `&[u8]` is `[u8; 32]` here.
	// But for better safety.
	let Ok(public_key) = &Public::try_from(public_key.as_ref()) else {
		log::error!("[pallet::account-migration] `public_key` must be valid; qed");

		return false;
	};

	signature.verify(message, public_key)
}

/// Calculate the multisig account.
pub fn multisig_of(
	who: AccountId32,
	others: Vec<AccountId32>,
	threshold: u16,
) -> (Vec<AccountId32>, AccountId32) {
	// https://github.com/paritytech/substrate/blob/3bc3742d5c0c5269353d7809d9f8f91104a93273/frame/multisig/src/lib.rs#L525
	fn multisig_of_inner(members: &[AccountId32], threshold: u16) -> AccountId32 {
		let entropy = (b"modlpy/utilisuba", members, threshold).using_encoded(hashing::blake2_256);

		Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref())).expect(
			"[pallet::account-migration] infinite length input; no invalid inputs for type; qed",
		)
	}

	let mut members = others;

	members.push(who);
	members.sort();

	let multisig = multisig_of_inner(&members, threshold);

	(members, multisig)
}
