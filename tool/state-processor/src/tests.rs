use core::panic;

use crate::*;
use array_bytes::{bytes2hex, hex_n_into_unchecked};
use parity_scale_codec::Encode;
use primitive_types::H256;

struct Tester {
	solo_state: State,
	para_state: State,
	processed_state: State,
}

impl Tester {
	fn new() -> Self {
		Self {
			solo_state: State::from_file("test-data/crab.json").unwrap(),
			para_state: State::from_file("test-data/pangolin-parachain.json").unwrap(),
			processed_state: State::from_file("test-data/processed.json").unwrap(),
		}
	}
}

fn run_test<T>(test: T)
where
	T: FnOnce(&Tester) -> () + panic::UnwindSafe,
{
	let tester = Tester::new();
	let result = std::panic::catch_unwind(|| test(&tester));
	assert!(result.is_ok())
}

// --- System & Balances & Asset ---

#[test]
fn account_adjust_solo_chain_account() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5HakQe5khJMA2iZ99mQy2uAG2pXgub7aAH8k8bTwpNufWsRg
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0xf4171e1b64c96cc17f601f28d002cb5fcd27eab8b6585e296f4652be5bf05550",
		);

		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut account_info,
		);
		assert_ne!(account_info.nonce, 0);
		assert_ne!(account_info.consumers, 0);
		assert_ne!(account_info.providers, 0);
		assert_eq!(account_info.sufficients, 0);
		assert_ne!(account_info.data.free, 0);
		assert_ne!(account_info.data.free_kton_or_misc_frozen, 0);

		// after migrate
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Accounts",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_account_info,
		);
		assert_eq!(account_info.consumers, migrated_account_info.consumers);
		assert_eq!(account_info.providers, migrated_account_info.providers);
		assert_eq!(account_info.sufficients + 1, migrated_account_info.sufficients);
		// nonce reset
		assert_eq!(migrated_account_info.nonce, 0);
		// decimal adjust
		assert_eq!(account_info.data.free * GWEI, migrated_account_info.data.free);
		// the kton part has been removed.
		assert_eq!(migrated_account_info.data.free_kton_or_misc_frozen, 0);

		//  the kton part moved to the assert pallet
		let mut asset_account = AssetAccount::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"KtonAccounts",
			&blake2_128_concat_to_string(addr.encode()),
			&mut asset_account,
		);
		assert_eq!(asset_account.balance, account_info.data.free_kton_or_misc_frozen * GWEI);
		assert!(!asset_account.is_frozen);
	});
}

#[test]
fn account_adjust_with_remaining_balance_solo_account() {
	run_test(|tester| {
		// This is a pure substrate account_id(not derived one)
		// https://crab.subscan.io/account/5HakQe5khJMA2iZ99mQy2uAG2pXgub7aAH8k8bTwpNufWsRg
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0xf4171e1b64c96cc17f601f28d002cb5fcd27eab8b6585e296f4652be5bf05550",
		);

		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut account_info,
		);
		let mut remaining_balance = u128::default();
		tester.solo_state.get_value(
			b"Ethereum",
			b"RemainingRingBalance",
			&blake2_128_concat_to_string(addr.encode()),
			&mut remaining_balance,
		);

		// after migrate
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Accounts",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_account_info,
		);
		assert_eq!(
			migrated_account_info.data.free,
			account_info.data.free * GWEI + remaining_balance
		);
	});
}

#[test]
fn evm_account_adjust() {
	run_test(|tester| {
		// https://crab.subscan.io/account/0x740d5718a79A8559fEeE8B00922F8Cd773A81D84(5ELRpquT7C3mWtjeqR9f69c4swcFDHAfYC9wP5JSDg8rJHxZ)
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x64766d3a00000000000000740d5718a79a8559feee8b00922f8cd773a81d84ad",
		);

		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut account_info,
		);
		assert_ne!(account_info.nonce, 0);
		assert_ne!(account_info.data.free, 0);

		// after migrate
		let migrate_addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0x740d5718a79A8559fEeE8B00922F8Cd773A81D84");
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"System",
			b"Account",
			&bytes2hex("", subhasher::blake2_128_concat(&migrate_addr)),
			&mut migrated_account_info,
		);
		// assert the nonce doesn't changed.
		assert_eq!(migrated_account_info.nonce, account_info.nonce);
		assert_eq!(migrated_account_info.consumers, account_info.consumers);
		assert_eq!(migrated_account_info.providers, account_info.providers);
		assert_eq!(migrated_account_info.sufficients, account_info.sufficients);
		assert_eq!(migrated_account_info.data.free, account_info.data.free * GWEI);
	});
}

