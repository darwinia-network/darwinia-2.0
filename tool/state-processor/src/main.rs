mod type_registry;
use type_registry::*;

// std
use std::{env, fs::File, io::Read};
// crates.io
use anyhow::Result;
use fxhash::FxHashMap;
use parity_scale_codec::Decode;
// hack-ink
use subspector::ChainSpec;

fn main() -> Result<()> {
	env::set_var("RUST_LOG", "state_processor");
	pretty_env_logger::init();

	let mut accounts_infos = <Vec<AccountInfo>>::new();
	let _state = State::from_file("test-data/darwinia-node-export.json")?
		.take(b"System", b"Account", &mut accounts_infos)
		.prune(b"Babe", None)
		.prune(b"Timestamp", None)
		.prune(b"TransactionPayment", None)
		.prune(b"Authorship", None)
		.prune(b"ElectionProviderMultiphase", None)
		// TODO
		.prune(b"Offences", None)
		.prune(b"Historical", None)
		// TODO
		.prune(b"Session", None)
		.prune(b"Grandpa", None)
		.prune(b"ImOnline", None)
		.prune(b"AuthorityDiscovery", None)
		.prune(b"DarwiniaHeaderMmr", None)
		.prune(b"Democracy", None);

	dbg!(accounts_infos);

	Ok(())
}

struct State(FxHashMap<String, String>);
impl State {
	fn from_file(path: &str) -> Result<Self> {
		let mut f = File::open(path)?;
		let mut v = Vec::new();

		f.read_to_end(&mut v)?;

		Ok(Self(serde_json::from_slice::<ChainSpec>(&v)?.genesis.raw.top))
	}

	fn prune(mut self, pallet: &[u8], items: Option<&[&[u8]]>) -> Self {
		// Prune specific storages.
		if let Some(items) = items {
			for item in items {
				let k = item_key(pallet, item);

				self.0.remove(&k).or_else(|| {
					log::warn!(
						"`{}::{}: {k}` not found",
						String::from_utf8_lossy(pallet),
						String::from_utf8_lossy(item)
					);

					None
				});
			}
		}
		// Prune entire pallet.
		else {
			let prefix = pallet_key(pallet);
			let mut pruned = false;

			self.0.retain(|full_key, _| {
				if full_key.starts_with(&prefix) {
					pruned = true;

					false
				} else {
					true
				}
			});

			if !pruned {
				log::warn!("`{}: {prefix}` not found", String::from_utf8_lossy(pallet));
			}
		}

		self
	}

	fn take<T>(mut self, pallet: &[u8], item: &[u8], buffer: &mut Vec<T>) -> Self
	where
		T: Decode,
	{
		let item_key = item_key(pallet, item);

		self.0.retain(|full_key, v| {
			if full_key.starts_with(&item_key) {
				match decode(v) {
					Ok(v) => buffer.push(v),
					Err(e) => log::warn!("failed to decode `{full_key}:{v}`, due to `{e}`"),
				}

				false
			} else {
				true
			}
		});

		self
	}
}

fn pallet_key(pallet: &[u8]) -> String {
	let prefix = subhasher::twox128(pallet);

	array_bytes::bytes2hex("0x", &prefix)
}

fn item_key(pallet: &[u8], item: &[u8]) -> String {
	let k = substorager::storage_key(pallet, item);

	array_bytes::bytes2hex("0x", &k.0)
}

fn decode<T>(hex: &str) -> Result<T>
where
	T: Decode,
{
	let v = array_bytes::hex2bytes(hex).map_err(|e| anyhow::anyhow!("{e:?}"))?;

	Ok(T::decode(&mut &*v)?)
}
