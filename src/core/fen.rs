use crate::core::bitboard::BitBoard;
use crate::core::board::Board;
use crate::core::square::Square;
use crate::core::square_constants::*;
use crate::core::*;

pub const fn char_to_color(c: char) -> Color {
    if c.is_ascii_lowercase() {
        BLACK
    } else {
        WHITE
    }
}

pub const fn char_to_piece(ch: char) -> Result<Piece, ()> {
    match ch.to_ascii_lowercase() {
        'p' => Ok(PAWN),
        'n' => Ok(KNIGHT),
        'b' => Ok(BISHOP),
        'r' => Ok(ROOK),
        'q' => Ok(QUEEN),
        'k' => Ok(KING),
        _ => Err(()),
    }
}

pub fn side_to_play(ch: &str) -> Result<Color, ()> {
    match ch.to_ascii_lowercase().as_str() {
        "b" => Ok(BLACK),
        "w" => Ok(WHITE),
        _ => Err(()),
    }
}

pub fn castling_rights(s: &str) -> Result<[[bool; 2]; 2], ()> {
    let mut ans = [[false; 2]; 2];
    if s == "-" {
        return Ok(ans);
    }
    if s.chars().count() > 4 {
        return Err(());
    }
    for ch in s.chars() {
        match ch {
            'K' => ans[0][0] = true,
            'k' => ans[0][1] = true,
            'Q' => ans[1][0] = true,
            'q' => ans[1][1] = true,
            _ => return Err(()),
        }
    }
    Ok(ans)
}

pub fn en_passant_square(s: &String) -> Result<Option<Square>, ()> {
    if s == "-" {
        return Ok(None);
    }
    let chars = s.chars().collect::<Vec<char>>();
    if chars.len() != 2 {
        return Err(());
    }
    let file_char = chars[0];
    let rank_char = chars[1];

    if file_char < 'a' || file_char > 'h' || rank_char < '1' || rank_char > '8' {
        return Err(());
    }

    let file = file_char as u8 - 'a' as u8;
    let rank = rank_char as u8 - '1' as u8;

    Ok(Some(Square::from_coords(file, rank)))
}

pub fn to_string(board: &Board, full_move_counter: i32) -> String {
    let mut s: String = String::new();
    for rank in (0..8).rev() {
        let mut counter = 0;
        for file in 0..8 {
            let sq = Square::from_coords(file, rank);
            let bb = BitBoard::from_square(sq);
            match board.piece_at(bb) {
                None => counter += 1,
                Some(piece) => {
                    if counter > 0 {
                        s += counter.to_string().as_str();
                        counter = 0;
                    }
                    s += fen_char(piece, board.color_at(bb).expect("Board is inconsistent"))
                        .to_string()
                        .as_str();
                }
            }
        }
        if counter > 0 {
            s += counter.to_string().as_str();
        }
        if rank > 0 {
            s += "/";
        }
    }

    format!(
        "{} {} {} {} {} {}",
        s,
        if board.side_to_play == WHITE {
            "w"
        } else {
            "b"
        },
        castling_rights_to_string(board),
        match board.en_passant {
            None => "-".to_string(),
            Some(square) => square.to_string(),
        },
        board.half_move_counter,
        full_move_counter
    )
}

fn castling_rights_to_string(board: &Board) -> String {
    let mut s = String::with_capacity(4);
    let mut can_castle = false;
    if board.can_castle_king_side[WHITE as usize] {
        s += "K";
        can_castle = true;
    }
    if board.can_castle_queen_side[WHITE as usize] {
        s += "Q";
        can_castle = true;
    }
    if board.can_castle_king_side[BLACK as usize] {
        s += "k";
        can_castle = true;
    }
    if board.can_castle_queen_side[BLACK as usize] {
        s += "q";
        can_castle = true;
    }
    if !can_castle {
        s += "-"
    }
    s
}
fn fen_char(piece: Piece, color: Color) -> char {
    let ch = match piece {
        PAWN => 'P',
        BISHOP => 'B',
        KNIGHT => 'N',
        ROOK => 'R',
        QUEEN => 'Q',
        KING => 'K',
    };
    match color {
        WHITE => ch,
        BLACK => ch.to_ascii_lowercase(),
    }
}