#[test]
fn evm_contract_account_adjust_sufficients() {
	run_test(|tester| {
		// https://crab.subscan.io/account/0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2(5ELRpquT7C3mWtjeo2WC5kAYFWzyRP2h55XDwo8ogDNkjm4h)
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x64766d3a000000000000000050f880c35c31c13bfd9cbb7d28aafaeca3abd2d0",
		);

		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut account_info,
		);
		assert_eq!(account_info.sufficients, 0);

		// after migrated
		let migrate_addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2");
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"System",
			b"Account",
			&bytes2hex("", subhasher::blake2_128_concat(&migrate_addr)),
			&mut migrated_account_info,
		);
		assert_eq!(migrated_account_info.sufficients, 1);
	});
}

#[test]
fn account_adjust_for_both_solo_and_para_chain_account() {}

#[test]
fn ring_total_issuance() {
	run_test(|tester| {
		let mut total_issuance = u128::default();
		tester.solo_state.get_value(b"Balances", b"TotalIssuance", "", &mut total_issuance);
		assert_ne!(total_issuance, 0);

		// after migrate
		let mut migrated_total_issuance = u128::default();
		tester.processed_state.get_value(
			b"Balances",
			b"TotalIssuance",
			"",
			&mut migrated_total_issuance,
		);

		// TODO: (2230295244267321287000000000, 2230419970862321271000000000)
		// assert_eq!(total_issuance * GWEI, migrated_total_issuance);
	});
}

#[test]
fn kton_total_issuance() {}

#[test]
fn special_accounts() {}

#[test]
fn asset_creation() {
	run_test(|tester| {
		let mut details = AssetDetails::default();
		tester.processed_state.get_value(
			b"Assets",
			b"Asset",
			&blake2_128_concat_to_string(KTON_ID.encode()),
			&mut details,
		);
		assert!(details.accounts > 0);
		assert!(details.supply != 0);
		assert_eq!(details.min_balance, 1);
		assert_eq!(details.sufficients, details.accounts);
	});
}

#[test]
fn asset_metadata() {
	run_test(|tester| {
		let mut metadata = AssetMetadata::default();
		tester.processed_state.get_value(
			b"Assets",
			b"Metadata",
			&blake2_128_concat_to_string(KTON_ID.encode()),
			&mut metadata,
		);
		assert_eq!(metadata.decimals, 18);
		assert_eq!(metadata.symbol, b"KTON".to_vec());
		assert_eq!(metadata.name, b"Darwinia Commitment Token".to_vec());
	});
}

// --- EVM & Ethereum ---

#[test]
fn evm_code_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2
		let addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2");
		let mut code = Vec::<u8>::new();
		tester.solo_state.get_value(
			b"EVM",
			b"AccountCodes",
			&blake2_128_concat_to_string(addr.encode()),
			&mut code,
		);
		assert_ne!(code.len(), 0);

		// after migrate
		let mut migrated_code = Vec::<u8>::new();
		tester.processed_state.get_value(
			b"Evm",
			b"AccountCodes",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_code,
		);
		assert_eq!(code, migrated_code);
	});
}

