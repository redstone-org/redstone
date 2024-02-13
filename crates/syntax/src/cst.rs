//! Concrete Syntax Tree (CST)
//!
//! Uses `rowan` internally.

use rowan::Language;

use crate::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JavaLanguage {}

impl Language for JavaLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<JavaLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<JavaLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<JavaLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<JavaLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<JavaLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<JavaLanguage>;
