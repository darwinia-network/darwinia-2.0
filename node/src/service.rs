// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
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

//! Service and service factory implementation. Specialized wrapper over substrate service.

// std
use std::{
	collections::BTreeMap,
	sync::{Arc, Mutex},
	time::Duration,
};
// darwinia
use crate::frontier_service;
use darwinia_runtime::AuraId;
use dc_primitives::*;
// substrate
use sc_network_common::service::NetworkBlock;
use sp_core::Pair;
use sp_runtime::app_crypto::AppKey;

type FullBackend = sc_service::TFullBackend<Block>;
type FullClient<RuntimeApi, Executor> =
	sc_service::TFullClient<Block, RuntimeApi, sc_executor::NativeElseWasmExecutor<Executor>>;

/// A set of APIs that darwinia-like runtimes must implement.
pub trait RuntimeApiCollection:
	cumulus_primitives_core::CollectCollationInfo<Block>
	+ sp_api::ApiExt<Block, StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>
	+ sp_api::Metadata<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_consensus_aura::AuraApi<Block, <<AuraId as AppKey>::Pair as Pair>::Public>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
	+ fp_rpc::EthereumRuntimeRPCApi<Block>
	+ fp_rpc::ConvertTransactionRuntimeApi<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}
impl<Api> RuntimeApiCollection for Api where
	Api: cumulus_primitives_core::CollectCollationInfo<Block>
		+ sp_api::ApiExt<Block, StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>
		+ sp_api::Metadata<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_consensus_aura::AuraApi<Block, <<AuraId as AppKey>::Pair as Pair>::Public>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}

/// Native executor instance.
pub struct DarwiniaRuntimeExecutor;
impl sc_executor::NativeExecutionDispatch for DarwiniaRuntimeExecutor {
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		darwinia_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		darwinia_runtime::native_version()
	}
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
pub fn new_partial<RuntimeApi, Executor>(
	config: &sc_service::Configuration,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> Result<
	sc_service::PartialComponents<
		FullClient<RuntimeApi, Executor>,
		FullBackend,
		(),
		sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, Executor>>,
		sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>,
		(
			Arc<fc_db::Backend<Block>>,
			Option<fc_rpc_core::types::FilterPool>,
			fc_rpc_core::types::FeeHistoryCache,
			fc_rpc_core::types::FeeHistoryCacheLimit,
			Option<sc_telemetry::Telemetry>,
			Option<sc_telemetry::TelemetryWorkerHandle>,
		),
	>,
	sc_service::Error,
>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = sc_telemetry::TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;
	let executor = sc_executor::NativeElseWasmExecutor::<Executor>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);
	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);
	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());
	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);
	let import_queue = parachain_build_import_queue(
		client.clone(),
		config,
		telemetry.as_ref().map(|telemetry| telemetry.handle()),
		&task_manager,
	)?;
	// Frontier stuffs.
	let frontier_backend = Arc::new(fc_db::Backend::open(
		Arc::clone(&client),
		&config.database,
		&frontier_service::db_config_dir(config),
	)?);
	let filter_pool = Some(Arc::new(Mutex::new(BTreeMap::new())));
	let fee_history_cache = Arc::new(Mutex::new(BTreeMap::new()));
	let fee_history_cache_limit = eth_rpc_config.fee_history_limit;

	#[cfg(not(feature = "manual-seal"))]
	{
		let import_queue = parachain_build_import_queue(
			client.clone(),
			config,
			telemetry.as_ref().map(|telemetry| telemetry.handle()),
			&task_manager,
		)?;

		Ok(PartialComponents {
			backend,
			client,
			import_queue,
			keystore_container,
			task_manager,
			transaction_pool,
			select_chain: (),
			other: (
				frontier_backend.clone(),
				filter_pool,
				fee_history_cache,
				fee_history_cache_limit,
				telemetry,
				telemetry_worker_handle,
			),
		})
	}

	#[cfg(feature = "manual-seal")]
	{
		use fc_consensus::FrontierBlockImport;
		let frontier_block_import =
			FrontierBlockImport::new(client.clone(), client.clone(), frontier_backend.clone());

		let import_queue = sc_consensus_manual_seal::import_queue(
			Box::new(frontier_block_import.clone()),
			&task_manager.spawn_essential_handle(),
			config.prometheus_registry(),
		);

		Ok(PartialComponents {
			backend,
			client,
			import_queue,
			keystore_container,
			task_manager,
			transaction_pool,
			select_chain: (),
			other: (
				frontier_backend.clone(),
				filter_pool,
				fee_history_cache,
				fee_history_cache_limit,
				telemetry,
				telemetry_worker_handle,
			),
		})
	}
}

