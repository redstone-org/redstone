#![allow(bad_style, missing_docs, unreachable_pub)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    SEMICOLON,
    COMMA,
    DOT,
    DOT3,
    AT,
    COLON2,
    ABSTRACT_KW,
    ASSERT_KW,
    BOOLEAN_KW,
    BREAK_KW,
    BYTE_KW,
    CASE_KW,
    CATCH_KW,
    CHAR_KW,
    CLASS_KW,
    CONST_KW,
    CONTINUE_KW,
    DEFAULT_KW,
    DO_KW,
    DOUBLE_KW,
    ELSE_KW,
    ENUM_KW,
    EXTENDS_KW,
    FINAL_KW,
    FINALLY_KW,
    FLOAT_KW,
    FOR_KW,
    IF_KW,
    GOTO_KW,
    IMPLEMENTS_KW,
    IMPORT_KW,
    INSTANCEOF_KW,
    INT_KW,
    INTERFACE_KW,
    LONG_KW,
    NATIVE_KW,
    NEW_KW,
    PACKAGE_KW,
    PRIVATE_KW,
    PROTECTED_KW,
    PUBLIC_KW,
    RETURN_KW,
    SHORT_KW,
    STATIC_KW,
    STRICTFP_KW,
    SUPER_KW,
    SWITCH_KW,
    SYNCHRONIZED_KW,
    THIS_KW,
    THROW_KW,
    THROWS_KW,
    TRANSIENT_KW,
    TRY_KW,
    VOID_KW,
    VOLATILE_KW,
    WHEN_KW,
    INTEGER,
    FLOATING_POINT,
    BOOLEAN,
    CHARACTER,
    STRING,
    TEXT_BLOCK,
    NULL,
    IDENT,
    WHITESPACE,
    COMMENT,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            ABSTRACT_KW
                | ASSERT_KW
                | BOOLEAN_KW
                | BREAK_KW
                | BYTE_KW
                | CASE_KW
                | CATCH_KW
                | CHAR_KW
                | CLASS_KW
                | CONST_KW
                | CONTINUE_KW
                | DEFAULT_KW
                | DO_KW
                | DOUBLE_KW
                | ELSE_KW
                | ENUM_KW
                | EXTENDS_KW
                | FINAL_KW
                | FINALLY_KW
                | FLOAT_KW
                | FOR_KW
                | IF_KW
                | GOTO_KW
                | IMPLEMENTS_KW
                | IMPORT_KW
                | INSTANCEOF_KW
                | INT_KW
                | INTERFACE_KW
                | LONG_KW
                | NATIVE_KW
                | NEW_KW
                | PACKAGE_KW
                | PRIVATE_KW
                | PROTECTED_KW
                | PUBLIC_KW
                | RETURN_KW
                | SHORT_KW
                | STATIC_KW
                | STRICTFP_KW
                | SUPER_KW
                | SWITCH_KW
                | SYNCHRONIZED_KW
                | THIS_KW
                | THROW_KW
                | THROWS_KW
                | TRANSIENT_KW
                | TRY_KW
                | VOID_KW
                | VOLATILE_KW
                | WHEN_KW
        )
    }
    pub fn is_punctuation(self) -> bool {
        matches!(
            self,
            L_PAREN
                | R_PAREN
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
                | L_ANGLE
                | R_ANGLE
                | SEMICOLON
                | COMMA
                | DOT
                | DOT3
                | AT
                | COLON2
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            INTEGER | FLOATING_POINT | BOOLEAN | CHARACTER | STRING | TEXT_BLOCK | NULL
        )
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            ';' => SEMICOLON,
            ',' => COMMA,
            '.' => DOT,
            '@' => AT,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { ["("] => { $ crate :: SyntaxKind :: L_PAREN } ; [")"] => { $ crate :: SyntaxKind :: R_PAREN } ; ["{"] => { $ crate :: SyntaxKind :: L_CURLY } ; ["}"] => { $ crate :: SyntaxKind :: R_CURLY } ; ["["] => { $ crate :: SyntaxKind :: L_BRACK } ; ["]"] => { $ crate :: SyntaxKind :: R_BRACK } ; ["<"] => { $ crate :: SyntaxKind :: L_ANGLE } ; [">"] => { $ crate :: SyntaxKind :: R_ANGLE } ; [";"] => { $ crate :: SyntaxKind :: SEMICOLON } ; [","] => { $ crate :: SyntaxKind :: COMMA } ; ["."] => { $ crate :: SyntaxKind :: DOT } ; ["..."] => { $ crate :: SyntaxKind :: DOT3 } ; ["@"] => { $ crate :: SyntaxKind :: AT } ; ["::"] => { $ crate :: SyntaxKind :: COLON2 } ; ["abstract"] => { $ crate :: SyntaxKind :: ABSTRACT_KW } ; ["assert"] => { $ crate :: SyntaxKind :: ASSERT_KW } ; ["boolean"] => { $ crate :: SyntaxKind :: BOOLEAN_KW } ; ["break"] => { $ crate :: SyntaxKind :: BREAK_KW } ; ["byte"] => { $ crate :: SyntaxKind :: BYTE_KW } ; ["case"] => { $ crate :: SyntaxKind :: CASE_KW } ; ["catch"] => { $ crate :: SyntaxKind :: CATCH_KW } ; ["char"] => { $ crate :: SyntaxKind :: CHAR_KW } ; ["class"] => { $ crate :: SyntaxKind :: CLASS_KW } ; ["const"] => { $ crate :: SyntaxKind :: CONST_KW } ; ["continue"] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; ["default"] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; ["do"] => { $ crate :: SyntaxKind :: DO_KW } ; ["double"] => { $ crate :: SyntaxKind :: DOUBLE_KW } ; ["else"] => { $ crate :: SyntaxKind :: ELSE_KW } ; ["enum"] => { $ crate :: SyntaxKind :: ENUM_KW } ; ["extends"] => { $ crate :: SyntaxKind :: EXTENDS_KW } ; ["final"] => { $ crate :: SyntaxKind :: FINAL_KW } ; ["finally"] => { $ crate :: SyntaxKind :: FINALLY_KW } ; ["float"] => { $ crate :: SyntaxKind :: FLOAT_KW } ; ["for"] => { $ crate :: SyntaxKind :: FOR_KW } ; ["if"] => { $ crate :: SyntaxKind :: IF_KW } ; ["goto"] => { $ crate :: SyntaxKind :: GOTO_KW } ; ["implements"] => { $ crate :: SyntaxKind :: IMPLEMENTS_KW } ; ["import"] => { $ crate :: SyntaxKind :: IMPORT_KW } ; ["instanceof"] => { $ crate :: SyntaxKind :: INSTANCEOF_KW } ; ["int"] => { $ crate :: SyntaxKind :: INT_KW } ; ["interface"] => { $ crate :: SyntaxKind :: INTERFACE_KW } ; ["long"] => { $ crate :: SyntaxKind :: LONG_KW } ; ["native"] => { $ crate :: SyntaxKind :: NATIVE_KW } ; ["new"] => { $ crate :: SyntaxKind :: NEW_KW } ; ["package"] => { $ crate :: SyntaxKind :: PACKAGE_KW } ; ["private"] => { $ crate :: SyntaxKind :: PRIVATE_KW } ; ["protected"] => { $ crate :: SyntaxKind :: PROTECTED_KW } ; ["public"] => { $ crate :: SyntaxKind :: PUBLIC_KW } ; ["return"] => { $ crate :: SyntaxKind :: RETURN_KW } ; ["short"] => { $ crate :: SyntaxKind :: SHORT_KW } ; ["static"] => { $ crate :: SyntaxKind :: STATIC_KW } ; ["strictfp"] => { $ crate :: SyntaxKind :: STRICTFP_KW } ; ["super"] => { $ crate :: SyntaxKind :: SUPER_KW } ; ["switch"] => { $ crate :: SyntaxKind :: SWITCH_KW } ; ["synchronized"] => { $ crate :: SyntaxKind :: SYNCHRONIZED_KW } ; ["this"] => { $ crate :: SyntaxKind :: THIS_KW } ; ["throw"] => { $ crate :: SyntaxKind :: THROW_KW } ; ["throws"] => { $ crate :: SyntaxKind :: THROWS_KW } ; ["transient"] => { $ crate :: SyntaxKind :: TRANSIENT_KW } ; ["try"] => { $ crate :: SyntaxKind :: TRY_KW } ; ["void"] => { $ crate :: SyntaxKind :: VOID_KW } ; ["volatile"] => { $ crate :: SyntaxKind :: VOLATILE_KW } ; ["when"] => { $ crate :: SyntaxKind :: WHEN_KW } ; [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }
