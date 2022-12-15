// darwinia
use crate::*;
use sp_core::H256;

impl Processor {
	/// Process evm state storage
	///
	/// The pallet-evm storage item list:
	/// 1. type AccountCodes<T> = StorageMap<_, Blake2_128Concat, H160, Vec<u8>, ValueQuery>;
	/// 2. type AccountStorages<T> = StorageDoubleMap<_, Blake2_128Concat, H160, Blake2_128Concat,
	/// H256, H256, ValueQuery>;
	///
	/// Given that the items in both of these storage are based on H160, so the key-value pairs do
	/// not need to be modified.
	pub fn process_evm(&mut self) {
		let mut account_codes = Map::default();
		let mut account_storages = Map::default();

		let state = &mut self.shell_chain_spec.genesis.raw.top;

		log::info!("set AccountCodes");
		self.solo_state.take::<Vec<u8>, _>(
			b"EVM",
			b"AccountCodes",
			&mut account_codes,
			untouched_key,
		);
		account_codes.into_iter().for_each(|(k, v)| {
			state.insert(k, array_bytes::bytes2hex("0x", &v));
		});

		log::info!("set AccountStorages");
		self.solo_state.take::<H256, _>(
			b"EVM",
			b"AccountStorages",
			&mut account_storages,
			untouched_key,
		);
		account_storages.into_iter().for_each(|(k, v)| {
			state.insert(k, array_bytes::bytes2hex("0x", &v.encode()));
		});
	}
}