async fn build_relay_chain_interface(
	polkadot_config: sc_service::Configuration,
	parachain_config: &sc_service::Configuration,
	telemetry_worker_handle: Option<sc_telemetry::TelemetryWorkerHandle>,
	task_manager: &mut sc_service::TaskManager,
	collator_options: cumulus_client_cli::CollatorOptions,
	hwbench: Option<sc_sysinfo::HwBench>,
) -> cumulus_relay_chain_interface::RelayChainResult<(
	Arc<(dyn 'static + cumulus_relay_chain_interface::RelayChainInterface)>,
	Option<polkadot_service::CollatorPair>,
)> {
	match collator_options.relay_chain_rpc_url {
		Some(relay_chain_url) => {
			let client = cumulus_relay_chain_rpc_interface::create_client_and_start_worker(
				relay_chain_url,
				task_manager,
			)
			.await?;
			Ok((
				Arc::new(cumulus_relay_chain_rpc_interface::RelayChainRpcInterface::new(client))
					as Arc<_>,
				None,
			))
		},
		None => cumulus_relay_chain_inprocess_interface::build_inprocess_relay_chain(
			polkadot_config,
			parachain_config,
			telemetry_worker_handle,
			task_manager,
			hwbench,
		),
	}
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[allow(clippy::too_many_arguments)]
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor, RB, BIC>(
	parachain_config: sc_service::Configuration,
	polkadot_config: sc_service::Configuration,
	collator_options: cumulus_client_cli::CollatorOptions,
	id: cumulus_primitives_core::ParaId,
	_rpc_ext_builder: RB,
	build_consensus: BIC,
	hwbench: Option<sc_sysinfo::HwBench>,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> sc_service::error::Result<(sc_service::TaskManager, Arc<FullClient<RuntimeApi, Executor>>)>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	sc_client_api::StateBackendFor<FullBackend, Block>: sp_api::StateBackend<Hashing>,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
	RB: 'static
		+ Send
		+ Fn(
			Arc<sc_service::TFullClient<Block, RuntimeApi, Executor>>,
		) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>,
	BIC: FnOnce(
		Arc<FullClient<RuntimeApi, Executor>>,
		Option<&substrate_prometheus_endpoint::Registry>,
		Option<sc_telemetry::TelemetryHandle>,
		&sc_service::TaskManager,
		Arc<dyn cumulus_relay_chain_interface::RelayChainInterface>,
		Arc<sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>>,
		Arc<sc_network::NetworkService<Block, Hash>>,
		sp_keystore::SyncCryptoStorePtr,
		bool,
	) -> Result<
		Box<dyn cumulus_client_consensus_common::ParachainConsensus<Block>>,
		sc_service::Error,
	>,
{
	let parachain_config = cumulus_client_service::prepare_node_config(parachain_config);
	let sc_service::PartialComponents {
		backend,
		client,
		import_queue,
		keystore_container,
		mut task_manager,
		transaction_pool,
		select_chain: _,
		other:
			(
				frontier_backend,
				filter_pool,
				fee_history_cache,
				fee_history_cache_limit,
				mut telemetry,
				telemetry_worker_handle,
			),
	} = new_partial::<RuntimeApi, Executor>(&parachain_config, eth_rpc_config)?;

	let (relay_chain_interface, collator_key) = build_relay_chain_interface(
		polkadot_config,
		&parachain_config,
		telemetry_worker_handle,
		&mut task_manager,
		collator_options.clone(),
		hwbench.clone(),
	)
	.await
	.map_err(|e| match e {
		cumulus_relay_chain_interface::RelayChainError::ServiceError(
			polkadot_service::Error::Sub(x),
		) => x,
		s => s.to_string().into(),
	})?;

	let block_announce_validator =
		cumulus_client_network::BlockAnnounceValidator::new(relay_chain_interface.clone(), id);

	let force_authoring = parachain_config.force_authoring;
	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let import_queue = cumulus_client_service::SharedImportQueue::new(import_queue);

	let (network, system_rpc_tx, tx_handler_controller, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &parachain_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: import_queue.clone(),
			block_announce_validator_builder: Some(Box::new(|_| {
				Box::new(block_announce_validator)
			})),
			warp_sync: None,
		})?;
	let overrides = frontier_service::overrides_handle(client.clone());
	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		eth_rpc_config.eth_log_block_cache,
		eth_rpc_config.eth_statuses_cache,
		prometheus_registry.clone(),
	));
	#[cfg(feature = "manual-seal")]
	let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);
	let rpc_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let network = network.clone();
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let overrides = overrides.clone();
		let fee_history_cache = fee_history_cache.clone();
		let max_past_logs = eth_rpc_config.max_past_logs;
		let collator = parachain_config.role.is_authority();

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				graph: pool.pool().clone(),
				deny_unsafe,
				is_authority: collator,
				network: network.clone(),
				filter_pool: filter_pool.clone(),
				backend: frontier_backend.clone(),
				max_past_logs,
				fee_history_cache: fee_history_cache.clone(),
				fee_history_cache_limit,
				overrides: overrides.clone(),
				block_data_cache: block_data_cache.clone(),
				#[cfg(feature = "manual-seal")]
				command_sink: Some(command_sink.clone()),
			};

			crate::rpc::create_full(deps, subscription_task_executor).map_err(Into::into)
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: keystore_container.sync_keystore(),
		backend: backend.clone(),
		network: network.clone(),
		system_rpc_tx,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
	})?;

	frontier_service::spawn_frontier_tasks(
		&task_manager,
		client.clone(),
		backend,
		frontier_backend,
		filter_pool,
		overrides,
		fee_history_cache,
		fee_history_cache_limit,
	);

	if let Some(hwbench) = hwbench {
		sc_sysinfo::print_hwbench(&hwbench);

		if let Some(ref mut telemetry) = telemetry {
			let telemetry_handle = telemetry.handle();
			task_manager.spawn_handle().spawn(
				"telemetry_hwbench",
				None,
				sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
			);
		}
	}

	let announce_block = {
		let network = network.clone();
		Arc::new(move |hash, data| network.announce_block(hash, data))
	};

	let relay_chain_slot_duration = Duration::from_secs(6);

	if validator {
		let parachain_consensus = build_consensus(
			client.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
			&task_manager,
			relay_chain_interface.clone(),
			transaction_pool,
			network,
			keystore_container.sync_keystore(),
			force_authoring,
		)?;

		let spawner = task_manager.spawn_handle();

		let params = cumulus_client_service::StartCollatorParams {
			para_id: id,
			block_status: client.clone(),
			announce_block,
			client: client.clone(),
			task_manager: &mut task_manager,
			relay_chain_interface,
			spawner,
			parachain_consensus,
			import_queue,
			collator_key: collator_key.expect("Command line arguments do not allow this. qed"),
			relay_chain_slot_duration,
		};

		cumulus_client_service::start_collator(params).await?;
	} else {
		let params = cumulus_client_service::StartFullNodeParams {
			client: client.clone(),
			announce_block,
			task_manager: &mut task_manager,
			para_id: id,
			relay_chain_interface,
			relay_chain_slot_duration,
			import_queue,
			collator_options,
		};

		cumulus_client_service::start_full_node(params)?;
	}

	start_network.start_network();

	Ok((task_manager, client))
}

