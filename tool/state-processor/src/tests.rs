// Read the demo items out from the origin crab.json and crab-parachain.json
// Compared with the data processed.json

use core::panic;

use crate::{full_key, AccountData, AccountInfo, Map, State, GWEI};
use array_bytes::{bytes2hex, hex_n_into_unchecked};
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

// --- System & Balances ---

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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
			&mut migrated_account_info,
		);
		// assert pointer not changes
		assert_eq!(account_info.consumers, migrated_account_info.consumers);
		assert_eq!(account_info.providers, migrated_account_info.providers);
		assert_eq!(account_info.sufficients, migrated_account_info.sufficients);
		// assert nonce reset
		assert_eq!(migrated_account_info.nonce, 0);
		// the kton part has been removed.
		assert_eq!(migrated_account_info.data.free_kton_or_misc_frozen, 0);
		// assert decimal adjust
		assert_eq!(account_info.data.free * GWEI, migrated_account_info.data.free);
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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
			&mut account_info,
		);
		let mut remaining_balance = u128::default();
		tester.solo_state.get_value(
			b"Ethereum",
			b"RemainingRingBalance",
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
			&mut remaining_balance,
		);

		// after migrate
		let mut migrated_account_info = AccountInfo::default();
		tester.processed_state.get_value(
			b"AccountMigration",
			b"Accounts",
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
			&mut code,
		);
		assert_ne!(code.len(), 0);

		// after migrate
		let mut migrated_code = Vec::<u8>::new();
		tester.processed_state.get_value(
			b"Evm",
			b"AccountCodes",
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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

		let storage_item_len = tester.solo_state.0.iter().fold(0u32, |sum, (k, v)| {
			if k.starts_with(&full_key(
				b"EVM",
				b"AccountStorages",
				&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
				&bytes2hex("", subhasher::blake2_128_concat(&addr)),
				&bytes2hex("", subhasher::blake2_128_concat(&storage_key)),
			),
			&mut storage_value,
		);
		assert_ne!(storage_value, H256::zero());

		// after migrate
		let migrated_storage_item_len =
			tester.processed_state.0.iter().fold(0u32, |sum, (k, v)| {
				if k.starts_with(&full_key(
					b"Evm",
					b"AccountStorages",
					&bytes2hex("", subhasher::blake2_128_concat(&addr)),
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
				&bytes2hex("", subhasher::blake2_128_concat(&addr)),
				&bytes2hex("", subhasher::blake2_128_concat(&storage_key)),
			),
			&mut migrated_storage_value,
		);
		assert_eq!(storage_value, migrated_storage_value);
	});
}

// --- Staking ---

// --- Vesting ---
