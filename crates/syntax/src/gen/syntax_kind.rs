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
    pub fn is_seperator(self) -> bool {
        matches!(
            self,
            L_PAREN
                | R_PAREN
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
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
}
#[macro_export]
macro_rules ! T { [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }
