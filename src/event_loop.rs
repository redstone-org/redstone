use crossbeam_channel::select;
use lsp_server::Connection;
use lsp_types::{notification::Notification, request};

use crate::{flags, utils};

pub type Handler = Box<dyn Fn(Event)>;

pub struct EventLoop {
    _config: flags::LspServer,
    connection: Connection,
    handler: Handler,
}

impl EventLoop {
    pub fn new(_config: flags::LspServer, connection: Connection, handler: Handler) -> EventLoop {
        EventLoop {
            _config,
            connection,
            handler,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        while let Some(event) = self.next_event() {
            if event.is_exit_event() {
                utils::trace_info("Recieved exit message, exiting main-loop");
                return Ok(());
            }
            self.handler.as_ref()(event)
        }

        Ok(())
    }

    fn next_event(&self) -> Option<Event> {
        select! {
            recv(self.connection.receiver) -> msg => msg.ok().map(Event::from_lsp_msg)
        }
    }
}

#[derive(Debug)]
pub enum Event {
    LspRequest(lsp_server::Request),
    LspResponse(lsp_server::Response),
    LspNotification(lsp_server::Notification),
}

impl Event {
    pub fn from_lsp_msg(msg: lsp_server::Message) -> Event {
        match msg {
            lsp_server::Message::Request(inner) => Event::LspRequest(inner),
            lsp_server::Message::Response(inner) => Event::LspResponse(inner),
            lsp_server::Message::Notification(inner) => Event::LspNotification(inner),
        }
    }

    pub fn is_exit_event(&self) -> bool {
        match self {
            Event::LspNotification(notification) => {
                return notification.method == lsp_types::notification::Exit::METHOD;
            }
            _ => false,
        }
    }
}
