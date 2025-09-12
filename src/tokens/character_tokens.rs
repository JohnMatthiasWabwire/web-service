#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Character Token Defintion
pub type CharacterToken = &'static str;

// Character Tokens
pub const A_TOKEN: CharacterToken = "a";
pub const B_TOKEN: CharacterToken = "b";
pub const C_TOKEN: CharacterToken = "c";
pub const D_TOKEN: CharacterToken = "d";
pub const E_TOKEN: CharacterToken = "e";
pub const F_TOKEN: CharacterToken = "f";
pub const G_TOKEN: CharacterToken = "g";
pub const H_TOKEN: CharacterToken = "h";
pub const I_TOKEN: CharacterToken = "i";
pub const J_TOKEN: CharacterToken = "j";
pub const K_TOKEN: CharacterToken = "k";
pub const L_TOKEN: CharacterToken = "l";
pub const M_TOKEN: CharacterToken = "m";
pub const N_TOKEN: CharacterToken = "n";
pub const O_TOKEN: CharacterToken = "o";
pub const P_TOKEN: CharacterToken = "p";
pub const Q_TOKEN: CharacterToken = "q";
pub const R_TOKEN: CharacterToken = "r";
pub const S_TOKEN: CharacterToken = "s";
pub const T_TOKEN: CharacterToken = "t";
pub const U_TOKEN: CharacterToken = "u";
pub const V_TOKEN: CharacterToken = "v";
pub const W_TOKEN: CharacterToken = "w";
pub const X_TOKEN: CharacterToken = "x";
pub const Y_TOKEN: CharacterToken = "y";
pub const Z_TOKEN: CharacterToken = "z";
pub const CAPITAL_A_TOKEN: CharacterToken = "A";
pub const CAPITAL_B_TOKEN: CharacterToken = "B";
pub const CAPITAL_C_TOKEN: CharacterToken = "C";
pub const CAPITAL_D_TOKEN: CharacterToken = "D";
pub const CAPITAL_E_TOKEN: CharacterToken = "E";
pub const CAPITAL_F_TOKEN: CharacterToken = "F";
pub const CAPITAL_G_TOKEN: CharacterToken = "G";
pub const CAPITAL_H_TOKEN: CharacterToken = "H";
pub const CAPITAL_I_TOKEN: CharacterToken = "I";
pub const CAPITAL_J_TOKEN: CharacterToken = "J";
pub const CAPITAL_K_TOKEN: CharacterToken = "K";
pub const CAPITAL_L_TOKEN: CharacterToken = "L";
pub const CAPITAL_M_TOKEN: CharacterToken = "M";
pub const CAPITAL_N_TOKEN: CharacterToken = "N";
pub const CAPITAL_O_TOKEN: CharacterToken = "O";
pub const CAPITAL_P_TOKEN: CharacterToken = "P";
pub const CAPITAL_Q_TOKEN: CharacterToken = "Q";
pub const CAPITAL_R_TOKEN: CharacterToken = "R";
pub const CAPITAL_S_TOKEN: CharacterToken = "S";
pub const CAPITAL_T_TOKEN: CharacterToken = "T";
pub const CAPITAL_U_TOKEN: CharacterToken = "U";
pub const CAPITAL_V_TOKEN: CharacterToken = "V";
pub const CAPITAL_W_TOKEN: CharacterToken = "W";
pub const CAPITAL_X_TOKEN: CharacterToken = "X";
pub const CAPITAL_Y_TOKEN: CharacterToken = "Y";
pub const CAPITAL_Z_TOKEN: CharacterToken = "Z";

// Character Token Vector
pub fn characters_vector() -> Vec<CharacterToken> {
    let characters: Vec<CharacterToken> = Vec::from([
        A_TOKEN,
        B_TOKEN,
        C_TOKEN,
        D_TOKEN,
        E_TOKEN,
        F_TOKEN,
        G_TOKEN,
        H_TOKEN,
        I_TOKEN,
        J_TOKEN,
        K_TOKEN,
        L_TOKEN,
        M_TOKEN,
        N_TOKEN,
        O_TOKEN,
        P_TOKEN,
        Q_TOKEN,
        R_TOKEN,
        S_TOKEN,
        T_TOKEN,
        U_TOKEN,
        V_TOKEN,
        W_TOKEN,
        X_TOKEN,
        Y_TOKEN,
        Z_TOKEN,
        CAPITAL_A_TOKEN,
        CAPITAL_B_TOKEN,
        CAPITAL_C_TOKEN,
        CAPITAL_D_TOKEN,
        CAPITAL_E_TOKEN,
        CAPITAL_F_TOKEN,
        CAPITAL_G_TOKEN,
        CAPITAL_H_TOKEN,
        CAPITAL_I_TOKEN,
        CAPITAL_J_TOKEN,
        CAPITAL_K_TOKEN,
        CAPITAL_L_TOKEN,
        CAPITAL_M_TOKEN,
        CAPITAL_N_TOKEN,
        CAPITAL_O_TOKEN,
        CAPITAL_P_TOKEN,
        CAPITAL_Q_TOKEN,
        CAPITAL_R_TOKEN,
        CAPITAL_S_TOKEN,
        CAPITAL_T_TOKEN,
        CAPITAL_U_TOKEN,
        CAPITAL_V_TOKEN,
        CAPITAL_W_TOKEN,
        CAPITAL_X_TOKEN,
        CAPITAL_Y_TOKEN,
        CAPITAL_Z_TOKEN,
    ]);

    return characters;
}
