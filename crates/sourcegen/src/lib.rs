pub mod syntax_kind;
pub mod utils;

use quote::format_ident;
use syntax_kind::GenerateSyntaxKinds;
use utils::to_upper_snake_case;

pub type StrArrayRef<'a> = &'a [&'a str];

pub type StrTupleArrayRef<'a> = &'a [(&'a str, &'a str)];

pub struct AstSource<'a> {
    pub seperators: StrTupleArrayRef<'a>,
    pub reserved_keywords: StrArrayRef<'a>,
    pub contextual_keywords: StrArrayRef<'a>,
    pub literals: StrArrayRef<'a>,
    pub nodes: StrArrayRef<'a>,
}

pub fn generate_syntax_kinds(source: &AstSource) -> String {
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

    let recipe = GenerateSyntaxKinds {
        seperators,
        reserved_keywords,
        literals,
    };

    utils::reformat(recipe.generate())
}
