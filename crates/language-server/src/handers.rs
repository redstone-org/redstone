use crate::{event_loop::Event, utils};

/// Top Level Event Handler that dispatches to other handlers.
pub fn handle_event(event: &Event) {
    match event {
        Event::LspRequest(request) => {
            utils::trace_info(&format!("Recieved lsp request {:?}", request));
        }
        Event::LspResponse(response) => {
            utils::trace_info(&format!("Recieved lsp response {:?}", response));
        }
        Event::LspNotification(notification) => {
            utils::trace_info(&format!("Recieved lsp notification {:?}", notification));
        }
    }
}
