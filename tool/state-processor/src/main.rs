// std
use std::{env, fs::File, io::Read};
// crates.io
use anyhow::Result;
use fxhash::FxHashMap;
// hack-ink
use subspector::ChainSpec;

fn main() -> Result<()> {
	env::set_var("RUST_LOG", "state_processor");
	pretty_env_logger::init();

	let state = State::from_file("test-data/darwinia-node-export.json")?
		.prune(b"Babe", None)?
		.prune(b"Timestamp", None)?
		.prune(b"TransactionPayment", None)?
		.prune(b"Authorship", None)?
		.prune(b"ElectionProviderMultiphase", None)?
		// TODO
		.prune(b"Offences", None)?
		.prune(b"Historical", None)?
		// TODO
		.prune(b"Session", None)?
		.prune(b"Grandpa", None)?
		.prune(b"ImOnline", None)?
		.prune(b"AuthorityDiscovery", None)?
		.prune(b"DarwiniaHeaderMmr", None)?
		.prune(b"Democracy", None)?;

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

	fn prune(mut self, pallet: &[u8], items: Option<&[&[u8]]>) -> Result<Self> {
		// Prune specific storages.
		if let Some(items) = items {
			for item in items {
				let k = substorager::storage_key(pallet, item);
				let k = array_bytes::bytes2hex("0x", &k.0);

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
			let prefix = subhasher::twox128(pallet);
			let prefix = array_bytes::bytes2hex("0x", &prefix);
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

		Ok(self)
	}
}
