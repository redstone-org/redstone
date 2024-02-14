use std::fs;

use flags::GenerateAst;

use syntax::source::{AST_TOKENS_SOURCE, SYNTAX_KIND_SOURCE};
use ungrammar::Grammar;

mod flags;
mod ungrammar_json;

static UNGRAMMAR_FILE: &str = "crates/syntax/java.ungram";
static SYNTAX_KIND_FILE: &str = "crates/syntax/src/gen/syntax_kind.rs";
static AST_TOKENS_FILE: &str = "crates/syntax/src/gen/ast_tokens.rs";
static AST_NODES_FILE: &str = "crates/syntax/src/gen/ast_nodes.rs";

fn main() {
    let xtask_flags = flags::Xtask::from_env_or_exit();

    match xtask_flags.subcommand {
        flags::XtaskCmd::GenerateAst(params) => generate_ast(params),
        flags::XtaskCmd::UngrammarJson(_) => ungrammar_json::run(),
    }
}

fn generate_ast(params: GenerateAst) {
    if params.dry_run {
        println!("Running in dry-run mode");
        return;
    }

    let workspace_dir = sourcegen::utils::workspace_dir();

    // 1. Generate Syntax Kinds
    let syntax_kinds = sourcegen::generate_syntax_kinds(&SYNTAX_KIND_SOURCE);
    let syntax_kinds_file = workspace_dir.join(SYNTAX_KIND_FILE);

    println!("Syncing syntax kinds");
    sourcegen::utils::sync(syntax_kinds_file.as_path(), &syntax_kinds);

    // 2. Generate AST Tokens
    let tokens_source = AST_TOKENS_SOURCE
        .tokens
        .iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>();
    let ast_tokens = sourcegen::generate_ast_tokens(tokens_source);
    let ast_tokens_file = workspace_dir.join(AST_TOKENS_FILE);

    println!("Syncing ast tokens");
    sourcegen::utils::sync(ast_tokens_file.as_path(), &ast_tokens);

    // 3. Generate AST Nodes
    let ungrammar_file = workspace_dir.join(UNGRAMMAR_FILE);
    let grammar: Grammar = match fs::read_to_string(ungrammar_file).unwrap().parse() {
        Ok(g) => g,
        Err(err) => {
            println!("Error parsing grammar - {}", err);
            std::process::exit(1)
        }
    };

    let ast_nodes = sourcegen::generate_ast_nodes(&grammar);
    let ast_nodes_file = workspace_dir.join(AST_NODES_FILE);

    println!("Syncing ast nodes");
    sourcegen::utils::sync(ast_nodes_file.as_path(), &ast_nodes);
}
