// darwinia
use crate::*;

/// Copied from frontier. https://github.com/paritytech/frontier/blob/polkadot-v0.9.30/primitives/storage/src/lib.rs#L23
pub const PALLET_ETHEREUM_SCHEMA: &[u8] = b":ethereum_schema";

impl Processor {
	pub fn process_ethereum(&mut self) {
		log::info!("set PALLET_ETHEREUM_SCHEMA");
		let state = &mut self.shell_chain_spec.genesis.raw.top;
		state.insert(array_bytes::bytes2hex("0x", PALLET_ETHEREUM_SCHEMA), "0x3".into());
	}
}

#[test]
fn test_schema_key() {
	assert_eq!(
		array_bytes::bytes2hex("0x", PALLET_ETHEREUM_SCHEMA),
		"0x3a657468657265756d5f736368656d61"
	);
}
