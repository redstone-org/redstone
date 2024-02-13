use std::fs;

use flags::GenerateAst;

use sourcegen::AstSource;
use syntax::source::SYNTAX_KIND_SOURCE;
use ungrammar::Grammar;

mod flags;
mod ungrammar_json;

static UNGRAMMAR_FILE: &str = "crates/syntax/java.ungram";
static SYNTAX_KIND_FILE: &str = "crates/syntax/src/gen/syntax_kind.rs";
static AST_TOKENS_FILE: &str = "crates/syntax/src/gen/ast_tokens.rs";

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

    let ungrammar_file = workspace_dir.join(UNGRAMMAR_FILE);
    let grammar: Grammar = fs::read_to_string(ungrammar_file).unwrap().parse().unwrap();

    // 1. Generate Syntax Kinds
    let syntax_kinds = sourcegen::generate_syntax_kinds(&SYNTAX_KIND_SOURCE);
    let syntax_kinds_file = workspace_dir.join(SYNTAX_KIND_FILE);

    println!("Syncing syntax kinds");
    sourcegen::utils::sync(syntax_kinds_file.as_path(), &syntax_kinds);

    // 2. Generate AST Tokens
    let ast_source = lower(&grammar);
    let ast_tokens = sourcegen::generate_ast_tokens(&ast_source);
    let ast_tokens_file = workspace_dir.join(AST_TOKENS_FILE);

    println!("Syncing ast tokens");
    sourcegen::utils::sync(ast_tokens_file.as_path(), &ast_tokens);

    // 3. Generate AST Nodes
    println!("{:#?}", grammar);
}

fn lower(_grammar: &Grammar) -> AstSource {
    // We manually set it for now, only need terminals that hold an inner value.
    // TODO: Is there a cleaner way to use SyntaxKind and AST Token generation based on ungrammar
    let tokens = vec![
        "Ident",
        "Whitespace",
        "Comment",
        "Integer",
        "FloatingPoint",
        "Boolean",
        "Character",
        "String",
        "TextBlock",
    ];
    let tokens = tokens.iter().map(|it| it.to_string()).collect::<Vec<_>>();
    AstSource { tokens }
}