pub struct FenComponents {
    pieces: String,
    side_to_move: String,
    castling: String,
    en_passant: String,
    half_move_clock: String,
}

impl FenComponents {
    pub fn parse(string: &str) -> Result<FenComponents, ()> {
        let s = string.split(" ").collect::<Vec<&str>>();
        if s.len() != 6 {
            Err(())
        } else {
            Ok(FenComponents {
                pieces: String::from(s[0]),
                side_to_move: String::from(s[1]),
                castling: String::from(s[2]),
                en_passant: String::from(s[3]),
                half_move_clock: String::from(s[4]),
                /* the last piece is the full move counter which is not used */
            })
        }
    }

    pub fn board(&self) -> Result<Board, ()> {
        let mut board = Board::empty(SQ_E1, SQ_E8);
        self.set_pieces(&mut board)?;
        board.side_to_play = side_to_play(self.side_to_move.to_ascii_lowercase().as_str())?;
        [board.can_castle_king_side, board.can_castle_queen_side] =
            castling_rights(self.castling.as_str())?;
        board.en_passant = en_passant_square(&self.en_passant)?;
        board.half_move_counter = self.half_move_clock.parse::<u8>().map_err(|_| ())?;
        Ok(board)
    }

    fn set_pieces(&self, board: &mut Board) -> Result<(), ()> {
        let mut rank = 7 as usize;
        let mut file = 0 as usize;
        for ch in self.pieces.chars() {
            if ch == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            if ch >= '1' && ch <= '8' {
                file += ch as usize - '0' as usize;
                continue;
            }
            if ch == '8' {
                continue;
            }
            let sq = Square::from_coords(file as u8, rank as u8);
            let color = char_to_color(ch);
            let piece = char_to_piece(ch)?;
            if piece == KING {
                board.set_king_pos(color, sq)?;
            } else {
                board.put_piece_safe(piece, color, sq)?;
            }
            file += 1;
        }
        Ok(())
    }
}

pub fn board_from_fen(string: &str) -> Option<Board> {
    FenComponents::parse(string).ok()?.board().ok()
}

#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::fen::*;
    use crate::core::r#move::Move;

    #[test]
    pub fn test() -> Result<(), ()> {
        let expected = "7k/3n4/p1p2p1Q/P7/2BP4/2P4P/4p1P1/6K1 b - - 0 1";
        let board = FenComponents::parse(expected)?.board()?;
        let actual = to_string(&board, 1);
        println!("{}", board);
        Ok(assert_eq!(actual, expected))
    }

    #[test]
    pub fn test_initial_position() {
        let board = Board::from_initial_position();
        let actual = to_string(&board, 1);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(actual, expected);
        let actual_board = board_from_fen(expected).unwrap();
        assert_eq!(board, actual_board);
    }

    #[test]
    pub fn test_initial_position_plus_one_move() {
        let mut board = Board::from_initial_position();
        board.make_move(Move::new(PAWN, SQ_E2, SQ_E4));
        let actual = to_string(&board, 1);
        let expected = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        assert_eq!(actual, expected);
        let actual_board = board_from_fen(expected).unwrap();
        assert_eq!(board, actual_board);
    }

    #[test]
    pub fn test_initial_position_plus_2_moves() {
        let mut board = Board::from_initial_position();
        board.make_move(Move::new(PAWN, SQ_E2, SQ_E4));
        board.make_move(Move::new(PAWN, SQ_C7, SQ_C5));
        assert_eq!(
            to_string(&board, 2),
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
        );
        assert_eq!(
            board,
            board_from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")
                .unwrap()
        );
    }

    #[test]
    pub fn test_initial_position_plus_3_moves() {
        let mut board = Board::from_initial_position();
        board.make_move(Move::new(PAWN, SQ_E2, SQ_E4));
        board.make_move(Move::new(PAWN, SQ_C7, SQ_C5));
        board.make_move(Move::new(KNIGHT, SQ_G1, SQ_F3));

        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        assert_eq!(to_string(&board, 2), fen);
        assert_eq!(board, board_from_fen(fen).unwrap());
    }
}
