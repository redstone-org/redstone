#![warn(rust_2018_idioms, unused_lifetimes)]

use event_loop::EventLoop;
use lsp_server::Connection;
use lsp_types::InitializeParams;

mod capabilities;
mod event_loop;
mod file_service;
mod flags;
mod handers;
mod utils;

fn main() -> anyhow::Result<()> {
    let redstone_flags = flags::Redstone::from_env_or_exit();

    match redstone_flags.subcommand {
        flags::RedstoneCmd::LspServer(cmd) => {
            run_lsp_server(cmd)?;
        }
    }

    Ok(())
}

fn run_lsp_server(config: flags::LspServer) -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();

    let (initialize_id, params) = match connection.initialize_start() {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };

    utils::trace_info(&format!("Got params - {}", params));

    let _init_parms: InitializeParams = serde_json::from_value(params).unwrap();

    let server_capabilites = capabilities::server_capabilities();

    utils::trace_info(&format!(
        "Server capabilities are: {:?}",
        server_capabilites
    ));

    let initialize_result = serde_json::json!({
        "capabilities": server_capabilites,
        "serverInfo": {
            "name": "redstone",
            "version": "0.0.1"
        }
    });

    connection.initialize_finish(initialize_id, initialize_result)?;

    utils::trace_info("Initialized connection");

    let event_loop = EventLoop::new(
        config,
        connection,
        Box::new(|event| handers::handle_event(&event)),
    );

    event_loop.run()?;

    io_threads.join()?;

    utils::trace_info("Server shutdown");

    Ok(())
}
