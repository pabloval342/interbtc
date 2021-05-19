// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    chain_spec,
    cli::{Cli, Subcommand},
};
use btc_parachain_runtime::Block;
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::{Configuration, PartialComponents, TaskManager};

#[cfg(feature = "aura-grandpa")]
use sc_cli::Role;

#[cfg(feature = "cumulus-polkadot")]
use {
    crate::cli::RelayChainCli,
    codec::Encode,
    cumulus_primitives::ParaId,
    cumulus_service::genesis::generate_genesis_block,
    log::info,
    polkadot_parachain::primitives::AccountIdConversion,
    sc_cli::{CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams, NetworkParams, SharedParams},
    sc_service::config::{BasePath, PrometheusConfig},
    sp_core::hexdisplay::HexDisplay,
    sp_runtime::traits::Block as BlockT,
    std::{io::Write, net::SocketAddr},
};

fn load_spec(
    id: &str,
    #[cfg(feature = "cumulus-polkadot")] para_id: ParaId,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    match id {
        #[cfg(feature = "cumulus-polkadot")]
        "rococo" => Ok(Box::new(chain_spec::rococo_testnet_config(para_id))),
        #[cfg(feature = "aura-grandpa")]
        "beta" => Ok(Box::new(chain_spec::beta_testnet_config())),
        "dev" => Ok(Box::new(chain_spec::development_config(
            #[cfg(feature = "cumulus-polkadot")]
            para_id,
        ))),
        "" => Ok(Box::new(chain_spec::local_config(
            #[cfg(feature = "cumulus-polkadot")]
            para_id,
        ))),
        path => Ok(Box::new(chain_spec::ChainSpec::from_json_file(path.into())?)),
    }
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "BTC Parachain".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/interlay/btc-parachain/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(
            id,
            #[cfg(feature = "cumulus-polkadot")]
            self.run.parachain_id.unwrap_or(100).into(),
        )
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &btc_parachain_runtime::VERSION
    }
}

#[cfg(feature = "cumulus-polkadot")]
impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "BTC Parachain".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Cumulus test parachain collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		rococo-collator [parachain-args] -- [relaychain-args]"
            .into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/cumulus/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name().to_string()].iter()).load_spec(id)
    }

    fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        polkadot_cli::Cli::native_runtime_version(chain_spec)
    }
}

#[cfg(feature = "cumulus-polkadot")]
fn extract_genesis_wasm(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Result<Vec<u8>> {
    let mut storage = chain_spec.build_storage()?;

    storage
        .top
        .remove(sp_core::storage::well_known_keys::CODE)
        .ok_or_else(|| "Could not find wasm file in genesis state!".into())
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = btc_parachain_service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client, task_manager, ..
                } = btc_parachain_service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client, task_manager, ..
                } = btc_parachain_service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = btc_parachain_service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = btc_parachain_service::new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;

                runner.sync_run(|config| cmd.run::<Block, btc_parachain_service::Executor>(config))
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                    .into())
            }
        }
        #[cfg(feature = "cumulus-polkadot")]
        Some(Subcommand::ExportGenesisState(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let block: Block = generate_genesis_block(&load_spec(
                &params.chain.clone().unwrap_or_default(),
                params.parachain_id.into(),
            )?)?;
            let raw_header = block.header().encode();
            let output_buf = if params.raw {
                raw_header
            } else {
                format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        #[cfg(feature = "cumulus-polkadot")]
        Some(Subcommand::ExportGenesisWasm(params)) => {
            let mut builder = sc_cli::LoggerBuilder::new("");
            builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
            let _ = builder.init();

            let raw_wasm_blob = extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
            let output_buf = if params.raw {
                raw_wasm_blob
            } else {
                format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
            };

            if let Some(output) = &params.output {
                std::fs::write(output, output_buf)?;
            } else {
                std::io::stdout().write_all(&output_buf)?;
            }

            Ok(())
        }
        None => {
            let runner = cli.create_runner(&*cli.run)?;

            runner
                .run_node_until_exit(|config| async move { start_node(cli, config).await })
                .map_err(Into::into)
        }
    }
}

#[cfg(feature = "aura-grandpa")]
async fn start_node(_: Cli, config: Configuration) -> sc_service::error::Result<TaskManager> {
    match config.role {
        Role::Light => btc_parachain_service::new_light(config),
        _ => btc_parachain_service::new_full(config),
    }
}

#[cfg(feature = "cumulus-polkadot")]
async fn start_node(cli: Cli, config: Configuration) -> sc_service::error::Result<TaskManager> {
    let key = sp_core::Pair::generate().0;

    let para_id = chain_spec::Extensions::try_get(&*config.chain_spec).map(|e| e.para_id);

    let polkadot_cli = RelayChainCli::new(
        &config,
        [RelayChainCli::executable_name().to_string()]
            .iter()
            .chain(cli.relaychain_args.iter()),
    );

    let id = ParaId::from(cli.run.parachain_id.or(para_id).unwrap_or(100));

    let parachain_account = AccountIdConversion::<polkadot_primitives::v0::AccountId>::into_account(&id);

    let block: Block = generate_genesis_block(&config.chain_spec).map_err(|e| format!("{:?}", e))?;
    let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

    let task_executor = config.task_executor.clone();
    let polkadot_config = SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, task_executor)
        .map_err(|err| format!("Relay chain argument error: {}", err))?;

    info!("Parachain id: {:?}", id);
    info!("Parachain Account: {}", parachain_account);
    info!("Parachain genesis state: {}", genesis_state);
    info!(
        "Is collating: {}",
        if config.role.is_authority() { "yes" } else { "no" }
    );

    btc_parachain_service::start_node(config, key, polkadot_config, id)
        .await
        .map(|srv| srv.0)
}

#[cfg(feature = "cumulus-polkadot")]
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

#[cfg(feature = "cumulus-polkadot")]
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
        Ok(self
            .shared_params()
            .base_path()
            .or_else(|| self.base_path.clone().map(Into::into)))
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

    fn prometheus_config(&self, default_listen_port: u16) -> Result<Option<PrometheusConfig>> {
        self.base.base.prometheus_config(default_listen_port)
    }

    fn init<C: SubstrateCli>(&self) -> Result<()> {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool()
    }

    fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
        self.base.base.state_cache_child_ratio()
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

    fn telemetry_external_transport(&self) -> Result<Option<sc_service::config::ExtTransport>> {
        self.base.base.telemetry_external_transport()
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
}
