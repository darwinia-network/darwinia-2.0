// Read the demo items out from the origin crab.json and crab-parachain.json
// Compared with the data processed.json

use crate::State;

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

#[test]
fn balance_adjust_for_only_solo_chain_account() {}

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
