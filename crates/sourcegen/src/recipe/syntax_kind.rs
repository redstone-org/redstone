use proc_macro2::Ident;
use quote::{format_ident, quote, IdentFragment};

use crate::{utils, SyntaxKindSource};

pub struct GenerateSyntaxKinds<'a> {
    pub source: &'a SyntaxKindSource<'a>,
}

impl<'a> GenerateSyntaxKinds<'a> {
    pub fn generate(&self) -> String {
        format!("{} {}", self.generate_enum(), self.generate_macro())
    }

    pub fn generate_enum(&self) -> String {
        let (char_token_values, char_tokens): (Vec<_>, Vec<_>) = self
            .source
            .punctuations
            .iter()
            .filter(|(token, _name)| token.len() == 1)
            .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
            .unzip();

        let punctuations = as_idents(self.source.punctuations, |(_token, name)| name);

        let reserved_keywords = as_idents(self.source.reserved_keywords, |i| {
            format!("{}_KW", utils::to_upper_snake_case(i))
        });

        let literals = as_idents(self.source.literals, |i| i);

        let tokens = as_idents(self.source.tokens, |i| i);

        let nodes = as_idents(self.source.nodes, |i| i);

        let generated = quote! {
            #![allow(bad_style, missing_docs, unreachable_pub)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            #[repr(u16)]
            pub enum SyntaxKind {
                #[doc(hidden)]
                TOMBSTONE,
                #[doc(hidden)]
                EOF,
                #(#punctuations,)*
                #(#reserved_keywords,)*
                #(#literals,)*
                #(#tokens,)*
                #(#nodes,)*

                #[doc(hidden)]
                __LAST,
            }
            use self::SyntaxKind::*;

            impl SyntaxKind {
                pub fn is_keyword(self) -> bool {
                    matches!(self, #(#reserved_keywords)|*)
                }

                pub fn is_punctuation(self) -> bool {
                    matches!(self, #(#punctuations)|*)

                }

                pub fn is_literal(self) -> bool {
                    matches!(self, #(#literals)|*)
                }

                pub fn from_char(c: char) -> Option<SyntaxKind> {
                    let tok = match c {
                        #(#char_token_values => #char_tokens,)*
                        _ => return None,
                    };
                    Some(tok)
                }
            }
        };

        generated.to_string()
    }

    pub fn generate_macro(&self) -> String {
        let punctuation_values = self.source.punctuations.iter().map(|(token, _name)| {
            quote! { #token }
        });

        let punctuations = as_idents(self.source.punctuations, |(_token, name)| name);

        let reserved_keywords_values = self.source.reserved_keywords.iter().map(|keyword| {
            quote! { #keyword }
        });

        let reserved_keywords = as_idents(self.source.reserved_keywords, |i| {
            format!("{}_KW", utils::to_upper_snake_case(i))
        });

        let generated = quote! {
            #[macro_export]
            macro_rules! T {
                #([#punctuation_values] => { $crate::SyntaxKind::#punctuations };)*
                 #([#reserved_keywords_values] => { $crate::SyntaxKind::#reserved_keywords };)*
            }
        };

        generated.to_string()
    }
}

fn as_idents<'a, F, R, A>(values: &'a [A], map: F) -> Vec<Ident>
where
    F: Fn(&'a A) -> R + 'a,
    R: IdentFragment,
{
    values
        .iter()
        .map(|i| format_ident!("{}", map(i)))
        .collect::<Vec<_>>()
}
