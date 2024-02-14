pub mod utils;

mod recipe;

use quote::format_ident;
use recipe::ast_nodes::GenerateAstNodes;
use recipe::ast_tokens::GenerateAstTokens;
use recipe::syntax_kind::GenerateSyntaxKinds;
use ungrammar::Grammar;
use utils::to_upper_snake_case;

pub type StrArrayRef<'a> = &'a [&'a str];

pub type StrTupleArrayRef<'a> = &'a [(&'a str, &'a str)];

pub struct SyntaxKindSource<'a> {
    pub punctuations: StrTupleArrayRef<'a>,
    pub reserved_keywords: StrArrayRef<'a>,
    pub contextual_keywords: StrArrayRef<'a>,
    pub literals: StrArrayRef<'a>,
    pub nodes: StrArrayRef<'a>,
    pub tokens: StrArrayRef<'a>,
}

pub struct AstTokenSource<'a> {
    pub tokens: StrArrayRef<'a>,
}

pub fn generate_syntax_kinds(source: &SyntaxKindSource) -> String {
    let recipe = GenerateSyntaxKinds { source };

    utils::reformat(recipe.generate())
}

pub fn generate_ast_tokens(tokens: Vec<String>) -> String {
    let recipe = GenerateAstTokens { tokens };

    utils::reformat(recipe.generate())
}

pub fn generate_ast_nodes(grammar: &Grammar) -> String {
    let recipe = GenerateAstNodes { grammar };

    utils::reformat(recipe.generate())
}
