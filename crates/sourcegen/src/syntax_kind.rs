use proc_macro2::Ident;
use quote::quote;

pub struct GenerateSyntaxKinds {
    pub seperators: Vec<Ident>,
    pub reserved_keywords: Vec<Ident>,
    pub literals: Vec<Ident>,
}

impl GenerateSyntaxKinds {
    pub fn generate(&self) -> String {
        let seperators = &self.seperators;
        let reserved_keywords = &self.reserved_keywords;
        let literals = &self.literals;
        let tokens: Vec<&str> = vec![];
        let nodes: Vec<&str> = vec![];

        let generated = quote! {
            #![allow(bad_style, missing_docs, unreachable_pub)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            #[repr(u16)]
            pub enum SyntaxKind {
                #[doc(hidden)]
                TOMBSTONE,
                #[doc(hidden)]
                EOF,
                #(#seperators,)*
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

                pub fn is_seperator(self) -> bool {

                    matches!(self, #(#seperators)|*)

                }

                pub fn is_literal(self) -> bool {
                    matches!(self, #(#literals)|*)
                }
            }

            #[macro_export]
            macro_rules! T {
                [lifetime_ident] => { $crate::SyntaxKind::LIFETIME_IDENT };
                [ident] => { $crate::SyntaxKind::IDENT };
                [shebang] => { $crate::SyntaxKind::SHEBANG };
            }
        };

        generated.to_string()
    }
}
