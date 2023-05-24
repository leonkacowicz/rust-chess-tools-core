use crate::core::bitboard::BitBoard;
use crate::core::bitboard_attacks::*;
use crate::core::bitboard_constants::*;
use crate::core::board::Board;
use crate::core::magic_bitboard::*;
use crate::core::r#move::Move;
use crate::core::square::Square;
use crate::core::square_constants::*;
use crate::core::*;

// Disable warnings

// // The debug version
#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! if_debug {
    ($args:expr) => {
        $args
    };
}

#[allow(unused_macros)]
#[cfg(debug_assertions)]
macro_rules! if_ndebug {
    ($args:expr) => {};
}

#[allow(unused_macros)]
#[cfg(not(debug_assertions))]
macro_rules! if_debug {
    ($args:expr) => {};
}

#[allow(unused_macros)]
#[cfg(not(debug_assertions))]
macro_rules! if_ndebug {
    ($args:expr) => {
        $args
    };
}

pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub moves: Vec<Move>,
    checkers: BitBoard,
    our_piece_i: BitBoard,
    enemy_piece: BitBoard,
    any_piece: BitBoard,
    block_mask: BitBoard,
    king: BitBoard,
    king_sq: Square,
    other_king: BitBoard,
    pinned: BitBoard,
    rook_queen: BitBoard,
    bishop_queen: BitBoard,
    us: Color,
    evasive: bool,
}

