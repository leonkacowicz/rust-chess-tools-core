use super::square::Square;
use crate::core::Piece;
use std::fmt::{Display, Formatter};
use Move::*;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    NormalMove {
        origin: Square,
        dest: Square,
        piece: Piece,
    },
    CastleKingSideWhite,
    CastleQueenSideWhite,
    CastleKingSideBlack,
    CastleQueenSideBlack,
    Promotion {
        origin: Square,
        dest: Square,
        piece: Piece,
    },
    EnPassant {
        origin: Square,
        dest: Square,
        capture: Square,
    },
    NullMove,
}
impl Move {
    pub const fn new(piece: Piece, origin: Square, dest: Square) -> Move {
        NormalMove {
            origin,
            dest,
            piece,
        }
    }

    pub const fn promote(origin: Square, dest: Square, piece: Piece) -> Move {
        Promotion {
            origin,
            dest,
            piece,
        }
    }

    pub const fn en_passant(origin: Square, dest: Square, capture: Square) -> Move {
        EnPassant {
            origin,
            dest,
            capture,
        }
    }
}
impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalMove { origin, dest, .. } => f.write_fmt(format_args!("{}{}", origin, dest)),
            EnPassant { origin, dest, .. } => f.write_fmt(format_args!("{}{}", origin, dest)),
            CastleKingSideWhite => f.write_str("e1g1"),
            CastleQueenSideWhite => f.write_str("e1c1"),
            CastleKingSideBlack => f.write_str("e8g8"),
            CastleQueenSideBlack => f.write_str("e8c8"),
            Promotion {
                origin,
                dest,
                piece,
            } => f.write_fmt(format_args!("{}{}{}", origin, dest, piece)),
            NullMove => f.write_str("(none)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::r#move::Move;
    use std::mem::size_of;

    #[test]
    pub fn test_move() {
        println!("Size of NormalMove {} ", size_of::<Move>())
    }
}
