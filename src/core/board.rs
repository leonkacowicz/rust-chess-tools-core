use crate::core::bitboard::BitBoard;
use crate::core::bitboard_attacks::*;
use crate::core::bitboard_constants::*;
use crate::core::magic_bitboard::*;
use crate::core::r#move::Move;
use crate::core::square::Square;
use crate::core::square_constants::*;
use crate::core::*;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Board {
    pub(crate) piece_of_color: [BitBoard; 2],
    pub(crate) piece_of_type: [BitBoard; 5],
    pub(crate) king_pos: [Square; 2],
    pub(crate) en_passant: Option<Square>,
    pub(crate) can_castle_king_side: [bool; 2],
    pub(crate) can_castle_queen_side: [bool; 2],
    pub(crate) side_to_play: Color,
    pub(crate) half_move_counter: u8,
}

impl Board {
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }
    pub fn empty(white_king: Square, black_king: Square) -> Board {
        Board {
            piece_of_color: [
                BitBoard::from_square(white_king),
                BitBoard::from_square(black_king),
            ],
            piece_of_type: [BitBoard::EMPTY; 5],
            king_pos: [white_king, black_king],
            en_passant: None,
            can_castle_king_side: [false, false],
            can_castle_queen_side: [false, false],
            side_to_play: WHITE,
            half_move_counter: 0,
        }
    }
    pub fn from_initial_position() -> Board {
        let rank_1_8: BitBoard = RANK_1 | RANK_8;
        Board {
            side_to_play: WHITE,
            can_castle_king_side: [true; 2],
            can_castle_queen_side: [true; 2],
            en_passant: None,
            piece_of_color: [RANK_1 | RANK_2, RANK_7 | RANK_8],
            piece_of_type: [
                RANK_2 | RANK_7,
                (FILE_B | FILE_G) & rank_1_8,
                (FILE_C | FILE_F) & rank_1_8,
                (FILE_A | FILE_H) & rank_1_8,
                FILE_D & rank_1_8,
            ],
            king_pos: [SQ_E1, SQ_E8],
            half_move_counter: 0,
        }
    }

    pub fn set_king_pos(&mut self, color: Color, square: Square) -> Result<(), ()> {
        if self.king_pos[color.opposite()] == square {
            return Err(());
        }
        let square_bb = BitBoard::from(square);
        for bb in self.piece_of_type {
            if bb * square_bb {
                return Err(());
            }
        }
        Ok(self.set_king_pos_fast(color, square, square_bb))
    }

    pub fn set_king_pos_fast(&mut self, color: Color, square: Square, square_bb: BitBoard) {
        let color: usize = color.into();
        self.piece_of_color[color] -= self.king_pos[color];
        self.king_pos[color] = square;
        self.piece_of_color[color] |= square_bb;
    }

    pub fn remove_piece_fast(&mut self, square_bb_inv: BitBoard) {
        self.piece_of_color[0] &= square_bb_inv;
        self.piece_of_color[1] &= square_bb_inv;
        self.piece_of_type[0] &= square_bb_inv;
        self.piece_of_type[1] &= square_bb_inv;
        self.piece_of_type[2] &= square_bb_inv;
        self.piece_of_type[3] &= square_bb_inv;
        self.piece_of_type[4] &= square_bb_inv;
    }

    pub fn move_piece(&mut self, origin: Square, dest: Square, piece: Piece, color: Color) -> bool {
        let piece_of_color = self.piece_of_color[color as usize];
        let origin_bb = BitBoard::from(origin);
        if !piece_of_color.intersects(origin_bb) {
            return false;
        }
        let dest_bb = BitBoard::from(dest);
        if piece_of_color.intersects(dest_bb) {
            return false;
        }

        if self.king_pos[color as usize] == origin {
            self.set_king_pos_fast(color, dest, dest_bb);
        } else {
            let from_bb_inv = !origin_bb;
            let dest_bb_inv = !dest_bb;
            self.remove_piece_fast(from_bb_inv);
            self.remove_piece_fast(dest_bb_inv);
            self.put_piece_fast(piece, color, dest_bb);
        }

        return true;
    }

    pub fn put_piece_safe(&mut self, piece: Piece, color: Color, square: Square) -> Result<(), ()> {
        if self.king_pos[0] == square || self.king_pos[1] == square {
            return Err(());
        }
        let square_bb = BitBoard::from(square);
        if piece == KING {
            return Ok(self.set_king_pos_fast(color, square, square_bb));
        }
        self.remove_piece_fast(!square_bb);
        Ok(self.put_piece_fast(piece, color, square_bb))
    }

    pub fn put_piece_fast(&mut self, piece: Piece, color: Color, square_bb: BitBoard) {
        self.piece_of_color[color as usize] |= square_bb;
        self.piece_of_type[piece as usize] |= square_bb;
    }

    pub fn piece_at(&self, square_bb: BitBoard) -> Option<Piece> {
        if square_bb * self.king_pos[0] || square_bb * self.king_pos[1] {
            return Some(KING);
        } else if self.piece_of_type[PAWN as usize] * square_bb {
            return Some(PAWN);
        } else if self.piece_of_type[KNIGHT as usize] * square_bb {
            return Some(KNIGHT);
        } else if self.piece_of_type[BISHOP as usize] * square_bb {
            return Some(BISHOP);
        } else if self.piece_of_type[ROOK as usize] * square_bb {
            return Some(ROOK);
        } else if self.piece_of_type[QUEEN as usize] * square_bb {
            return Some(QUEEN);
        } else {
            None
        }
    }
    pub fn color_at(&self, square_bb: BitBoard) -> Option<Color> {
        if self.piece_of_color[WHITE] * square_bb {
            Some(WHITE)
        } else if self.piece_of_color[BLACK] * square_bb {
            Some(BLACK)
        } else {
            None
        }
    }

    pub fn make_move(&mut self, m: Move) {
        self.en_passant = None;
        match m {
            Move::CastleKingSideWhite => self.castle_king_side_white_fast(),
            Move::CastleQueenSideWhite => self.castle_queen_side_white_fast(),
            Move::CastleKingSideBlack => self.castle_king_side_black_fast(),
            Move::CastleQueenSideBlack => self.castle_queen_side_black_fast(),
            Move::Promotion {
                origin,
                dest,
                piece,
            } => self.promote(origin, dest, piece),
            Move::NormalMove {
                origin,
                dest,
                piece,
            } => self.make_normal_move(origin, dest, piece),
            Move::EnPassant {
                origin,
                dest,
                capture,
            } => self.make_en_passant_move(origin, dest, capture),
            Move::NullMove => return,
        }
        self.side_to_play = self.side_to_play.opposite();
    }

    fn make_normal_move(&mut self, origin: Square, dest: Square, piece: Piece) {
        let origin_bb = BitBoard::from_square(origin);
        let dest_bb = BitBoard::from(dest);
        debug_assert_eq!(Some(piece), self.piece_at(origin_bb));

        let both_squares = origin_bb | dest_bb;
        self.piece_of_color[self.side_to_play] ^= both_squares;

        match self.piece_at(dest_bb) {
            None => {
                self.half_move_counter += 1;
            }
            Some(piece) => {
                self.piece_of_color[self.side_to_play.opposite()] ^= dest_bb;
                self.piece_of_type[piece as usize] ^= dest_bb;
                self.half_move_counter = 0;
            }
        }

        self.update_castling_rights(both_squares);
        if piece == KING {
            self.can_castle_king_side[self.side_to_play] = false;
            self.can_castle_queen_side[self.side_to_play] = false;
            self.king_pos[self.side_to_play] = dest;
        } else {
            self.piece_of_type[piece as usize] ^= both_squares;

            if piece == PAWN {
                if origin_bb * RANK_2 && dest_bb * RANK_4 {
                    self.en_passant = Some(origin.shift(UP));
                } else if origin_bb * RANK_7 && dest_bb * RANK_5 {
                    self.en_passant = Some(origin.shift(DOWN));
                }
                self.half_move_counter = 0;
            }
        }
    }

    #[inline]
    fn update_castling_rights(&mut self, both_squares: BitBoard) {
        const CORNERS: BitBoard = BitBoard(BB_A1.0 | BB_H1.0 | BB_A8.0 | BB_H8.0);
        if both_squares * CORNERS {
            if both_squares * BB_A1 {
                self.can_castle_queen_side[WHITE] = false;
            }
            if both_squares * BB_H1 {
                self.can_castle_king_side[WHITE] = false;
            }
            if both_squares * BB_A8 {
                self.can_castle_queen_side[BLACK] = false;
            }
            if both_squares * BB_H8 {
                self.can_castle_king_side[BLACK] = false;
            }
        }
    }

    fn make_en_passant_move(&mut self, origin: Square, dest: Square, capture: Square) {
        let origin_bb = BitBoard::from_square(origin);
        let dest_bb = BitBoard::from(dest);
        let both_squares = origin_bb | dest_bb;
        let capture_bb = BitBoard::from_square(capture);
        self.piece_of_color[self.side_to_play] ^= both_squares;
        self.piece_of_type[PAWN as usize] ^= both_squares | capture_bb;
        self.piece_of_color[self.side_to_play.opposite()] ^= capture_bb;
        self.half_move_counter = 0;
    }

    fn castle_king_side_white_fast(&mut self) {
        self.king_pos[WHITE] = SQ_G1;
        self.piece_of_color[WHITE] ^= BB_E1 | BB_F1 | BB_G1 | BB_H1;
        self.piece_of_type[ROOK as usize] ^= BB_F1 | BB_H1;
        self.can_castle_king_side[WHITE] = false;
        self.can_castle_queen_side[WHITE] = false;
        self.half_move_counter += 1;
    }

    fn castle_queen_side_white_fast(&mut self) {
        self.king_pos[WHITE] = SQ_C1;
        self.piece_of_color[WHITE] ^= BB_A1 | BB_C1 | BB_D1 | BB_E1;
        self.piece_of_type[ROOK as usize] ^= BB_D1 | BB_A1;
        self.can_castle_king_side[WHITE] = false;
        self.can_castle_queen_side[WHITE] = false;
        self.half_move_counter += 1;
    }

    fn castle_king_side_black_fast(&mut self) {
        self.king_pos[BLACK] = SQ_G8;
        self.piece_of_color[BLACK] ^= BB_E8 | BB_F8 | BB_G8 | BB_H8;
        self.piece_of_type[ROOK as usize] ^= BB_F8 | BB_H8;
        self.can_castle_king_side[BLACK] = false;
        self.can_castle_queen_side[BLACK] = false;
        self.half_move_counter += 1;
    }

    fn castle_queen_side_black_fast(&mut self) {
        self.king_pos[BLACK] = SQ_C8;
        self.piece_of_color[BLACK] ^= BB_A8 | BB_C8 | BB_D8 | BB_E8;
        self.piece_of_type[ROOK as usize] ^= BB_D8 | BB_A8;
        self.can_castle_king_side[BLACK] = false;
        self.can_castle_queen_side[BLACK] = false;
        self.half_move_counter += 1;
    }

    fn promote(&mut self, origin: Square, dest: Square, piece: Piece) {
        let origin_bb = BitBoard::from_square(origin);
        let dest_bb = BitBoard::from_square(dest);
        let dest_bbi = !dest_bb;
        let both_squares = origin_bb | dest_bb;
        self.piece_of_color[self.side_to_play] ^= both_squares;
        self.piece_of_type[PAWN as usize] ^= origin_bb;

        self.piece_of_color[self.side_to_play.opposite()] &= dest_bbi;
        self.piece_of_type[1] &= dest_bbi;
        self.piece_of_type[2] &= dest_bbi;
        self.piece_of_type[3] &= dest_bbi;
        self.piece_of_type[4] &= dest_bbi;

        self.piece_of_type[piece as usize] |= dest_bb;

        self.update_castling_rights(both_squares);
        self.half_move_counter = 0;
    }

    pub const fn piece_of_color(&self, color: Color) -> BitBoard {
        self.piece_of_color[color as usize]
    }

    pub const fn piece_of_opposite_color(&self, color: Color) -> BitBoard {
        self.piece_of_color[color.opposite() as usize]
    }

    pub const fn piece_of_type(&self, piece: Piece) -> BitBoard {
        self.piece_of_type[piece as usize]
    }

    #[inline]
    pub fn under_check(&self, color: Color) -> bool {
        let enemy_piece = self.piece_of_color[color.opposite()];
        let king = self.king_pos[color];
        let enemy_king = self.king_pos[color.opposite()];
        let any_piece = self.piece_of_color[0] | self.piece_of_color[1];
        let rook_or_queen = self.piece_of_type(ROOK) | self.piece_of_type(QUEEN);
        let bishop_or_queen = self.piece_of_type(BISHOP) | self.piece_of_type(QUEEN);
        if !(KNIGHT_ATTACKS[king] & enemy_piece & self.piece_of_type(KNIGHT)).empty() {
            return true;
        }
        if !(pawn_attacks(color, king) & enemy_piece & self.piece_of_type(PAWN)).empty() {
            return true;
        }
        if king_attacks(king) * enemy_king {
            return true;
        }
        if !(rook_attacks(king, any_piece) & enemy_piece & rook_or_queen).empty() {
            return true;
        }
        if !(bishop_attacks(king, any_piece) & enemy_piece & bishop_or_queen).empty() {
            return true;
        }
        false
    }

    pub fn check_consistency(&self) -> bool {
        if self.piece_of_color[0] * self.piece_of_color[1] {
            eprintln!("inconsistent colors");
            return false;
        }
        let mut accum = self.piece_of_type[0];
        for i in 1..5 {
            if accum * self.piece_of_type[i] {
                eprintln!("inconsistent pieces {}", i);
                return false;
            }
            accum |= self.piece_of_type[i]
        }
        return true;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..=7 as i32).rev() {
            f.write_fmt(format_args!("\n {}  ", y + 1))?;
            for x in 0..=7 as i32 {
                let bb: BitBoard = BitBoard::from_coords(x as u8, y as u8);
                if bb * self.piece_of_color[WHITE] {
                    if bb * self.piece_of_type[BISHOP as usize] {
                        f.write_str(" ♗")?;
                    } else if bb * self.piece_of_type[ROOK as usize] {
                        f.write_str(" ♖")?;
                    } else if bb * self.piece_of_type[KNIGHT as usize] {
                        f.write_str(" ♘")?;
                    } else if bb * self.piece_of_type[QUEEN as usize] {
                        f.write_str(" ♕")?;
                    } else if bb * self.piece_of_type[PAWN as usize] {
                        f.write_str(" ♙")?;
                    } else if bb * BitBoard::from_square(self.king_pos[WHITE]) {
                        f.write_str(" ♔")?;
                    } else {
                        f.write_str(" X")?;
                    }
                } else if bb * self.piece_of_color[BLACK] {
                    if bb * self.piece_of_type[BISHOP as usize] {
                        f.write_str(" ♝")?;
                    } else if bb * self.piece_of_type[ROOK as usize] {
                        f.write_str(" ♜")?;
                    } else if bb * self.piece_of_type[KNIGHT as usize] {
                        f.write_str(" ♞")?;
                    } else if bb * self.piece_of_type[QUEEN as usize] {
                        f.write_str(" ♛")?;
                    } else if bb * self.piece_of_type[PAWN as usize] {
                        f.write_str(" ♟")?;
                    } else if bb * BitBoard::from_square(self.king_pos[BLACK]) {
                        f.write_str(" ♚")?;
                    } else {
                        f.write_str(" x")?;
                    }
                } else {
                    f.write_str(" ◦")?;
                }
            }
        }
        return f.write_str("\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::square_constants::*;
    use crate::core::Color::*;
    use crate::core::Piece::*;

    #[test]
    pub fn test_display() {
        println!("{}\n", Board::from_initial_position());
    }

    #[test]
    pub fn under_check_test_1() {
        let board = Board::from_initial_position();
        assert_eq!(board.under_check(WHITE), false);
        assert_eq!(board.under_check(BLACK), false);
    }

    #[test]
    pub fn under_check_test_2() {
        let mut board = Board::empty(SQ_F2, SQ_E7);

        board.put_piece_safe(ROOK, WHITE, SQ_D2).unwrap();
        board.put_piece_safe(KNIGHT, WHITE, SQ_E2).unwrap();
        board.put_piece_safe(PAWN, WHITE, SQ_H2).unwrap();
        board.put_piece_safe(PAWN, WHITE, SQ_G3).unwrap();
        board.put_piece_safe(BISHOP, WHITE, SQ_F3).unwrap();
        board.put_piece_safe(KNIGHT, BLACK, SQ_B5).unwrap();
        board.put_piece_safe(ROOK, BLACK, SQ_C5).unwrap();
        board.put_piece_safe(PAWN, BLACK, SQ_F5).unwrap();
        board.put_piece_safe(PAWN, WHITE, SQ_C6).unwrap();
        board.put_piece_safe(BISHOP, BLACK, SQ_E6).unwrap();
        board.put_piece_safe(PAWN, BLACK, SQ_F7).unwrap();
        board.put_piece_safe(PAWN, BLACK, SQ_H7).unwrap();

        println!("{}", board);
        assert_eq!(board.under_check(WHITE), false);
        assert_eq!(board.under_check(BLACK), false);
    }
}
