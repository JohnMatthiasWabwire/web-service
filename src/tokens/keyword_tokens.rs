#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Keyword Token Defintion
pub type KeywordToken = &'static str;

// Keyword Tokens
pub const ABSTRACT_TOKEN: KeywordToken = "abstract";
pub const ANY_TOKEN: KeywordToken = "any";
pub const AS_TOKEN: KeywordToken = "as";
pub const ASYNC_TOKEN: KeywordToken = "async";
pub const AWAIT_TOKEN: KeywordToken = "await";
pub const BOX_TOKEN: KeywordToken = "box";
pub const BREAK_TOKEN: KeywordToken = "break";
pub const CASE_TOKEN: KeywordToken = "case";
pub const CATCH_TOKEN: KeywordToken = "catch";
pub const CONST_TOKEN: KeywordToken = "const";
pub const CONTINUE_TOKEN: KeywordToken = "continue";
pub const DEFER_TOKEN: KeywordToken = "defer";
pub const DEINIT_TOKEN: KeywordToken = "deinit";
pub const DO_TOKEN: KeywordToken = "do";
pub const DYN_TOKEN: KeywordToken = "dyn";
pub const DYNAMIC_TOKEN: KeywordToken = "dynamic";
pub const ELSE_TOKEN: KeywordToken = "else";
pub const ENUM_TOKEN: KeywordToken = "enum";
pub const EXPORT_TOKEN: KeywordToken = "export";
pub const EXTERN_TOKEN: KeywordToken = "extern";
pub const FALSE_TOKEN: KeywordToken = "false";
pub const FINAL_TOKEN: KeywordToken = "final";
pub const FN_TOKEN: KeywordToken = "fn";
pub const FUNC_TOKEN: KeywordToken = "func";
pub const FUNCTION_TOKEN: KeywordToken = "function";
pub const FOR_TOKEN: KeywordToken = "for";
pub const IF_TOKEN: KeywordToken = "if";
pub const IMPL_TOKEN: KeywordToken = "impl";
pub const IMPORT_TOKEN: KeywordToken = "import";
pub const INIT_TOKEN: KeywordToken = "init";
pub const INTERFACE_TOKEN: KeywordToken = "interface";
pub const IN_TOKEN: KeywordToken = "in";
pub const LET_TOKEN: KeywordToken = "let";
pub const MACRO_TOKEN: KeywordToken = "macro";
pub const MATCH_TOKEN: KeywordToken = "match";
pub const MOD_TOKEN: KeywordToken = "mod";
pub const MODULE_TOKEN: KeywordToken = "module";
pub const MOVE_TOKEN: KeywordToken = "move";
pub const MUT_TOKEN: KeywordToken = "mut";
pub const MUTABLE_TOKEN: KeywordToken = "mutable";
pub const NIL_TOKEN: KeywordToken = "nil";
pub const NULL_TOKEN: KeywordToken = "null";
pub const OVERRIDE_TOKEN: KeywordToken = "override";
pub const PACKAGE_TOKEN: KeywordToken = "package";
pub const PRIV_TOKEN: KeywordToken = "priv";
pub const PRIVATE_TOKEN: KeywordToken = "private";
pub const PUB_TOKEN: KeywordToken = "pub";
pub const PUBLIC_TOKEN: KeywordToken = "public";
pub const RAW_TOKEN: KeywordToken = "raw";
pub const REF_TOKEN: KeywordToken = "ref";
pub const RETURN_TOKEN: KeywordToken = "return";
pub const SELF_TOKEN: KeywordToken = "self";
pub const STATIC_TOKEN: KeywordToken = "static";
pub const STRUCT_TOKEN: KeywordToken = "struct";
pub const SUPER_TOKEN: KeywordToken = "super";
pub const SWITCH_TOKEN: KeywordToken = "switch";
pub const THROW_TOKEN: KeywordToken = "throw";
pub const TRAIT_TOKEN: KeywordToken = "trait";
pub const TRUE_TOKEN: KeywordToken = "true";
pub const TRY_TOKEN: KeywordToken = "try";
pub const TYPE_TOKEN: KeywordToken = "type";
pub const TYPEDEF_TOKEN: KeywordToken = "typedef";
pub const TYPEOF_TOKEN: KeywordToken = "typeof";
pub const UNION_TOKEN: KeywordToken = "union";
pub const UNSAFE_TOKEN: KeywordToken = "unsafe";
pub const USE_TOKEN: KeywordToken = "use";
pub const UNSIZED_TOKEN: KeywordToken = "unsized";
pub const VAR_TOKEN: KeywordToken = "var";
pub const VARIABLE_TOKEN: KeywordToken = "variable";
pub const WHERE_TOKEN: KeywordToken = "where";
pub const WHILE_TOKEN: KeywordToken = "while";

// Keyword Token Vector
pub fn keywords_vector() -> Vec<KeywordToken> {
    let keywords: Vec<KeywordToken> = Vec::from([
        ABSTRACT_TOKEN,
        ANY_TOKEN,
        AS_TOKEN,
        ASYNC_TOKEN,
        AWAIT_TOKEN,
        BOX_TOKEN,
        BREAK_TOKEN,
        CASE_TOKEN,
        CATCH_TOKEN,
        CONST_TOKEN,
        CONTINUE_TOKEN,
        DEFER_TOKEN,
        DEINIT_TOKEN,
        DO_TOKEN,
        DYN_TOKEN,
        DYNAMIC_TOKEN,
        ELSE_TOKEN,
        ENUM_TOKEN,
        EXPORT_TOKEN,
        EXTERN_TOKEN,
        FALSE_TOKEN,
        FINAL_TOKEN,
        FN_TOKEN,
        FUNC_TOKEN,
        FUNCTION_TOKEN,
        FOR_TOKEN,
        IF_TOKEN,
        IMPL_TOKEN,
        IMPORT_TOKEN,
        INIT_TOKEN,
        INTERFACE_TOKEN,
        IN_TOKEN,
        LET_TOKEN,
        MACRO_TOKEN,
        MATCH_TOKEN,
        MOD_TOKEN,
        MODULE_TOKEN,
        MOVE_TOKEN,
        MUT_TOKEN,
        MUTABLE_TOKEN,
        NIL_TOKEN,
        NULL_TOKEN,
        OVERRIDE_TOKEN,
        PACKAGE_TOKEN,
        PRIV_TOKEN,
        PRIVATE_TOKEN,
        PUB_TOKEN,
        PUBLIC_TOKEN,
        RAW_TOKEN,
        REF_TOKEN,
        RETURN_TOKEN,
        SELF_TOKEN,
        STATIC_TOKEN,
        STRUCT_TOKEN,
        SUPER_TOKEN,
        SWITCH_TOKEN,
        THROW_TOKEN,
        TRAIT_TOKEN,
        TRUE_TOKEN,
        TRY_TOKEN,
        TYPE_TOKEN,
        TYPEDEF_TOKEN,
        TYPEOF_TOKEN,
        UNION_TOKEN,
        UNSAFE_TOKEN,
        USE_TOKEN,
        UNSIZED_TOKEN,
        VAR_TOKEN,
        VARIABLE_TOKEN,
        WHERE_TOKEN,
        WHILE_TOKEN,
    ]);

    return keywords;
}
