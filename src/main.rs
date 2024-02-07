#![warn(rust_2018_idioms, unused_lifetimes)]

use std::io::{stderr, Write};

use anyhow::Result;
use crossbeam_channel::{select, Receiver};
use lsp_server::{Connection, Notification as LspServerNotification};
use lsp_types::{notification::Notification, InitializeParams, ServerCapabilities};

mod flags;

fn main() -> Result<()> {
    let redstone_flags = flags::Redstone::from_env_or_exit();

    match redstone_flags.subcommand {
        flags::RedstoneCmd::LspServer(cmd) => {
            run_lsp_server(cmd)?;
        }
    }

    Ok(())
}

fn run_lsp_server(config: flags::LspServer) -> Result<()> {
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

    trace_info(&format!("Got params - {}", params));

    let _init_parms: InitializeParams = serde_json::from_value(params).unwrap();

    let server_capabilites = ServerCapabilities::default();

    let initialize_result = serde_json::json!({
        "capabilities": server_capabilites,
        "serverInfo": {
            "name": "redstone",
            "version": "0.0.1"
        }
    });

    connection.initialize_finish(initialize_id, initialize_result)?;

    trace_info("Initialized connection");

    main_loop(config, connection)?;

    io_threads.join()?;

    trace_info("Server shutdown");

    Ok(())
}

// TODO: Replace with tracing
fn trace_info(message: &str) {
    eprintln!("{}", message);
    stderr().flush().unwrap();
}

fn main_loop(_config: flags::LspServer, connection: Connection) -> anyhow::Result<()> {
    while let Some(event) = next_event(&connection.receiver) {
        if is_exit_event(&event) {
            trace_info("Recieved exit message, exiting main-loop");
            return Ok(());
        }
        trace_info(&format!("Recieved event {:?}", event));
    }

    Ok(())
}

#[derive(Debug)]
enum Event {
    Lsp(lsp_server::Message),
    _Others,
}

impl Event {
    fn lsp(msg: lsp_server::Message) -> Event {
        Event::Lsp(msg)
    }
}

fn is_exit_event(event: &Event) -> bool {
    match event {
        Event::Lsp(lsp_server::Message::Notification(LspServerNotification { method, .. })) => {
            return method == lsp_types::notification::Exit::METHOD;
        }
        _ => false,
    }
}

fn next_event(lsp_reciever: &Receiver<lsp_server::Message>) -> Option<Event> {
    select! {
        recv(lsp_reciever) -> msg => msg.ok().map(Event::lsp)
    }
}
