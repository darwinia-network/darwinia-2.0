// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

// std
use std::{env, net::SocketAddr, path::PathBuf};
// crates.io
use codec::Encode;
// cumulus
use cumulus_primitives_core::ParaId;
// darwinia
use crate::{
	chain_spec::*,
	cli::{Cli, RelayChainCli, Subcommand},
	frontier_service,
	service::{self, *},
};
use dc_primitives::Block;
// frontier
use fc_db::frontier_database_dir;
// substrate
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::{
	CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams, NetworkParams,
	Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
	config::{BasePath, PrometheusConfig},
	ChainSpec, DatabaseSource, PartialComponents,
};
use sp_core::{crypto::Ss58AddressFormatRegistry, hexdisplay::HexDisplay};
use sp_runtime::traits::{AccountIdConversion, Block as BlockT};

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Darwinia".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Darwinia\n\nThe command-line arguments provided first will be \
			passed to the parachain node, while the arguments provided after -- will be passed \
			to the relay chain node.\n\n\
			{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/darwinia-network/darwinia/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2018
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		#[cfg(feature = "crab-native")]
		if spec.is_crab() {
			return &crab_runtime::VERSION;
		}

		#[cfg(feature = "darwinia-native")]
		if spec.is_darwinia() {
			return &darwinia_runtime::VERSION;
		}

		#[cfg(feature = "pangolin-native")]
		if spec.is_pangolin() {
			return &pangolin_runtime::VERSION;
		}

		#[cfg(feature = "pangoro-native")]
		if spec.is_pangoro() {
			return &pangoro_runtime::VERSION;
		}

		panic!(
			"No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!"
		);
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Darwinia".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Darwinia\n\nThe command-line arguments provided first will be \
			passed to the parachain node, while the arguments provided after -- will be passed \
			to the relay chain node.\n\n\
			{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/darwinia-network/darwinia/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2018
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}
impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_ws_listen_port() -> u16 {
		9945
	}

	fn rpc_http_listen_port() -> u16 {
		9934
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}
impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self.shared_params().base_path()?.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_ws(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	/// Creates partial components for the runtimes that are supported by the benchmarks.
	macro_rules! construct_benchmark_partials {
		($config:expr, $cli:ident, |$partials:ident| $code:expr) => {{
			#[cfg(feature = "crab-native")]
			if $config.chain_spec.is_crab() {
				let $partials = new_partial::<CrabRuntimeApi, CrabRuntimeExecutor>(
					&$config,
					&$cli.eth_args.build_eth_rpc_config(),
				)?;

				return $code;
			}

			#[cfg(feature = "darwinia-native")]
			if $config.chain_spec.is_darwinia() {
				let $partials = new_partial::<DarwiniaRuntimeApi, DarwiniaRuntimeExecutor>(
					&$config,
					&$cli.eth_args.build_eth_rpc_config(),
				)?;

				return $code;
			}

			#[cfg(feature = "pangolin-native")]
			if $config.chain_spec.is_pangolin() {
				let $partials = new_partial::<PangolinRuntimeApi, PangolinRuntimeExecutor>(
					&$config,
					&$cli.eth_args.build_eth_rpc_config(),
				)?;

				return $code;
			}

			#[cfg(feature = "pangoro-native")]
			if $config.chain_spec.is_pangoro() {
				let $partials = new_partial::<PangoroRuntimeApi, PangoroRuntimeExecutor>(
					&$config,
					&$cli.eth_args.build_eth_rpc_config(),
				)?;

				return $code;
			}

			panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
		}};
	}

	macro_rules! construct_async_run {
		(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
			let runner = $cli.create_runner($cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			#[cfg(feature = "crab-native")]
			if chain_spec.is_crab() {
				return runner.async_run(|$config| {
					let $components = service::new_partial::<
						CrabRuntimeApi,
						CrabRuntimeExecutor,
					>(
						&$config,
						&$cli.eth_args.build_eth_rpc_config()
					)?;
					let task_manager = $components.task_manager;

					{ $( $code )* }.map(|v| (v, task_manager))
				});
			}

			#[cfg(feature = "darwinia-native")]
			if chain_spec.is_darwinia() {
				return runner.async_run(|$config| {
					let $components = service::new_partial::<
						DarwiniaRuntimeApi,
						DarwiniaRuntimeExecutor,
					>(
						&$config,
						&$cli.eth_args.build_eth_rpc_config()
					)?;
					let task_manager = $components.task_manager;

					{ $( $code )* }.map(|v| (v, task_manager))
				});
			}

			#[cfg(feature = "pangolin-native")]
			if chain_spec.is_pangolin() {
				return runner.async_run(|$config| {
					let $components = service::new_partial::<
						PangolinRuntimeApi,
						PangolinRuntimeExecutor,
					>(
						&$config,
						&$cli.eth_args.build_eth_rpc_config()
					)?;
					let task_manager = $components.task_manager;

					{ $( $code )* }.map(|v| (v, task_manager))
				});
			}

			#[cfg(feature = "pangoro-native")]
			if chain_spec.is_pangoro() {
				return runner.async_run(|$config| {
					let $components = service::new_partial::<
						PangoroRuntimeApi,
						PangoroRuntimeExecutor,
					>(
						&$config,
						&$cli.eth_args.build_eth_rpc_config()
					)?;
					let task_manager = $components.task_manager;

					{ $( $code )* }.map(|v| (v, task_manager))
				});
			}

			panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
		}}
	}

	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.backend, None))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// Remove Frontier DB.
				let db_config_dir = frontier_service::db_config_dir(&config);
				let frontier_database_config = match config.database {
					DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
						path: frontier_database_dir(&db_config_dir, "db"),
						cache_size: 0,
					},
					DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
						path: frontier_database_dir(&db_config_dir, "paritydb"),
					},
					_ =>
						return Err(format!("Cannot purge `{:?}` database", config.database).into()),
				};

				cmd.base.run(frontier_database_config)?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);
				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				let state_version = Cli::native_runtime_version(&spec).state_version();
				cmd.run::<Block>(&*spec, state_version)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::FrontierDb(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				#[cfg(feature = "crab-native")]
				if config.chain_spec.is_crab() {
					let PartialComponents { client, other: (frontier_backend, ..), .. } =
						service::new_partial::<CrabRuntimeApi, CrabRuntimeExecutor>(
							&config,
							&cli.eth_args.build_eth_rpc_config(),
						)?;

					return cmd.run::<_, dc_primitives::Block>(client, frontier_backend);
				}

				#[cfg(feature = "darwinia-native")]
				if config.chain_spec.is_darwinia() {
					let PartialComponents { client, other: (frontier_backend, ..), .. } =
						service::new_partial::<DarwiniaRuntimeApi, DarwiniaRuntimeExecutor>(
							&config,
							&cli.eth_args.build_eth_rpc_config(),
						)?;

					return cmd.run::<_, dc_primitives::Block>(client, frontier_backend);
				}

				#[cfg(feature = "pangolin-native")]
				if config.chain_spec.is_pangolin() {
					let PartialComponents { client, other: (frontier_backend, ..), .. } =
						service::new_partial::<PangolinRuntimeApi, PangolinRuntimeExecutor>(
							&config,
							&cli.eth_args.build_eth_rpc_config(),
						)?;

					return cmd.run::<_, dc_primitives::Block>(client, frontier_backend);
				}

				#[cfg(feature = "pangoro-native")]
				if config.chain_spec.is_pangoro() {
					let PartialComponents { client, other: (frontier_backend, ..), .. } =
						service::new_partial::<PangoroRuntimeApi, PangoroRuntimeExecutor>(
							&config,
							&cli.eth_args.build_eth_rpc_config(),
						)?;

					return cmd.run::<_, dc_primitives::Block>(client, frontier_backend);
				}

				panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(&**cmd)?;

			// Switch on the concrete benchmark sub-command-
			match &**cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| {
							#[cfg(feature = "crab-native")]
							if config.chain_spec.is_crab() {
								return cmd.run::<Block, CrabRuntimeExecutor>(config);
							}

							#[cfg(feature = "darwinia-native")]
							if config.chain_spec.is_darwinia() {
								return cmd.run::<Block, DarwiniaRuntimeExecutor>(config);
							}

							#[cfg(feature = "pangolin-native")]
							if config.chain_spec.is_pangolin() {
								return cmd.run::<Block, PangolinRuntimeExecutor>(config);
							}

							#[cfg(feature = "pangoro-native")]
							if config.chain_spec.is_pangoro() {
								return cmd.run::<Block, PangoroRuntimeExecutor>(config);
							}

							panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
						})
					} else {
						Err("Benchmarking wasn't enabled when building the node. You can enable it with `--features runtime-benchmarks`.".into())
					},
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, cli, |partials| cmd.run(partials.client))
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
					"Compile with --features=runtime-benchmarks to enable storage benchmarks.".into(),
				)),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, cli, |partials| {
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();

						cmd.run(config, partials.client.clone(), db, storage)
					})
				}),
				BenchmarkCmd::Machine(cmd) =>
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())),
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use sc_service::TaskManager;

			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			type HostFunctionsOf<E> = ExtendedHostFunctions<
				sp_io::SubstrateHostFunctions,
				<E as NativeExecutionDispatch>::ExtendHostFunctions,
			>;

			// grab the task manager.
			let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager = TaskManager::new(runner.config().tokio_handle.clone(), *registry)
				.map_err(|e| format!("Error: {:?}", e))?;

			#[cfg(feature = "crab-native")]
			if chain_spec.is_crab() {
				return runner.async_run(|_| {
					Ok((cmd.run::<Block, HostFunctionsOf<CrabRuntimeExecutor>>(), task_manager))
				});
			}

			#[cfg(feature = "darwinia-native")]
			if chain_spec.is_darwinia() {
				return runner.async_run(|_| {
					Ok((cmd.run::<Block, HostFunctionsOf<DarwiniaRuntimeExecutor>>(), task_manager))
				});
			}

			#[cfg(feature = "pangolin-native")]
			if chain_spec.is_pangolin() {
				return runner.async_run(|_| {
					Ok((cmd.run::<Block, HostFunctionsOf<PangolinRuntimeExecutor>>(), task_manager))
				});
			}

			#[cfg(feature = "pangoro-native")]
			if chain_spec.is_pangoro() {
				return runner.async_run(|_| {
					Ok((cmd.run::<Block, HostFunctionsOf<PangoroRuntimeExecutor>>(), task_manager))
				});
			}

			panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err(
			"Try-runtime was not enabled when building the node. You can enable it with `--features try-runtime`.".into()
		),
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let chain_spec = &config.chain_spec;
				let hwbench = if !cli.no_hardware_benchmarks {
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})
				} else {
					None
				};

				set_default_ss58_version(chain_spec);

				let para_id = Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);
				let id = ParaId::from(para_id);
				let parachain_account =
					AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);
				let state_version = Cli::native_runtime_version(&config.chain_spec).state_version();
				let block: Block =
					cumulus_client_cli::generate_genesis_block(&*config.chain_spec, state_version)
						.map_err(|e| format!("{:?}", e))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));
				let tokio_handle = config.tokio_handle.clone();
				let eth_rpc_config = cli.eth_args.build_eth_rpc_config();

				log::info!("Parachain id: {:?}", id);
				log::info!("Parachain Account: {}", parachain_account);
				log::info!("Parachain genesis state: {}", genesis_state);
				log::info!(
					"Is collating: {}",
					if config.role.is_authority() { "yes" } else { "no" }
				);

				if !collator_options.relay_chain_rpc_urls.is_empty() && !cli.relay_chain_args.is_empty() {
					log::warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
				}

				if chain_spec.is_dev() {
					#[cfg(feature = "crab-native")]
					if chain_spec.is_crab() {
						return service::start_dev_node::<CrabRuntimeApi, CrabRuntimeExecutor>(
							config,
							&eth_rpc_config,
						)
						.map_err(Into::into);
					}

					#[cfg(feature = "darwinia-native")]
					if chain_spec.is_darwinia() {
						return service::start_dev_node::<DarwiniaRuntimeApi, DarwiniaRuntimeExecutor>(
							config,
							&eth_rpc_config,
						)
						.map_err(Into::into)
					}

					#[cfg(feature = "pangolin-native")]
					if chain_spec.is_pangolin() {
						return service::start_dev_node::<PangolinRuntimeApi, PangolinRuntimeExecutor>(
							config,
							&eth_rpc_config,
						)
						.map_err(Into::into)
					}

					#[cfg(feature = "pangoro-native")]
					if chain_spec.is_pangoro() {
						return service::start_dev_node::<PangoroRuntimeApi, PangoroRuntimeExecutor>(
							config,
							&eth_rpc_config,
						)
						.map_err(Into::into)
					}
				}

				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				#[cfg(feature = "crab-native")]
				if chain_spec.is_crab() {
					return service::start_parachain_node::<CrabRuntimeApi, CrabRuntimeExecutor>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
						&eth_rpc_config,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into);
				}

				#[cfg(feature = "darwinia-native")]
				if chain_spec.is_darwinia() {
					return service::start_parachain_node::<DarwiniaRuntimeApi, DarwiniaRuntimeExecutor>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
						&eth_rpc_config,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into);
				}

				#[cfg(feature = "pangolin-native")]
				if chain_spec.is_pangolin() {
					return service::start_parachain_node::<PangolinRuntimeApi, PangolinRuntimeExecutor>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
						&eth_rpc_config,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into);
				}

				#[cfg(feature = "pangoro-native")]
				if chain_spec.is_pangoro() {
					return service::start_parachain_node::<PangoroRuntimeApi, PangoroRuntimeExecutor>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
						&eth_rpc_config,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into);
				}

				panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!");
			})
		},
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
	let id = if id.is_empty() {
		let n = get_exec_name().unwrap_or_default();
		["darwinia", "crab", "pangolin"]
			.iter()
			.cloned()
			.find(|&chain| n.starts_with(chain))
			.unwrap_or("darwinia")
	} else {
		id
	};

	Ok(match id.to_lowercase().as_str() {
		#[cfg(feature = "crab-native")]
		"crab" => Box::new(crab_chain_spec::config()),
		#[cfg(feature = "crab-native")]
		"crab-genesis" => Box::new(crab_chain_spec::genesis_config()),
		#[cfg(feature = "crab-native")]
		"crab-dev" => Box::new(crab_chain_spec::development_config()),
		#[cfg(feature = "crab-native")]
		"crab-local" => Box::new(crab_chain_spec::local_config()),
		#[cfg(feature = "darwinia-native")]
		"darwinia" => Box::new(darwinia_chain_spec::config()),
		#[cfg(feature = "darwinia-native")]
		"darwinia-genesis" => Box::new(darwinia_chain_spec::genesis_config()),
		#[cfg(feature = "darwinia-native")]
		"darwinia-dev" => Box::new(darwinia_chain_spec::development_config()),
		#[cfg(feature = "darwinia-native")]
		"darwinia-local" => Box::new(darwinia_chain_spec::local_config()),
		#[cfg(feature = "pangolin-native")]
		"pangolin" => Box::new(pangolin_chain_spec::config()),
		#[cfg(feature = "pangolin-native")]
		"pangolin-genesis" => Box::new(pangolin_chain_spec::genesis_config()),
		#[cfg(feature = "pangolin-native")]
		"pangolin-dev" => Box::new(pangolin_chain_spec::development_config()),
		#[cfg(feature = "pangolin-native")]
		"pangolin-local" => Box::new(pangolin_chain_spec::local_config()),
		#[cfg(feature = "pangoro-native")]
		"pangoro" => Box::new(pangoro_chain_spec::config()),
		#[cfg(feature = "pangoro-native")]
		"pangoro-genesis" => Box::new(pangoro_chain_spec::genesis_config()),
		#[cfg(feature = "pangoro-native")]
		"pangoro-dev" => Box::new(pangoro_chain_spec::development_config()),
		#[cfg(feature = "pangoro-native")]
		"pangoro-local" => Box::new(pangoro_chain_spec::local_config()),
		_ => {
			let path = PathBuf::from(id);
			let chain_spec =
				Box::new(DummyChainSpec::from_json_file(path.clone())?) as Box<dyn ChainSpec>;

			if chain_spec.is_crab() {
				return Ok(Box::new(CrabChainSpec::from_json_file(path)?));
			}

			if chain_spec.is_darwinia() {
				return Ok(Box::new(DarwiniaChainSpec::from_json_file(path)?));
			}

			if chain_spec.is_pangolin() {
				return Ok(Box::new(PangolinChainSpec::from_json_file(path)?));
			}

			if chain_spec.is_pangoro() {
				return Ok(Box::new(PangoroChainSpec::from_json_file(path)?));
			}

			panic!("No feature(crab-native, darwinia-native, pangolin-native, pangoro-native) is enabled!")
		},
	})
}

fn get_exec_name() -> Option<String> {
	env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

fn set_default_ss58_version(chain_spec: &dyn IdentifyVariant) {
	let ss58_version = if chain_spec.is_crab() || chain_spec.is_pangolin() {
		Ss58AddressFormatRegistry::SubstrateAccount
	} else {
		Ss58AddressFormatRegistry::DarwiniaAccount
	}
	.into();

	sp_core::crypto::set_default_ss58_version(ss58_version);
}
