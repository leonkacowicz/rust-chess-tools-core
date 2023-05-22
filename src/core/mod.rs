use self::Color::*;
use crate::core::Piece::*;
use std::fmt::{Display, Formatter, Write};
use std::ops::Index;

pub mod bitboard;
mod bitboard_attacks;
pub mod bitboard_constants;
pub mod board;
pub mod fen;
pub mod magic_bitboard;
mod magic_bitboard_magic_numbers;
pub mod r#move;
pub mod move_generator;
pub mod square;
pub mod square_constants;
pub mod zobrist_hash;

pub const UP: i8 = 8;
pub const DOWN: i8 = -8;
pub const LEFT: i8 = -1;
pub const RIGHT: i8 = 1;
pub const UP_LEFT: i8 = UP + LEFT;
pub const DOWN_LEFT: i8 = DOWN + LEFT;
pub const UP_RIGHT: i8 = UP + RIGHT;
pub const DOWN_RIGHT: i8 = DOWN + RIGHT;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

impl Piece {
    pub fn from(value: u8) -> Piece {
        match value {
            0 => PAWN,
            1 => KNIGHT,
            2 => BISHOP,
            3 => ROOK,
            4 => QUEEN,
            5 => KING,
            _ => panic!(),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PAWN => 'p',
            KNIGHT => 'n',
            BISHOP => 'b',
            ROOK => 'r',
            QUEEN => 'q',
            KING => 'k',
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    pub const fn opposite(self) -> Color {
        unsafe {
            return std::mem::transmute::<u8, Color>((self as u8) ^ 1);
        }
    }

    pub const fn index(self) -> i8 {
        match self {
            WHITE => 0,
            BLACK => 1,
        }
    }

    pub const fn fwd_dir(self) -> i8 {
        match self {
            WHITE => UP,
            BLACK => DOWN,
        }
    }
}

impl Into<usize> for Color {
    fn into(self) -> usize {
        match self {
            WHITE => 0,
            BLACK => 1,
        }
    }
}

impl<T> Index<Piece> for [T; 6] {
    type Output = T;

    #[inline]
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Color::{BLACK, WHITE};
    #[test]
    fn opposite_color() {
        assert_eq!(BLACK.opposite(), WHITE);
    }
}
