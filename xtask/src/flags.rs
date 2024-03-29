xflags::xflags! {
    src "./src/flags.rs"

    cmd xtask {
        cmd generate-ast {
            optional --dry-run
        }
        cmd ungrammar-json {
        }
    }
}
// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    GenerateAst(GenerateAst),
    UngrammarJson(UngrammarJson),
}

#[derive(Debug)]
pub struct GenerateAst {
    pub dry_run: bool,
}

#[derive(Debug)]
pub struct UngrammarJson;

impl Xtask {
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