#[test]
fn evm_account_storage_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2
		let addr: [u8; 20] =
			hex_n_into_unchecked::<_, _, 20>("0x0050f880c35c31c13bfd9cbb7d28aafaeca3abd2");

		let storage_item_len = tester.solo_state.0.iter().fold(0u32, |sum, (k, _)| {
			if k.starts_with(&full_key(
				b"EVM",
				b"AccountStorages",
				&blake2_128_concat_to_string(addr.encode()),
			)) {
				sum + 1
			} else {
				sum
			}
		});
		assert_ne!(storage_item_len, 0);

		let storage_key: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x2093bcd1218dc1519493ee712ddfee3f4ced2d74096331d39d4247147baf17e2",
		);
		let mut storage_value = H256::zero();
		tester.solo_state.get_value(
			b"EVM",
			b"AccountStorages",
			&format!(
				"{}{}",
				&blake2_128_concat_to_string(addr.encode()),
				&blake2_128_concat_to_string(storage_key),
			),
			&mut storage_value,
		);
		assert_ne!(storage_value, H256::zero());

		// after migrate
		let migrated_storage_item_len =
			tester.processed_state.0.iter().fold(0u32, |sum, (k, _)| {
				if k.starts_with(&full_key(
					b"Evm",
					b"AccountStorages",
					&blake2_128_concat_to_string(addr.encode()),
				)) {
					sum + 1
				} else {
					sum
				}
			});
		assert_eq!(storage_item_len, migrated_storage_item_len);

		let mut migrated_storage_value = H256::zero();
		tester.processed_state.get_value(
			b"Evm",
			b"AccountStorages",
			&format!(
				"{}{}",
				&blake2_128_concat_to_string(addr.encode()),
				&blake2_128_concat_to_string(storage_key),
			),
			&mut migrated_storage_value,
		);
		assert_eq!(storage_value, migrated_storage_value);
	});
}

// --- Staking ---

#[test]
fn bounded_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5FxS8ugbXi4WijFuNS45Wg3Z5QsdN8hLZMmo71afoW8hJP67
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0xac288b0d41a3dcb69b025f51d9ad76ee088339f1c27708e164f9b019c584897d",
		);

		let mut controller = [0u8; 32];
		tester.solo_state.get_value(
			b"Staking",
			b"Bonded",
			&twox64_concat_to_string(addr.encode()),
			&mut controller,
		);
		assert_ne!(controller, [0u8; 32]);

		// after migrate
		let mut migrated_controller = [0u8; 32];
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Bonded",
			&twox64_concat_to_string(addr.encode()),
			&mut migrated_controller,
		);
		assert_eq!(migrated_controller, controller);
	});
}

#[test]
fn deposit_items_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5Dfh9agy74KFmdYqxNGEWae9fE9pdzYnyCUJKqK47Ac64zqM
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x46eb701bdc7f74ffda9c4335d82b3ae8d4e52c5ac630e50d68ab99822e29b3f6",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.deposit_items.len(), 0);
		let deposits_sum: u128 = ledger.deposit_items.iter().map(|i| i.value).sum();

		// after migrate
		let mut migrated_deposits = Vec::<Deposit>::new();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Deposits",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_deposits,
		);
		assert_eq!(migrated_deposits.len(), ledger.deposit_items.len());
		ledger.deposit_items.iter().zip(migrated_deposits.iter()).for_each(|(old, new)| {
			assert_eq!(new.value, old.value * GWEI);
			assert_eq!(new.expired_time, old.expire_time as u128);
			assert!(new.in_use);
		});
		let migrated_deposits_sum: u128 = migrated_deposits.iter().map(|i| i.value).sum();
		assert_eq!(migrated_deposits_sum, deposits_sum * GWEI);
	});
}

#[test]
fn ledgers_staked_value_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5Dfh9agy74KFmdYqxNGEWae9fE9pdzYnyCUJKqK47Ac64zqM
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x46eb701bdc7f74ffda9c4335d82b3ae8d4e52c5ac630e50d68ab99822e29b3f6",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.active, 0);
		assert_ne!(ledger.active_kton, 0);

		// after migrate
		let mut migrated_ledger = Ledger::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Ledgers",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_ledger,
		);
		assert_eq!(migrated_ledger.staked_ring, ledger.active * GWEI);
		assert_eq!(migrated_ledger.staked_kton, ledger.active_kton * GWEI);
	});
}

