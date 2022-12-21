// Read the demo items out from the origin crab.json and crab-parachain.json
// Compared with the data processed.json

use core::panic;

use crate::{AccountData, AccountInfo, State, GWEI};
use array_bytes::{bytes2hex, hex_n_into_unchecked};

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
fn nonce_adjust_for_account_id_32() {}

#[test]
fn nonce_adjust_for_evm_account() {}

// --- EVM & Ethereum ---

#[test]
fn evm_code_migrate() {}

#[test]
fn evm_account_storage_migrate() {}

// --- Staking ---
