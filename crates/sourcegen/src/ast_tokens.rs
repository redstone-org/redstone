use quote::{format_ident, quote};

use crate::utils;

pub struct GenerateAstTokens {
    pub tokens: Vec<String>,
}

impl GenerateAstTokens {
    pub fn generate(&self) -> String {
        let tokens = self.tokens.iter().map(|token| {
            let name = format_ident!("{}", token);
            let kind = format_ident!("{}", utils::to_upper_snake_case(token));
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #name {
                    pub(crate) syntax: SyntaxToken,
                }
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(&self.syntax, f)
                    }
                }
                impl AstToken for #name {
                    fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }
                    fn cast(syntax: SyntaxToken) -> Option<Self> {
                        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                    }
                    fn syntax(&self) -> &SyntaxToken { &self.syntax }
                }
            }
        });

        let generated = quote! {
            use crate::{SyntaxKind::{self, *}, SyntaxToken, ast::AstToken};

            #(#tokens)*
        };

        generated.to_string()
    }
}
