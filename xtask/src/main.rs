use std::fs;

use flags::GenerateAst;

mod flags;

use syntax::source::AST_SOURCE;
use ungrammar::Grammar;

fn main() {
    let xtask_flags = flags::Xtask::from_env_or_exit();

    match xtask_flags.subcommand {
        flags::XtaskCmd::GenerateAst(params) => generate_ast(params),
    }
}

fn generate_ast(params: GenerateAst) {
    if params.dry_run {
        println!("Running in dry-run mode");
        return;
    }

    let workspace_dir = sourcegen::utils::workspace_dir();

    let grammar: Grammar = fs::read_to_string(workspace_dir.join("crates/syntax/java.ungram"))
        .unwrap()
        .parse()
        .unwrap();

    // 1. Generate Syntax Kinds
    let syntax_kinds = sourcegen::generate_syntax_kinds(&AST_SOURCE);
    let syntax_kinds_file = workspace_dir.join("crates/syntax/src/gen/syntax_kind.rs");

    println!("Syncing syntax kinds");
    sourcegen::utils::sync(syntax_kinds_file.as_path(), &syntax_kinds);

    // 2. Generate AST Tokens

    // 3. Generate AST Nodes

    println!("{:?}", grammar);
}
