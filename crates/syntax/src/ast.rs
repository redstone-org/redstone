//! Abstract Syntax Tree
//!
//! This is an extra view on top of CST.
//! Most of AST code is auto-generated from `java.ungram`

use crate::{
    cst::{SyntaxNode, SyntaxToken},
    SyntaxKind,
};

/// AST Non-terminals aka Productions
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

/// AST Terminals that holds an inner value
pub trait AstToken {
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}
