pub mod utils;

mod ast_tokens;
mod syntax_kinds;

use ast_tokens::GenerateAstTokens;
use quote::format_ident;
use syntax_kinds::GenerateSyntaxKinds;
use utils::to_upper_snake_case;

pub type StrArrayRef<'a> = &'a [&'a str];

pub type StrTupleArrayRef<'a> = &'a [(&'a str, &'a str)];

pub struct SyntaxKindSource<'a> {
    pub seperators: StrTupleArrayRef<'a>,
    pub reserved_keywords: StrArrayRef<'a>,
    pub contextual_keywords: StrArrayRef<'a>,
    pub literals: StrArrayRef<'a>,
    pub nodes: StrArrayRef<'a>,
    pub tokens: StrArrayRef<'a>,
}

pub fn generate_syntax_kinds(source: &SyntaxKindSource) -> String {
    let seperators = source
        .seperators
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let reserved_keywords = source
        .reserved_keywords
        .iter()
        .map(|name| format_ident!("{}_KW", to_upper_snake_case(name)))
        .collect::<Vec<_>>();

    let literals = source
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = source
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let recipe = GenerateSyntaxKinds {
        seperators,
        reserved_keywords,
        literals,
        tokens,
    };

    utils::reformat(recipe.generate())
}

pub struct AstSource {
    pub tokens: Vec<String>,
}

pub fn generate_ast_tokens(source: &AstSource) -> String {
    let recipe = GenerateAstTokens {
        tokens: source.tokens.clone(),
    };

    utils::reformat(recipe.generate())
}
