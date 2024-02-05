#![warn(rust_2018_idioms, unused_lifetimes)]

use anyhow::Result;

mod flags;

fn main() -> Result<()> {
    let redstone_flags = flags::Redstone::from_env_or_exit();

    match redstone_flags.subcommand {
        flags::RedstoneCmd::LspServer(_) => {
            println!("Starting Redstone Language Server, version 0.0.1");
        }
    }

    Ok(())
}
