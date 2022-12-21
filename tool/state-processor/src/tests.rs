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

// --- System ---

#[test]
fn account_adjust_for_only_solo_chain_account() {
	run_test(|tester| {
		// https://crab.subscan.io/account/5F2CnHR4JDJW4RXqXhH7dpk4tQuu2qCf7UPzZLX4exDg9VxE
		let addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
			"0x82cc5514cdaa945629347924da4b804735c1530be80a5e001ce0b413cc46aa47",
		);

		let mut account_info = AccountInfo::default();
		tester.solo_state.get_value(
			b"System",
			b"Account",
			&bytes2hex("", subhasher::blake2_128_concat(&addr)),
			&mut account_info,
		);

		assert_eq!(
			account_info,
			AccountInfo {
				nonce: 0,
				consumers: 0,
				providers: 1,
				sufficients: 0,
				data: AccountData {
					free: 48_904_000_000_000,
					reserved: 0,
					free_kton_or_misc_frozen: 0,
					reserved_kton_or_fee_frozen: 0,
				}
			}
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
			migrated_account_info,
			AccountInfo {
				nonce: 0,
				consumers: 0,
				providers: 1,
				sufficients: 0,
				data: AccountData {
					free: 48_904_000_000_000 * GWEI,
					reserved: 0,
					free_kton_or_misc_frozen: 0,
					reserved_kton_or_fee_frozen: 0,
				}
			}
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

#[test]
fn evm_code_migrate() {}

#[test]
fn evm_account_storage_migrate() {}
