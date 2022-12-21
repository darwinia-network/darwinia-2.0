// Read the demo items out from the origin crab.json and crab-parachain.json
// Compared with the data processed.json

use core::panic;

use crate::{AccountData, AccountInfo, State};
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

	fn run_test<T>(&self, test: T)
	where
		T: FnOnce() -> () + panic::UnwindSafe,
	{
		Self::new();
		let result = std::panic::catch_unwind(|| test());
		assert!(result.is_ok())
	}
}

#[test]
fn balance_ring_adjust_for_only_solo_chain_account() {
	let tester = Tester::new();
	// https://crab.subscan.io/account/5F2CnHR4JDJW4RXqXhH7dpk4tQuu2qCf7UPzZLX4exDg9VxE(0x82cc5514cdaa945629347924da4b804735c1530be80a5e001ce0b413cc46aa47)
	let test_addr: [u8; 32] = hex_n_into_unchecked::<_, _, 32>(
		"0x82cc5514cdaa945629347924da4b804735c1530be80a5e001ce0b413cc46aa47",
	);
	let mut account_info = AccountInfo::default();
	tester.solo_state.get_value::<AccountInfo>(
		b"System",
		b"Account",
		&bytes2hex("", subhasher::blake2_128_concat(&test_addr)),
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
}

#[test]
fn balance_adjust_for_both_solo_and_para_chain_account() {}

#[test]
fn ring_total_issuance() {}

#[test]
fn kton_total_issuance() {}

#[test]
fn special_accounts() {}

#[test]
fn nonce_adjust_for_account_id_32() {}

#[test]
fn nonce_adjust_for_evm_account() {}
