use std::io::{stderr, Write};

// TODO: Replace with tracing
pub fn trace_info(message: &str) {
    eprintln!("{}", message);
    stderr().flush().unwrap();
}
