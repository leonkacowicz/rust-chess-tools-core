use crate::core::bitboard::BitBoard;
use crate::core::bitboard_attacks::{king_attacks, knight_attacks, pawn_attacks};
use crate::core::bitboard_constants::*;
use crate::core::board::Board;
use crate::core::magic_bitboard::*;
use crate::core::r#move::Move;
use crate::core::square::Square;
use crate::core::square_constants::*;
use crate::core::*;

// Disable warnings

#[allow(unused_macros)]
// // The debug version
// #[cfg(debug_assertions)]
// macro_rules! if_debug {
//     ($args:expr) => {
//         $args
//     };
// }
//
// // Non-debug version
//
// #[cfg(not(debug_assertions))]
macro_rules! if_debug {
    ($args:expr) => {};
}

pub struct MoveGenerator<'a> {
    pub board: &'a Board,
    pub moves: Vec<Move>,
    magic_tables: &'a MagicTables,
    checkers: BitBoard,
    our_piece: BitBoard,
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
    pub fn new<'a>(board: &'a Board, magic_tables: &'a MagicTables) -> MoveGenerator<'a> {
        let our_piece = board.piece_of_color(board.side_to_play);
        let enemy_piece = board.piece_of_color(board.side_to_play.opposite());
        MoveGenerator {
            board: &board,
            moves: Vec::with_capacity(220),
            magic_tables,
            checkers: BitBoard::EMPTY,
            our_piece,
            enemy_piece,
            any_piece: our_piece | enemy_piece,
            block_mask: BitBoard::EMPTY,
            king: BitBoard::from_square(board.king_pos[board.side_to_play as usize]),
            king_sq: board.king_pos[board.side_to_play as usize],
            other_king: BitBoard::from_square(
                board.king_pos[board.side_to_play.opposite() as usize],
            ),
            pinned: BitBoard::EMPTY,
            rook_queen: board.piece_of_type(ROOK) | board.piece_of_type(QUEEN),
            bishop_queen: board.piece_of_type(BISHOP) | board.piece_of_type(QUEEN),
            us: board.side_to_play,
            evasive: false,
        }
    }

    pub fn generate(&mut self) -> &Vec<Move> {
        self.scan_board();
        if_debug!(println!("{}", self.block_mask));
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

    fn scan_board(&mut self) {
        let (rook_checkers, bishop_checkers) = self.update_checkers();
        self.update_block_mask(rook_checkers);
        self.update_block_mask(bishop_checkers);
        self.update_pins(
            self.magic_tables.rook_table.attacks_empty(self.king_sq),
            self.rook_queen,
        );
        self.update_pins(
            self.magic_tables.bishop_table.attacks_empty(self.king_sq),
            self.bishop_queen,
        );
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
        let rook_checkers =
            self.rook_attacks(king_sq, self.any_piece) & enemy_piece & self.rook_queen;
        let bishop_checkers =
            self.bishop_attacks(king_sq, self.any_piece) & enemy_piece & self.bishop_queen;
        self.checkers |= rook_checkers;
        self.checkers |= bishop_checkers;
        (rook_checkers, bishop_checkers)
    }

    #[inline]
    fn update_block_mask(&mut self, checkers: BitBoard) {
        let mut remaining = checkers;
        while !remaining.empty() {
            self.block_mask |= line_segment(self.king_sq, remaining.pop_lsb());
        }
    }

    #[inline]
    fn update_pins(&mut self, ray: BitBoard, attackers: BitBoard) {
        let mut remaining = self.enemy_piece & attackers & ray;
        while !remaining.empty() {
            let square = remaining.pop_lsb();
            let path = ray & line_segment(self.king_sq, square) & self.any_piece;
            if path.num_squares() == 1 {
                self.pinned |= path;
            }
        }
    }

    fn square_attacked(&self, sq: Square) -> bool {
        let enemy = self.enemy_piece;
        let any_piece = self.any_piece;
        !(knight_attacks(sq) & enemy & self.board.piece_of_type(KNIGHT)).empty()
            || !(pawn_attacks(self.us, sq) & enemy & self.board.piece_of_type(PAWN)).empty()
            || !(king_attacks(sq) & self.other_king).empty()
            || !(self.rook_attacks(sq, any_piece ^ self.king) & enemy & self.rook_queen).empty()
            || !(self.bishop_attacks(sq, any_piece ^ self.king) & enemy & self.bishop_queen).empty()
    }

    #[inline]
    fn generate_king_moves(&mut self) {
        let mut attacks = king_attacks(self.king_sq) & !self.our_piece;
        while !attacks.empty() {
            let sq = attacks.pop_lsb();
            // let bb = BitBoard::from_square(sq);
            if !self.square_attacked(sq) {
                // if self.enemy_piece * sq {
                //     self.moves.push(Move::capture(
                //         KING,
                //         self.king_sq,
                //         sq,
                //         self.board.piece_at(bb).unwrap(),
                //     ));
                // } else {
                self.moves.push(Move::new(KING, self.king_sq, sq));
                // }
            }
        }
    }

    #[inline]
    fn generate_non_king_moves(&mut self) {
        let mut remaining;
        let our_pieces = self.board.piece_of_color(self.us);
        remaining = self.board.piece_of_type(ROOK) & our_pieces;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            self.generate_slider_moves(sq, ROOK, self.rook_attacks(sq, self.any_piece));
        }

        remaining = self.board.piece_of_type(BISHOP) & our_pieces;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            self.generate_slider_moves(sq, BISHOP, self.bishop_attacks(sq, self.any_piece));
        }

        remaining = self.board.piece_of_type(QUEEN) & our_pieces;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            self.generate_slider_moves(sq, QUEEN, self.rook_attacks(sq, self.any_piece));
            self.generate_slider_moves(sq, QUEEN, self.bishop_attacks(sq, self.any_piece));
        }

        remaining = self.board.piece_of_type(PAWN) & our_pieces;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            let bb = BitBoard::from_square(sq);
            self.generate_pawn_moves(sq, bb);
        }

        remaining = self.board.piece_of_type(KNIGHT) & our_pieces & !self.pinned;
        while !remaining.empty() {
            let sq = remaining.pop_lsb();
            self.generate_knight_moves(sq);
        }
    }

    #[inline]
    fn rook_attacks(&self, origin: Square, occupancy: BitBoard) -> BitBoard {
        self.magic_tables.rook_table.attacks(origin, occupancy)
    }

    #[inline]
    fn bishop_attacks(&self, origin: Square, occupancy: BitBoard) -> BitBoard {
        self.magic_tables.bishop_table.attacks(origin, occupancy)
    }

    #[inline]
    fn generate_castles(&mut self) {
        if self.us == WHITE {
            self.generate_castles_move(BB_F1 | BB_G1, SQ_F1, SQ_G1, Move::CastleKingSideWhite);
            self.generate_castles_move(BB_D1 | BB_C1, SQ_D1, SQ_C1, Move::CastleQueenSideWhite);
        } else {
            self.generate_castles_move(BB_F8 | BB_G8, SQ_F8, SQ_G8, Move::CastleKingSideBlack);
            self.generate_castles_move(BB_D8 | BB_C8, SQ_D8, SQ_C8, Move::CastleQueenSideBlack);
        }
    }

    #[inline]
    fn generate_castles_move(&mut self, path: BitBoard, sq1: Square, sq2: Square, m: Move) {
        if self.board.can_castle_queen_side[self.us as usize] {
            if !self.any_piece.intersects(path) {
                if !self.square_attacked(sq1) && !self.square_attacked(sq2) {
                    self.moves.push(m);
                }
            }
        }
    }

    #[inline]
    fn generate_slider_moves(&mut self, origin_sq: Square, piece: Piece, attacks: BitBoard) {
        let mut attacks = attacks;
        attacks -= self.our_piece;
        if self.evasive {
            // remove squares that don't block the checker or capture it
            attacks &= self.checkers | self.block_mask;
        }
        if self.pinned * origin_sq {
            // if piece is pinned, it can only move away from or towards the king, but not any other direction
            attacks &= line(origin_sq, self.king_sq);
        }
        while !attacks.empty() {
            let dest = attacks.pop_lsb();
            // let dest_bb = BitBoard::from_square(dest);
            // if self.enemy_piece * dest_bb {
            //     self.moves.push(Move::capture(
            //         piece,
            //         origin_sq,
            //         dest,
            //         self.board.piece_at(dest_bb).unwrap(),
            //     ))
            // } else {
            self.moves.push(Move::new(piece, origin_sq, dest));
            // }
        }
    }
    fn generate_knight_moves(&mut self, origin: Square) {
        // this function assumes the knight is not pinned
        let mut attacks = knight_attacks(origin);

        attacks -= self.our_piece;
        if self.evasive {
            attacks &= self.checkers | self.block_mask;
        }

        while !attacks.empty() {
            let dest = attacks.pop_lsb();
            // let dest_bb = BitBoard::from_square(dest);
            // if self.enemy_piece * dest_bb {
            //     self.moves.push(Move::capture(
            //         KNIGHT,
            //         origin,
            //         dest,
            //         self.board.piece_at(dest_bb).unwrap(),
            //     ));
            // } else {
            self.moves.push(Move::new(KNIGHT, origin, dest));
            // }
        }
    }
    fn generate_pawn_moves(&mut self, origin: Square, origin_bb: BitBoard) {
        let fwd_dir = self.us.fwd_dir();
        let dest = origin.shift(fwd_dir);
        let fwd = BitBoard::from_square(dest);
        let promotion = (RANK_1 | RANK_8) * fwd;
        let first_move = !promotion && (RANK_2 | RANK_7) * origin_bb;
        let is_not_pinned = !self.pinned.intersects(origin_bb);
        if (!self.any_piece * fwd) && (is_not_pinned || line(origin, dest).intersects(self.king)) {
            // pawn can move forward if
            // 1. there's no piece in the destination square
            // 2. Either:
            //      - it is not pinned
            //      - it is pinned but it is moving towards or away from the king in a line (will continue pinned)
            if !self.evasive || self.block_mask * fwd {
                if promotion {
                    self.moves.push(Move::promote(origin, dest, QUEEN));
                    self.moves.push(Move::promote(origin, dest, ROOK));
                    self.moves.push(Move::promote(origin, dest, BISHOP));
                    self.moves.push(Move::promote(origin, dest, KNIGHT));
                } else {
                    self.moves.push(Move::new(PAWN, origin, dest));
                }
            }
            if first_move {
                let fwd2 = fwd.shift(fwd_dir);
                if !fwd2.intersects(self.any_piece)
                    && (!self.evasive || self.block_mask.intersects(fwd2))
                {
                    self.moves
                        .push(Move::new(PAWN, origin, dest.shift(fwd_dir)))
                }
            }
        }
        if origin.file() > 0 {
            let capture = dest.shift(LEFT);
            if is_not_pinned || line(origin, capture).intersects(self.king) {
                self.generate_pawn_captures(promotion, origin, capture);
            }
        }
        if origin.file() < 7 {
            let capture = dest.shift(RIGHT);
            if is_not_pinned || line(origin, capture).intersects(self.king) {
                self.generate_pawn_captures(promotion, origin, capture);
            }
        }
    }

    fn generate_pawn_captures(&mut self, is_promotion: bool, origin: Square, dest: Square) {
        let dest_bb = BitBoard::from_square(dest);
        if self.enemy_piece * dest_bb {
            if !self.evasive || self.checkers * dest_bb {
                if is_promotion {
                    self.moves.push(Move::promote(origin, dest, QUEEN));
                    self.moves.push(Move::promote(origin, dest, ROOK));
                    self.moves.push(Move::promote(origin, dest, BISHOP));
                    self.moves.push(Move::promote(origin, dest, KNIGHT));
                } else {
                    // let capture = self.board.piece_at(dest_bb).unwrap();
                    // self.moves.push(Move::capture(PAWN, origin, dest, capture));
                    self.moves.push(Move::new(PAWN, origin, dest));
                }
            }
        } else if let Some(en_passant) = self.board.en_passant {
            if en_passant == dest {
                let capture = Square::from_coords(dest.file(), origin.rank());
                if self.evasive || self.king_sq.rank() == origin.rank() {
                    let mut board = self.board.clone();
                    let en_passant = Move::en_passant(origin, dest, capture);
                    board.make_move(en_passant);
                    if !board.under_check(self.us, self.magic_tables) {
                        self.moves.push(en_passant);
                    }
                } else {
                    self.moves.push(Move::en_passant(origin, dest, capture));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::fen;
    use crate::core::fen::board_from_fen;
    use crate::core::magic_bitboard::*;
    use crate::core::move_generator::MoveGenerator;
    use crate::core::r#move::Move;
    use crate::core::square_constants::*;
    use crate::core::Piece::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MAGIC_TABLES: MagicTables = magic_tables();
    }

    fn performance_test(board: &Board, depth: i32, log: bool) -> usize {
        let mt = &MAGIC_TABLES;
        performance_test_rec(board, depth, &mt, log)
    }

    fn performance_test_rec(board: &Board, depth: i32, mt: &MagicTables, log: bool) -> usize {
        let mut generator = MoveGenerator::new(board, &mt);
        let moves = generator.generate();
        if depth == 1 && !log {
            return moves.len();
        }
        if depth == 0 {
            return 1;
        }
        let mut n = 0;
        for m in moves {
            let mut new_board = board.clone();
            new_board.make_move(*m);
            debug_assert!(
                new_board.check_consistency(),
                "consistency failed!\n{}, {:?}\n{}\n{}",
                m,
                m,
                board,
                new_board
            );
            debug_assert!(
                !new_board.under_check(board.side_to_play, &mt),
                "under check failed!\n{}, {:?}\n{}\n{}",
                m,
                m,
                board,
                new_board
            );
            let p = performance_test_rec(&new_board, depth - 1, mt, false);
            if log {
                // println!("{}: {} ({:?})", m, p, m);
                println!("{}: {}", m, p);
            }
            n += p;
        }
        n
    }

    #[test]
    pub fn perft() {
        let board = Board::from_initial_position();
        let mt: &MagicTables = &MAGIC_TABLES;
        let dummy = mt.bishop_table.attacks_empty(SQ_A1);
        let start = std::time::Instant::now();
        assert_eq!(performance_test(&board, 1, true), 20);
        assert_eq!(performance_test(&board, 2, true), 400);
        assert_eq!(performance_test(&board, 3, true), 8902);
        assert_eq!(performance_test(&board, 4, true), 197281);
        assert_eq!(performance_test(&board, 5, true), 4865609);
        assert_eq!(performance_test(&board, 6, true), 119060324);
        eprintln!("{}", start.elapsed().as_millis());
        println!("{}", dummy);
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
    pub fn test_en_passant_1() {
        let mut board = Board::from_initial_position();
        board.make_move(Move::new(PAWN, SQ_A2, SQ_A4));
        board.make_move(Move::new(PAWN, SQ_A7, SQ_A3));
        board.make_move(Move::new(PAWN, SQ_A4, SQ_A5));
        board.make_move(Move::new(PAWN, SQ_B7, SQ_B5));
        let mt = magic_tables();
        let mut mg = MoveGenerator::new(&board, &mt);
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