#[test]
fn ledgers_unbondings_migrate() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5FGL7pMZFZK4zWX2y3CRABeqMpMjBq77LhfYipWoBAT9gJsa
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x8d92774046fd3dc60d41825023506ad5ad91bd0d66e9c1df325fc3cf89c2d317",
		);

		let mut ledger = StakingLedger::default();
		tester.solo_state.get_value(
			b"Staking",
			b"Ledger",
			&blake2_128_concat_to_string(addr.encode()),
			&mut ledger,
		);
		assert_ne!(ledger.ring_staking_lock.unbondings.len(), 0);

		// after migrate
		let mut migrated_ledger = Ledger::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Ledgers",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_ledger,
		);
		ledger
			.ring_staking_lock
			.unbondings
			.iter()
			.zip(migrated_ledger.unstaking_ring.iter())
			.for_each(|(old, (amount, util))| {
				assert_eq!(*amount, old.amount * GWEI);
				// TODO https://github.com/darwinia-network/darwinia-2.0/issues/158
				// assert_eq!(*util, old.until);
			});
	});
}

#[test]
fn ring_pool_migrate() {
	run_test(|tester| {
		let mut ring_pool = u128::default();
		tester.solo_state.get_value(b"Staking", b"RingPool", "", &mut ring_pool);
		assert_ne!(ring_pool, 0);

		// after migrate
		let mut migrated_ring_pool = u128::default();
		tester.processed_state.get_value(b"Staking", b"RingPool", "", &mut migrated_ring_pool);
		assert_eq!(migrated_ring_pool, ring_pool * GWEI);
	});
}

#[test]
fn kton_pool_migrate() {
	run_test(|tester| {
		let mut kton_pool = u128::default();
		tester.solo_state.get_value(b"Staking", b"KtonPool", "", &mut kton_pool);
		assert_ne!(kton_pool, 0);

		// after migrate
		let mut migrated_kton_pool = u128::default();
		tester.processed_state.get_value(b"Staking", b"KtonPool", "", &mut migrated_kton_pool);
		assert_eq!(migrated_kton_pool, kton_pool * GWEI);
	});
}

#[test]
fn elapsed_time_migrate() {
	run_test(|tester| {
		let mut elapsed_time = u64::default();
		tester.solo_state.get_value(b"Staking", b"LivingTime", "", &mut elapsed_time);
		assert_ne!(elapsed_time, 0);

		// after migrate
		let mut migrated_elapsed_time = u128::default();
		tester.processed_state.get_value(
			b"Staking",
			b"ElapsedTime",
			"",
			&mut migrated_elapsed_time,
		);
		assert_eq!(migrated_elapsed_time, elapsed_time as u128);
	});
}

// --- Vesting ---
#[test]
fn vesting_info_adjust() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5EFJA3K6uRfkLxqjhHyrkJoQjfhmhyVyVEG5XtPPBM6yCCxM
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x608c62275934b164899ca6270c4b89c5d84b2390d4316fda980cd1b3acfad525",
		);

		let mut vesting_info = VestingInfo::default();
		tester.solo_state.get_value(
			b"Vesting",
			b"Vesting",
			&blake2_128_concat_to_string(addr.encode()),
			&mut vesting_info,
		);
		assert_ne!(vesting_info.locked, 0);
		assert_ne!(vesting_info.starting_block, 0);

		// after migrate
		let mut migrated_vesting_info = VestingInfo::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Vestings",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_vesting_info,
		);

		assert_eq!(migrated_vesting_info.locked, vesting_info.locked * GWEI);
		assert_eq!(migrated_vesting_info.per_block, vesting_info.per_block * GWEI * 2);
		assert!(migrated_vesting_info.starting_block < vesting_info.starting_block);
	});
}

// --- Indices ---

#[test]
fn indices_adjust() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5ELRpquT7C3mWtjes9CNUiDpW1x3VwQYK7ZWq3kiH91UMftL
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x64766d3a00000000000000c7912465c55be41bd09325b393f4fbea73f26d473b",
		);
		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut account_info,
		);
		assert_ne!(account_info.data.reserved, 0);

		// after migrated
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"System",
			b"Account",
			&blake2_128_concat_to_string(addr.encode()),
			&mut migrated_account_info,
		);
		assert_ne!(
			migrated_account_info.data.free,
			(account_info.data.free + account_info.data.reserved) * GWEI
		);
		assert_eq!(migrated_account_info.data.reserved, 0);
	});
}
