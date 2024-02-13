use crate::{
    ast::AstToken,
    SyntaxKind::{self, *},
    SyntaxToken,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Ident {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IDENT
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Whitespace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == WHITESPACE
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == COMMENT
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Integer {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INTEGER
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatingPoint {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for FloatingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for FloatingPoint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FLOATING_POINT
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boolean {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Boolean {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BOOLEAN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Character {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Character {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CHARACTER
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRING
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextBlock {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for TextBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for TextBlock {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TEXT_BLOCK
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