impl MoveGenerator<'_> {
    pub fn new(board: &Board) -> MoveGenerator {
        let our_piece = board.piece_of_color(board.side_to_play);
        let enemy_piece = board.piece_of_color(board.side_to_play.opposite());
        MoveGenerator {
            board: &board,
            moves: Vec::with_capacity(220),
            checkers: BitBoard::EMPTY,
            our_piece_i: !our_piece,
            enemy_piece,
            any_piece: our_piece | enemy_piece,
            block_mask: BitBoard::EMPTY,
            king: BitBoard::from_square(board.king_pos[board.side_to_play]),
            king_sq: board.king_pos[board.side_to_play],
            other_king: BitBoard::from_square(board.king_pos[board.side_to_play.opposite()]),
            pinned: BitBoard::EMPTY,
            rook_queen: enemy_piece & (board.piece_of_type(ROOK) | board.piece_of_type(QUEEN)),
            bishop_queen: enemy_piece & (board.piece_of_type(BISHOP) | board.piece_of_type(QUEEN)),
            us: board.side_to_play,
            evasive: false,
        }
    }

    pub fn generate(&mut self) -> &Vec<Move> {
        self.scan_board();
        self.generate_king_moves();
        let num_checkers = self.checkers.num_squares();
        if num_checkers == 2 {
            return &self.moves;
        } else if num_checkers == 1 {
            self.evasive = true;
            self.generate_non_king_moves();
        } else {
            self.generate_non_king_moves();
            self.generate_castles();
        }
        &self.moves
    }

    #[inline]
    fn scan_board(&mut self) {
        let (rook_checkers, bishop_checkers) = self.update_checkers();
        self.update_block_mask(rook_checkers);
        self.update_block_mask(bishop_checkers);
        self.update_pins(rook_attacks_empty(self.king_sq), self.rook_queen);
        self.update_pins(bishop_attacks_empty(self.king_sq), self.bishop_queen);
    }

    #[inline]
    fn update_checkers(&mut self) -> (BitBoard, BitBoard) {
        let b = self.board;
        let us = b.side_to_play;
        let enemy_piece = self.enemy_piece;
        let king_sq = self.king_sq;

        self.checkers |= knight_attacks(king_sq) & enemy_piece & b.piece_of_type(KNIGHT);
        self.checkers |= pawn_attacks(us, king_sq) & enemy_piece & b.piece_of_type(PAWN);
        self.checkers |= king_attacks(king_sq) & self.other_king;
        let rook_checkers = rook_attacks(king_sq, self.any_piece) & self.rook_queen;
        let bishop_checkers = bishop_attacks(king_sq, self.any_piece) & self.bishop_queen;
        self.checkers |= rook_checkers;
        self.checkers |= bishop_checkers;
        (rook_checkers, bishop_checkers)
    }

    #[inline(always)]
    fn update_block_mask(&mut self, checkers: BitBoard) {
        let mut remaining = checkers;
        while !remaining.empty() {
            self.block_mask |= LINE_SEGMENT[self.king_sq][remaining.pop_lsb()];
        }
    }

    #[inline(always)]
    fn update_pins(&mut self, ray: BitBoard, attackers: BitBoard) {
        let mut remaining = self.enemy_piece & attackers & ray;
        while !remaining.empty() {
            let square = remaining.pop_lsb();
            let path = ray & LINE_SEGMENT[self.king_sq][square] & self.any_piece;
            if path.num_squares() == 1 {
                self.pinned |= path;
            }
        }
    }

    #[inline]
    fn square_attacked(&self, sq: Square) -> bool {
        let enemy = self.enemy_piece;
        let occupancy = self.any_piece ^ self.king;
        KNIGHT_ATTACKS[sq] * (enemy & self.board.piece_of_type(KNIGHT))
            || PAWN_ATTACKS[self.us][sq] * (enemy & self.board.piece_of_type(PAWN))
            || KING_ATTACKS[sq].intersects(self.other_king)
            || rook_attacks(sq, occupancy) * (self.rook_queen)
            || bishop_attacks(sq, occupancy) * (self.bishop_queen)
    }

    #[inline]
    fn generate_king_moves(&mut self) {
        let mut attacks = king_attacks(self.king_sq) & self.our_piece_i;
        while !attacks.empty() {
            let sq = attacks.pop_lsb();
            if !self.square_attacked(sq) {
                self.moves.push(Move::new(KING, self.king_sq, sq));
            }
        }
    }

    #[inline]
    fn generate_non_king_moves(&mut self) {
        let mut remaining;
        let our_pieces = self.board.piece_of_color[self.us];
        let occupancy = self.any_piece;

        remaining = self.board.piece_of_type(ROOK) & our_pieces;
        while !remaining.empty() {
            let origin = remaining.pop_lsb();
            self.generate_slider_moves(origin, ROOK, rook_attacks(origin, occupancy));
        }

        remaining = self.board.piece_of_type(BISHOP) & our_pieces;
        while !remaining.empty() {
            let origin = remaining.pop_lsb();
            self.generate_slider_moves(origin, BISHOP, bishop_attacks(origin, occupancy));
        }

        remaining = self.board.piece_of_type(QUEEN) & our_pieces;
        while !remaining.empty() {
            let origin = remaining.pop_lsb();
            self.generate_slider_moves(origin, QUEEN, rook_attacks(origin, occupancy));
            self.generate_slider_moves(origin, QUEEN, bishop_attacks(origin, occupancy));
        }

        remaining = self.board.piece_of_type(PAWN) & our_pieces;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            let bb = BitBoard::from_square(sq);
            self.generate_pawn_moves(sq, bb);
        }

        remaining = self.board.piece_of_type(KNIGHT) & our_pieces & !self.pinned;
        while !remaining.empty() {
            let origin = remaining.pop_lsb();
            self.generate_knight_moves(origin);
        }
    }

    #[inline]
    fn generate_castles(&mut self) {
        if self.us == WHITE {
            if self.board.can_castle_king_side[WHITE] {
                self.generate_castles_move(BB_F1 | BB_G1, SQ_F1, SQ_G1, Move::CastleKingSideWhite);
            }
            if self.board.can_castle_queen_side[WHITE] {
                let path = BB_D1 | BB_C1 | BB_B1;
                self.generate_castles_move(path, SQ_D1, SQ_C1, Move::CastleQueenSideWhite);
            }
        } else {
            if self.board.can_castle_king_side[BLACK] {
                self.generate_castles_move(BB_F8 | BB_G8, SQ_F8, SQ_G8, Move::CastleKingSideBlack);
            }
            if self.board.can_castle_queen_side[BLACK] {
                let path = BB_D8 | BB_C8 | BB_B8;
                self.generate_castles_move(path, SQ_D8, SQ_C8, Move::CastleQueenSideBlack);
            }
        }
    }

    #[inline]
    fn generate_castles_move(&mut self, path: BitBoard, sq1: Square, sq2: Square, m: Move) {
        if !self.any_piece.intersects(path) {
            if !self.square_attacked(sq1) && !self.square_attacked(sq2) {
                self.moves.push(m);
            }
        }
    }

    #[inline]
    fn generate_slider_moves(&mut self, origin: Square, piece: Piece, attacks: BitBoard) {
        let mut attacks = attacks;
        attacks &= self.our_piece_i;
        if self.evasive {
            // remove squares that don't block the checker or capture it
            attacks &= self.checkers | self.block_mask;
        }
        if self.pinned * origin {
            // if piece is pinned, it can only move away from or towards the king, but not any other direction
            attacks &= LINE[origin][self.king_sq];
        }
        while !attacks.empty() {
            self.moves.push(Move::new(piece, origin, attacks.pop_lsb()));
        }
    }
    fn generate_knight_moves(&mut self, origin: Square) {
        // this function assumes the knight is not pinned
        let mut attacks = knight_attacks(origin);

        attacks &= self.our_piece_i;
        if self.evasive {
            attacks &= self.checkers | self.block_mask;
        }

        while !attacks.empty() {
            let dest = attacks.pop_lsb();
            self.moves.push(Move::new(KNIGHT, origin, dest));
        }
    }
    fn generate_pawn_moves(&mut self, origin: Square, origin_bb: BitBoard) {
        let fwd_dir = self.us.fwd_dir();
        let dest = origin.shift(fwd_dir);
        let fwd = origin_bb.shift(fwd_dir);
        let is_promotion = (RANK_1 | RANK_8) * fwd;
        let first_move = !is_promotion && (RANK_2 | RANK_7) * origin_bb;
        let is_not_pinned = !(self.pinned * origin_bb);
        if (!self.any_piece * fwd) && (is_not_pinned || LINE[origin][dest] * (self.king)) {
            // pawn can move forward if
            // 1. there's no piece in the destination square
            // 2. Either:
            //      - it is not pinned
            //      - it is pinned but it is moving towards or away from the king in a line (will continue pinned)
            if !self.evasive || self.block_mask * fwd {
                self.add_pawn_moves(origin, dest, is_promotion);
            }
            if first_move {
                let fwd2 = fwd.shift(fwd_dir);
                if !(fwd2 * self.any_piece) && (!self.evasive || self.block_mask * fwd2) {
                    self.moves
                        .push(Move::new(PAWN, origin, dest.shift(fwd_dir)))
                }
            }
        }
        if origin_bb * FILE_A_I {
            let capture = dest.shift(LEFT);
            if is_not_pinned || LINE[origin][capture].intersects(self.king) {
                self.generate_pawn_captures(is_promotion, origin, capture);
            }
        }
        if origin_bb * FILE_H_I {
            let capture = dest.shift(RIGHT);
            if is_not_pinned || LINE[origin][capture].intersects(self.king) {
                self.generate_pawn_captures(is_promotion, origin, capture);
            }
        }
    }

    #[inline(always)]
    fn add_pawn_moves(&mut self, origin: Square, dest: Square, is_promotion: bool) {
        if is_promotion {
            self.moves.push(Move::promote(origin, dest, QUEEN));
            self.moves.push(Move::promote(origin, dest, ROOK));
            self.moves.push(Move::promote(origin, dest, BISHOP));
            self.moves.push(Move::promote(origin, dest, KNIGHT));
        } else {
            self.moves.push(Move::new(PAWN, origin, dest));
        }
    }

    fn generate_pawn_captures(&mut self, is_promotion: bool, origin: Square, dest: Square) {
        let dest_bb = BitBoard::from_square(dest);
        if self.enemy_piece * dest_bb {
            if !self.evasive || self.checkers * dest_bb {
                self.add_pawn_moves(origin, dest, is_promotion);
            }
        } else if let Some(en_passant) = self.board.en_passant {
            if en_passant == dest {
                let capture = Square(dest.file() | (origin.0 & 0xF8));
                if self.evasive || self.king_sq.rank() == origin.rank() {
                    let mut board = self.board.clone();
                    let en_passant = Move::en_passant(origin, dest, capture);
                    board.make_move(en_passant);
                    if !board.under_check(self.us) {
                        self.moves.push(en_passant);
                    }
                } else {
                    self.moves.push(Move::en_passant(origin, dest, capture));
                }
            }
        }
    }
}

