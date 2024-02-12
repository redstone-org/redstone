xflags::xflags! {
    src "./src/flags.rs"

    cmd redstone {
        default cmd lsp-server {
            optional --version
        }
    }
}
// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Redstone {
    pub subcommand: RedstoneCmd,
}

#[derive(Debug)]
pub enum RedstoneCmd {
    LspServer(LspServer),
}

#[derive(Debug)]
pub struct LspServer {
    pub version: bool,
}

impl Redstone {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end