/// Build the import queue for the parachain runtime.
pub fn parachain_build_import_queue<RuntimeApi, Executor>(
	client: Arc<FullClient<RuntimeApi, Executor>>,
	config: &sc_service::Configuration,
	telemetry: Option<sc_telemetry::TelemetryHandle>,
	task_manager: &sc_service::TaskManager,
) -> Result<
	sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, Executor>>,
	sc_service::Error,
>
where
	RuntimeApi: 'static
		+ Send
		+ Sync
		+ sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: 'static + sc_executor::NativeExecutionDispatch,
{
	let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

	cumulus_client_consensus_aura::import_queue::<
		sp_consensus_aura::sr25519::AuthorityPair,
		_,
		_,
		_,
		_,
		_,
	>(cumulus_client_consensus_aura::ImportQueueParams {
		block_import: client.clone(),
		client,
		create_inherent_data_providers: move |_, _| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot =
				sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					*timestamp,
					slot_duration,
				);

			Ok((slot, timestamp))
		},
		registry: config.prometheus_registry(),
		spawner: &task_manager.spawn_essential_handle(),
		telemetry,
	})
	.map_err(Into::into)
}

/// Start a parachain node.
pub async fn start_parachain_node(
	parachain_config: sc_service::Configuration,
	polkadot_config: sc_service::Configuration,
	collator_options: cumulus_client_cli::CollatorOptions,
	id: cumulus_primitives_core::ParaId,
	hwbench: Option<sc_sysinfo::HwBench>,
	eth_rpc_config: &crate::cli::EthRpcConfig,
) -> sc_service::error::Result<(
	sc_service::TaskManager,
	Arc<
		sc_service::TFullClient<
			Block,
			darwinia_runtime::RuntimeApi,
			sc_executor::NativeElseWasmExecutor<DarwiniaRuntimeExecutor>,
		>,
	>,
)> {
	start_node_impl::<darwinia_runtime::RuntimeApi, DarwiniaRuntimeExecutor, _, _>(
		parachain_config,
		polkadot_config,
		collator_options,
		id,
		|_| Ok(jsonrpsee::RpcModule::new(())),
		|client,
		 prometheus_registry,
		 telemetry,
		 task_manager,
		 relay_chain_interface,
		 transaction_pool,
		 sync_oracle,
		 keystore,
		 force_authoring| {
			let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

			let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool,
				prometheus_registry,
				telemetry.clone(),
			);

			Ok(cumulus_client_consensus_aura::AuraConsensus::build::<
				sp_consensus_aura::sr25519::AuthorityPair,
				_,
				_,
				_,
				_,
				_,
				_,
			>(cumulus_client_consensus_aura::BuildAuraConsensusParams {
				proposer_factory,
				create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
					let relay_chain_interface = relay_chain_interface.clone();
					async move {
						let parachain_inherent =
							cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
								relay_parent,
								&relay_chain_interface,
								&validation_data,
								id,
							).await;
						let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

						let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

						let parachain_inherent = parachain_inherent.ok_or_else(|| {
							Box::<dyn std::error::Error + Send + Sync>::from(
								"Failed to create parachain inherent",
							)
						})?;
						Ok((slot, timestamp, parachain_inherent))
					}
				},
				block_import: client.clone(),
				para_client: client,
				backoff_authoring_blocks: Option::<()>::None,
				sync_oracle,
				keystore,
				force_authoring,
				slot_duration,
				// We got around 500ms for proposing
				block_proposal_slot_portion: cumulus_client_consensus_aura::SlotProportion::new(
					1f32 / 24f32,
				),
				// And a maximum of 750ms if slots are skipped
				max_block_proposal_slot_portion: Some(
					cumulus_client_consensus_aura::SlotProportion::new(1f32 / 16f32),
				),
				telemetry,
			}))
		},
		hwbench,
		eth_rpc_config,
	)
	.await
}
