use lazy_static::lazy_static;
use rand::{RngCore, SeedableRng};

use crate::core::bitboard::BitBoard;
use crate::core::bitboard_constants::*;
use crate::core::board::Board;
use crate::core::r#move::Move;
use crate::core::square::Square;
use crate::core::square_constants::*;
use crate::core::zobrist_hash::CastlingSide::{KingSide, QueenSide};
use crate::core::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
enum CastlingSide {
    KingSide,
    QueenSide,
}

impl<T> Index<CastlingSide> for [T; 2] {
    type Output = T;

    fn index(&self, index: CastlingSide) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<CastlingSide> for [T; 2] {
    fn index_mut(&mut self, index: CastlingSide) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

struct ZobristTable {
    pieces: [[[u64; 2]; 6]; 64],
    side: u64,
    castling: [[u64; 2]; 2],
    en_passant: [u64; 8],
}

lazy_static! {
    static ref ZOBRIST_TABLE: ZobristTable = zobrist_init(1);
}

fn zobrist_init(seed: u64) -> ZobristTable {
    let mut ans = ZobristTable {
        pieces: [[[0 as u64; 2]; 6]; 64],
        side: 0,
        castling: [[0; 2]; 2],
        en_passant: [0; 8],
    };
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for i in 0..64 {
        for j in 0..6 {
            ans.pieces[i][j][WHITE] = rng.next_u64();
            ans.pieces[i][j][BLACK] = rng.next_u64();
        }
    }
    ans.side = rng.next_u64();
    ans.castling[KingSide][WHITE] = rng.next_u64();
    ans.castling[KingSide][BLACK] = rng.next_u64();
    ans.castling[QueenSide][WHITE] = rng.next_u64();
    ans.castling[QueenSide][BLACK] = rng.next_u64();
    for i in 0..8 {
        ans.en_passant[i] = rng.next_u64();
    }
    ans
}

pub fn hash(board: &Board) -> u64 {
    let mut h: u64 = 0;
    if board.can_castle_king_side[WHITE] {
        h ^= ZOBRIST_TABLE.castling[KingSide][WHITE];
    }
    if board.can_castle_king_side[BLACK] {
        h ^= ZOBRIST_TABLE.castling[KingSide][BLACK];
    }
    if board.can_castle_queen_side[WHITE] {
        h ^= ZOBRIST_TABLE.castling[QueenSide][WHITE];
    }
    if board.can_castle_queen_side[BLACK] {
        h ^= ZOBRIST_TABLE.castling[QueenSide][BLACK];
    }

    if let Some(en_passant) = board.en_passant {
        h ^= ZOBRIST_TABLE.en_passant[en_passant.file() as usize];
    }

    for color in 0..2 {
        for piece in 0..5 {
            let mut remaining = board.piece_of_color[color] & board.piece_of_type[piece];
            while !remaining.empty() {
                let sq = remaining.pop_lsb();
                h ^= ZOBRIST_TABLE.pieces[sq][piece][color];
            }
        }
    }
    h ^= ZOBRIST_TABLE.pieces[board.king_pos[WHITE]][KING][WHITE];
    h ^= ZOBRIST_TABLE.pieces[board.king_pos[BLACK]][KING][BLACK];
    if board.side_to_play == BLACK {
        h ^= ZOBRIST_TABLE.side;
    }
    h
}

pub fn hash_update(board: &Board, prev: u64, m: Move) -> u64 {
    let mut hash = prev;
    hash ^= ZOBRIST_TABLE.side;
    if let Some(en_passant) = board.en_passant {
        hash ^= ZOBRIST_TABLE.en_passant[en_passant.file() as usize];
    }
    let (mut hash, castling_rights_loss) = match m {
        Move::NormalMove {
            piece,
            origin,
            dest,
        } => hash_update_normal(board, hash, piece, origin, dest),
        Move::CastleKingSideWhite => hash_update_castle_king_side_white(hash),
        Move::CastleQueenSideWhite => hash_update_castle_queen_side_white(hash),
        Move::CastleKingSideBlack => hash_update_castle_king_side_black(hash),
        Move::CastleQueenSideBlack => hash_update_castle_queen_side_black(hash),
        Move::Promotion {
            origin,
            dest,
            piece,
        } => hash_update_promotion(hash, origin, dest, piece, board),
        Move::EnPassant {
            origin,
            dest,
            capture,
        } => hash_update_en_passant(hash, origin, dest, capture, board.side_to_play),
        Move::NullMove => (hash, [[false; 2]; 2]),
    };
    if board.can_castle_king_side[WHITE] && castling_rights_loss[KingSide][WHITE] {
        hash ^= ZOBRIST_TABLE.castling[KingSide][WHITE];
    }
    if board.can_castle_king_side[BLACK] && castling_rights_loss[KingSide][BLACK] {
        hash ^= ZOBRIST_TABLE.castling[KingSide][BLACK];
    }
    if board.can_castle_queen_side[WHITE] && castling_rights_loss[QueenSide][WHITE] {
        hash ^= ZOBRIST_TABLE.castling[QueenSide][WHITE];
    }
    if board.can_castle_queen_side[BLACK] && castling_rights_loss[QueenSide][BLACK] {
        hash ^= ZOBRIST_TABLE.castling[QueenSide][BLACK];
    }
    hash
}

fn hash_update_en_passant(
    hash: u64,
    origin: Square,
    dest: Square,
    capture: Square,
    side_to_play: Color,
) -> (u64, [[bool; 2]; 2]) {
    (
        hash ^ ZOBRIST_TABLE.pieces[origin][PAWN][side_to_play]
            ^ ZOBRIST_TABLE.pieces[dest][PAWN][side_to_play]
            ^ ZOBRIST_TABLE.pieces[capture][PAWN][side_to_play.opposite()],
        [[false; 2]; 2],
    )
}

fn hash_update_promotion(
    hash: u64,
    origin: Square,
    dest: Square,
    piece: Piece,
    board: &Board,
) -> (u64, [[bool; 2]; 2]) {
    let dest_bb = BitBoard::from_square(dest);
    let opposite = board.side_to_play.opposite();
    if board.piece_of_color[opposite] * dest_bb {
        if let Some(capture) = board.piece_at(dest_bb) {
            return (
                hash ^ ZOBRIST_TABLE.pieces[origin][PAWN][board.side_to_play]
                    ^ ZOBRIST_TABLE.pieces[dest][piece][board.side_to_play]
                    ^ ZOBRIST_TABLE.pieces[dest][capture][opposite],
                [
                    [dest_bb * BB_H1, dest_bb * BB_H8],
                    [dest_bb * BB_A1, dest_bb * BB_A8],
                ],
            );
        }
    }
    (
        hash ^ ZOBRIST_TABLE.pieces[origin][PAWN][board.side_to_play]
            ^ ZOBRIST_TABLE.pieces[dest][piece][board.side_to_play],
        [[false; 2]; 2],
    )
}

#[inline]
fn hash_update_castle_king_side_white(hash: u64) -> (u64, [[bool; 2]; 2]) {
    (
        hash ^ ZOBRIST_TABLE.pieces[SQ_E1][KING][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_G1][KING][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_F1][ROOK][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_H1][ROOK][WHITE],
        [[true, false], [true, false]],
    )
}

#[inline]
fn hash_update_castle_king_side_black(hash: u64) -> (u64, [[bool; 2]; 2]) {
    (
        hash ^ ZOBRIST_TABLE.pieces[SQ_E8][KING][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_G8][KING][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_F8][ROOK][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_H8][ROOK][BLACK],
        [[false, true], [false, true]],
    )
}

#[inline]
fn hash_update_castle_queen_side_white(hash: u64) -> (u64, [[bool; 2]; 2]) {
    (
        hash ^ ZOBRIST_TABLE.pieces[SQ_E1][KING][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_C1][KING][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_D1][ROOK][WHITE]
            ^ ZOBRIST_TABLE.pieces[SQ_A1][ROOK][WHITE],
        [[true, false], [true, false]],
    )
}

#[inline]
fn hash_update_castle_queen_side_black(hash: u64) -> (u64, [[bool; 2]; 2]) {
    (
        hash ^ ZOBRIST_TABLE.pieces[SQ_E8][KING][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_C8][KING][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_D8][ROOK][BLACK]
            ^ ZOBRIST_TABLE.pieces[SQ_A8][ROOK][BLACK],
        [[false, true], [false, true]],
    )
}

fn hash_update_normal(
    board: &Board,
    prev: u64,
    piece: Piece,
    origin: Square,
    dest: Square,
) -> (u64, [[bool; 2]; 2]) {
    let mut hash = prev;
    let origin_bb = BitBoard::from_square(origin);
    let dest_bb = BitBoard::from_square(dest);
    hash ^= ZOBRIST_TABLE.pieces[origin][piece][board.side_to_play];
    hash ^= ZOBRIST_TABLE.pieces[dest][piece][board.side_to_play];
    if let Some(capture) = board.piece_at(dest_bb) {
        hash ^= ZOBRIST_TABLE.pieces[dest][capture][board.side_to_play.opposite()];
    }
    if piece == PAWN {
        if (dest.rank() == origin.rank() + 2) || (dest.rank() + 2 == origin.rank()) {
            hash ^= ZOBRIST_TABLE.en_passant[origin.file() as usize];
        }
    }
    let both = origin_bb | dest_bb;
    (
        hash,
        [
            [
                (both * BB_H1) || origin == SQ_E1,
                (both * BB_H8) || origin == SQ_E8,
            ],
            [
                (both * BB_A1) || origin == SQ_E1,
                (both * BB_A8) || origin == SQ_E8,
            ],
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::magic_bitboard::{magic_tables, MagicTables};
    use crate::core::move_generator::MoveGenerator;
    use crate::core::zobrist_hash::{hash, hash_update};
    use lazy_static::lazy_static;
    use rand::{RngCore, SeedableRng};

    lazy_static! {
        static ref MAGIC_TABLES: MagicTables = magic_tables();
    }

    #[test]
    pub fn test() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        for _i in 0..20000 {
            let mut board = Board::from_initial_position();
            let mut h = hash(&board);
            for _j in 0..300 {
                let mut mg = MoveGenerator::new(&board, &MAGIC_TABLES);
                mg.generate();
                if mg.moves.len() == 0 {
                    break;
                }
                let m = mg.moves[rng.next_u64() as usize % mg.moves.len()];
                let updated_hash = hash_update(&board, h, m);
                let mut new_board = board;
                new_board.make_move(m);

                let new_hash = hash(&new_board);

                if updated_hash != new_hash {
                    new_board = board;
                    new_board.make_move(m);
                }
                assert_eq!(
                    updated_hash, new_hash,
                    "{}\n{}\n{}\n{:?}\n{:?}",
                    board, new_board, m, board, new_board
                );
                h = updated_hash;
                board = new_board;
            }
        }
    }
}