#[allow(unused_mut)]
#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::fen::board_from_fen;
    use crate::core::move_generator::MoveGenerator;
    use crate::core::r#move::Move;
    use crate::core::square_constants::*;
    use crate::core::Piece::*;

    fn performance_test(board: &Board, depth: i32, log: bool) -> usize {
        performance_test_rec(board, depth, log)
    }

    fn performance_test_rec(board: &Board, depth: i32, log: bool) -> usize {
        let mut generator = MoveGenerator::new(board);
        generator.generate();
        let mut moves = generator.moves;
        if_debug!(if log {
            moves.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        });
        if depth == 1 && !log {
            return moves.len();
        }
        if depth == 0 {
            return 1;
        }
        let mut n = 0;
        for m in moves {
            let mut new_board = board.clone();
            new_board.make_move(m);
            // debug_assert!(
            //     new_board.check_consistency(),
            //     "consistency failed!\n{}, {:?}\n{}\n{}",
            //     m,
            //     m,
            //     board,
            //     new_board
            // );
            // debug_assert!(
            //     !new_board.under_check(board.side_to_play, &mt),
            //     "under check failed!\n{}, {:?}\n{}\n{}",
            //     m,
            //     m,
            //     board,
            //     new_board
            // );
            let p = performance_test_rec(&new_board, depth - 1, false);
            if log {
                // println!("{}: {} ({:?})", m, p, m);
                println!("{}: {}", m, p);
            }
            n += p;
        }
        n
    }

    #[test]
    pub fn perft_1() {
        let board = Board::from_initial_position();
        assert_eq!(performance_test(&board, 1, true), 20);
        assert_eq!(performance_test(&board, 2, true), 400);
        assert_eq!(performance_test(&board, 3, true), 8902);
        assert_eq!(performance_test(&board, 4, true), 197281);
        assert_eq!(performance_test(&board, 5, true), 4865609);
        assert_eq!(performance_test(&board, 6, true), 119060324);
    }

    #[test]
    pub fn perft_2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let board = board_from_fen(fen).unwrap();
        assert_eq!(performance_test(&board, 1, true), 48);
        assert_eq!(performance_test(&board, 2, true), 2039);
        assert_eq!(performance_test(&board, 3, true), 97862);
        assert_eq!(performance_test(&board, 4, true), 4085603);
        assert_eq!(performance_test(&board, 5, true), 193690690);
    }

    #[test]
    pub fn perft_3() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let board = board_from_fen(fen).unwrap();

        assert_eq!(performance_test(&board, 1, true), 14);
        assert_eq!(performance_test(&board, 2, true), 191);
        assert_eq!(performance_test(&board, 3, true), 2812);
        assert_eq!(performance_test(&board, 4, true), 43238);
        assert_eq!(performance_test(&board, 5, true), 674624);
    }

    #[test]
    pub fn perft_4() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let b = board_from_fen(fen).unwrap();

        assert_eq!(performance_test(&b, 1, true), 6);
        assert_eq!(performance_test(&b, 2, true), 264);
        assert_eq!(performance_test(&b, 3, true), 9467);
        assert_eq!(performance_test(&b, 4, true), 422333);
        assert_eq!(performance_test(&b, 5, true), 15833292);
    }

    #[test]
    pub fn perft_5() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let b = board_from_fen(fen).unwrap();
        assert_eq!(performance_test(&b, 1, true), 44);
        assert_eq!(performance_test(&b, 2, true), 1486);
        assert_eq!(performance_test(&b, 3, true), 62379);
        assert_eq!(performance_test(&b, 4, true), 2103487);
        assert_eq!(performance_test(&b, 5, true), 89941194);
    }

    // #[test]
    // pub fn perft_a2a4() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_A2, SQ_A4));
    //     let mt = MAGIC_TABLES();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         420
    //     );
    // }
    //
    // #[test]
    // pub fn perft_a2a4_b7b5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_A2, SQ_A4));
    //     board.make_move(Move::new(PAWN, SQ_B7, SQ_B5));
    //     let mt = MAGIC_TABLES();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         22
    //     );
    // }
    //
    // #[test]
    // pub fn perft_b1a3() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(KNIGHT, SQ_B1, SQ_A3));
    //     let mt = MAGIC_TABLES();
    //     assert_eq!(
    //         performance_test(&board, 3, &rook_magic, &bishop_magic, true),
    //         8885
    //     );
    // }
    // #[test]
    // pub fn perft_b1a3_b7b5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(KNIGHT, SQ_B1, SQ_A3));
    //     board.make_move(Move::new(PAWN, SQ_B7, SQ_B5));
    //     let mt = MAGIC_TABLES();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         420
    //     );
    // }
    //
    // #[test]
    // pub fn perft_b1a3_b7b5_a3b5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(KNIGHT, SQ_B1, SQ_A3));
    //     board.make_move(Move::new(PAWN, SQ_B7, SQ_B5));
    //     board.make_move(Move::capture(KNIGHT, SQ_A3, SQ_B5, PAWN));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         20
    //     );
    // }
    //
    // #[test]
    // pub fn perft_c2c3() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_C2, SQ_C3));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 3, &rook_magic, &bishop_magic, true),
    //         9272
    //     );
    // }
    //
    // #[test]
    // pub fn perft_c2c3_d7d6() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_C2, SQ_C3));
    //     board.make_move(Move::new(PAWN, SQ_D7, SQ_D6));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         545
    //     );
    // }
    //
    // #[test]
    // pub fn perft_c2c3_d7d6_d1a4() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_C2, SQ_C3));
    //     board.make_move(Move::new(PAWN, SQ_D7, SQ_D6));
    //     board.make_move(Move::new(QUEEN, SQ_D1, SQ_A4));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         6
    //     );
    // }
    //
    // #[test]
    // pub fn perft_c2c3_b8a6() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_C2, SQ_C3));
    //     board.make_move(Move::new(KNIGHT, SQ_B8, SQ_A6));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         418
    //     );
    // }
    //
    // #[test]
    // pub fn perft_c2c3_b8a6_d1a4() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_C2, SQ_C3));
    //     board.make_move(Move::new(KNIGHT, SQ_B8, SQ_A6));
    //     board.make_move(Move::new(QUEEN, SQ_D1, SQ_A4));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         18
    //     );
    // }
    //
    // #[test]
    // pub fn perft_b2b4() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_B2, SQ_B4));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 4, &rook_magic, &bishop_magic, true),
    //         216145
    //     );
    // }
    // #[test]
    // pub fn perft_b2b4_c7c5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_B2, SQ_B4));
    //     board.make_move(Move::new(PAWN, SQ_C7, SQ_C5));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 3, &rook_magic, &bishop_magic, true),
    //         11980
    //     );
    // }
    //
    // #[test]
    // pub fn perft_b2b4_c7c5_d2d3() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_B2, SQ_B4));
    //     board.make_move(Move::new(PAWN, SQ_C7, SQ_C5));
    //     board.make_move(Move::new(PAWN, SQ_D2, SQ_D3));
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         662
    //     );
    // }
    // #[test]
    // pub fn perft_b2b4_c7c5_d2d3_d8a5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_B2, SQ_B4));
    //     board.make_move(Move::new(PAWN, SQ_C7, SQ_C5));
    //     board.make_move(Move::new(PAWN, SQ_D2, SQ_D3));
    //     board.make_move(Move::new(QUEEN, SQ_D8, SQ_A5));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         28
    //     );
    // }
    //
    // #[test]
    // pub fn perft_f2f4() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_F2, SQ_F4));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 4, &rook_magic, &bishop_magic, true),
    //         198473
    //     );
    // }
    //
    // #[test]
    // pub fn perft_f2f4_e7e5() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_F2, SQ_F4));
    //     board.make_move(Move::new(PAWN, SQ_E7, SQ_E5));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 3, &rook_magic, &bishop_magic, true),
    //         14301
    //     );
    // }
    //
    // #[test]
    // pub fn perft_f2f4_e7e5_e1f2() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_F2, SQ_F4));
    //     board.make_move(Move::new(PAWN, SQ_E7, SQ_E5));
    //     board.make_move(Move::new(KING, SQ_E1, SQ_F2));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 2, &rook_magic, &bishop_magic, true),
    //         723
    //     );
    // }
    //
    // #[test]
    // pub fn perft_f2f4_e7e5_e1f2_d8f6() {
    //     let mut board = Board::from_initial_position();
    //     board.make_move(Move::new(PAWN, SQ_F2, SQ_F4));
    //     board.make_move(Move::new(PAWN, SQ_E7, SQ_E5));
    //     board.make_move(Move::new(KING, SQ_E1, SQ_F2));
    //     board.make_move(Move::new(QUEEN, SQ_D8, SQ_F6));
    //     println!("{}", board);
    //     let rook_magic = rook_magic_table();
    //     let bishop_magic = bishop_magic_table();
    //     assert_eq!(
    //         performance_test(&board, 1, &rook_magic, &bishop_magic, true),
    //         24
    //     );
    // }

    #[test]
    pub fn perft_2_c3b1() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(KNIGHT, SQ_C3, SQ_B1));

        println!("{}", board);
        assert_eq!(performance_test(&board, 2, true), 2038);
    }

    #[test]
    pub fn perft_2_c3b1_a6b5() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(KNIGHT, SQ_C3, SQ_B1));
        board.make_move(Move::new(BISHOP, SQ_A6, SQ_B5));
        println!("{}", board);
        assert_eq!(performance_test(&board, 1, true), 48);
    }

    #[test]
    pub fn perft_2_a1b1() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(ROOK, SQ_A1, SQ_B1));
        println!("{}", board);
        assert_eq!(performance_test(&board, 4, true), 3827454);
    }

    #[test]
    pub fn perft_2_a1b1_h3g2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(ROOK, SQ_A1, SQ_B1));
        board.make_move(Move::new(PAWN, SQ_H3, SQ_G2));
        println!("{}", board);
        assert_eq!(performance_test(&board, 3, true), 94098);
    }

    #[test]
    pub fn perft_2_a1b1_h3g2_a2a3() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(ROOK, SQ_A1, SQ_B1));
        board.make_move(Move::new(PAWN, SQ_H3, SQ_G2));
        board.make_move(Move::new(PAWN, SQ_A2, SQ_A3));
        println!("{}", board);
        assert_eq!(performance_test(&board, 2, true), 2201);
    }

    #[test]
    pub fn perft_2_a1b1_h3g2_a2a3_() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let mut board = board_from_fen(fen).unwrap();
        board.make_move(Move::new(ROOK, SQ_A1, SQ_B1));
        board.make_move(Move::new(PAWN, SQ_H3, SQ_G2));
        board.make_move(Move::new(PAWN, SQ_A2, SQ_A3));
        board.make_move(Move::promote(SQ_G2, SQ_H1, BISHOP));
        println!("{}", board);
        assert_eq!(performance_test(&board, 1, true), 45);
    }

    #[test]
    pub fn test_en_passant_1() {
        let mut board = Board::from_initial_position();
        board.make_move(Move::new(PAWN, SQ_A2, SQ_A4));
        board.make_move(Move::new(PAWN, SQ_A7, SQ_A3));
        board.make_move(Move::new(PAWN, SQ_A4, SQ_A5));
        board.make_move(Move::new(PAWN, SQ_B7, SQ_B5));
        let mut mg = MoveGenerator::new(&board);
        let moves = mg.generate();
        for m in moves {
            match *m {
                Move::EnPassant {
                    origin: _,
                    dest,
                    capture,
                } => assert_ne!(dest, capture),
                _ => {}
            }
        }
    }
}
