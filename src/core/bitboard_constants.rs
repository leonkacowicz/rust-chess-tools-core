use crate::core::bitboard::BitBoard;
use crate::core::square::{same_diag, Square};
use crate::core::{DOWN_RIGHT, UP_RIGHT};

pub const BB_A1: BitBoard = BitBoard::from_coords(0, 0);
pub const BB_B1: BitBoard = BitBoard::from_coords(1, 0);
pub const BB_C1: BitBoard = BitBoard::from_coords(2, 0);
pub const BB_D1: BitBoard = BitBoard::from_coords(3, 0);
pub const BB_E1: BitBoard = BitBoard::from_coords(4, 0);
pub const BB_F1: BitBoard = BitBoard::from_coords(5, 0);
pub const BB_G1: BitBoard = BitBoard::from_coords(6, 0);
pub const BB_H1: BitBoard = BitBoard::from_coords(7, 0);

pub const BB_A2: BitBoard = BitBoard::from_coords(0, 1);
pub const BB_B2: BitBoard = BitBoard::from_coords(1, 1);
pub const BB_C2: BitBoard = BitBoard::from_coords(2, 1);
pub const BB_D2: BitBoard = BitBoard::from_coords(3, 1);
pub const BB_E2: BitBoard = BitBoard::from_coords(4, 1);
pub const BB_F2: BitBoard = BitBoard::from_coords(5, 1);
pub const BB_G2: BitBoard = BitBoard::from_coords(6, 1);
pub const BB_H2: BitBoard = BitBoard::from_coords(7, 1);

pub const BB_A3: BitBoard = BitBoard::from_coords(0, 2);
pub const BB_B3: BitBoard = BitBoard::from_coords(1, 2);
pub const BB_C3: BitBoard = BitBoard::from_coords(2, 2);
pub const BB_D3: BitBoard = BitBoard::from_coords(3, 2);
pub const BB_E3: BitBoard = BitBoard::from_coords(4, 2);
pub const BB_F3: BitBoard = BitBoard::from_coords(5, 2);
pub const BB_G3: BitBoard = BitBoard::from_coords(6, 2);
pub const BB_H3: BitBoard = BitBoard::from_coords(7, 2);

pub const BB_A4: BitBoard = BitBoard::from_coords(0, 3);
pub const BB_B4: BitBoard = BitBoard::from_coords(1, 3);
pub const BB_C4: BitBoard = BitBoard::from_coords(2, 3);
pub const BB_D4: BitBoard = BitBoard::from_coords(3, 3);
pub const BB_E4: BitBoard = BitBoard::from_coords(4, 3);
pub const BB_F4: BitBoard = BitBoard::from_coords(5, 3);
pub const BB_G4: BitBoard = BitBoard::from_coords(6, 3);
pub const BB_H4: BitBoard = BitBoard::from_coords(7, 3);

pub const BB_A5: BitBoard = BitBoard::from_coords(0, 4);
pub const BB_B5: BitBoard = BitBoard::from_coords(1, 4);
pub const BB_C5: BitBoard = BitBoard::from_coords(2, 4);
pub const BB_D5: BitBoard = BitBoard::from_coords(3, 4);
pub const BB_E5: BitBoard = BitBoard::from_coords(4, 4);
pub const BB_F5: BitBoard = BitBoard::from_coords(5, 4);
pub const BB_G5: BitBoard = BitBoard::from_coords(6, 4);
pub const BB_H5: BitBoard = BitBoard::from_coords(7, 4);

pub const BB_A6: BitBoard = BitBoard::from_coords(0, 5);
pub const BB_B6: BitBoard = BitBoard::from_coords(1, 5);
pub const BB_C6: BitBoard = BitBoard::from_coords(2, 5);
pub const BB_D6: BitBoard = BitBoard::from_coords(3, 5);
pub const BB_E6: BitBoard = BitBoard::from_coords(4, 5);
pub const BB_F6: BitBoard = BitBoard::from_coords(5, 5);
pub const BB_G6: BitBoard = BitBoard::from_coords(6, 5);
pub const BB_H6: BitBoard = BitBoard::from_coords(7, 5);

pub const BB_A7: BitBoard = BitBoard::from_coords(0, 6);
pub const BB_B7: BitBoard = BitBoard::from_coords(1, 6);
pub const BB_C7: BitBoard = BitBoard::from_coords(2, 6);
pub const BB_D7: BitBoard = BitBoard::from_coords(3, 6);
pub const BB_E7: BitBoard = BitBoard::from_coords(4, 6);
pub const BB_F7: BitBoard = BitBoard::from_coords(5, 6);
pub const BB_G7: BitBoard = BitBoard::from_coords(6, 6);
pub const BB_H7: BitBoard = BitBoard::from_coords(7, 6);

pub const BB_A8: BitBoard = BitBoard::from_coords(0, 7);
pub const BB_B8: BitBoard = BitBoard::from_coords(1, 7);
pub const BB_C8: BitBoard = BitBoard::from_coords(2, 7);
pub const BB_D8: BitBoard = BitBoard::from_coords(3, 7);
pub const BB_E8: BitBoard = BitBoard::from_coords(4, 7);
pub const BB_F8: BitBoard = BitBoard::from_coords(5, 7);
pub const BB_G8: BitBoard = BitBoard::from_coords(6, 7);
pub const BB_H8: BitBoard = BitBoard::from_coords(7, 7);

pub const BB_A1_I: BitBoard = BitBoard(!BB_A1.0);
pub const BB_B1_I: BitBoard = BitBoard(!BB_B1.0);
pub const BB_C1_I: BitBoard = BitBoard(!BB_C1.0);
pub const BB_D1_I: BitBoard = BitBoard(!BB_D1.0);
pub const BB_E1_I: BitBoard = BitBoard(!BB_E1.0);
pub const BB_F1_I: BitBoard = BitBoard(!BB_F1.0);
pub const BB_G1_I: BitBoard = BitBoard(!BB_G1.0);
pub const BB_H1_I: BitBoard = BitBoard(!BB_H1.0);

pub const BB_A2_I: BitBoard = BitBoard(!BB_A2.0);
pub const BB_B2_I: BitBoard = BitBoard(!BB_B2.0);
pub const BB_C2_I: BitBoard = BitBoard(!BB_C2.0);
pub const BB_D2_I: BitBoard = BitBoard(!BB_D2.0);
pub const BB_E2_I: BitBoard = BitBoard(!BB_E2.0);
pub const BB_F2_I: BitBoard = BitBoard(!BB_F2.0);
pub const BB_G2_I: BitBoard = BitBoard(!BB_G2.0);
pub const BB_H2_I: BitBoard = BitBoard(!BB_H2.0);

pub const BB_A3_I: BitBoard = BitBoard(!BB_A3.0);
pub const BB_B3_I: BitBoard = BitBoard(!BB_B3.0);
pub const BB_C3_I: BitBoard = BitBoard(!BB_C3.0);
pub const BB_D3_I: BitBoard = BitBoard(!BB_D3.0);
pub const BB_E3_I: BitBoard = BitBoard(!BB_E3.0);
pub const BB_F3_I: BitBoard = BitBoard(!BB_F3.0);
pub const BB_G3_I: BitBoard = BitBoard(!BB_G3.0);
pub const BB_H3_I: BitBoard = BitBoard(!BB_H3.0);

pub const BB_A4_I: BitBoard = BitBoard(!BB_A4.0);
pub const BB_B4_I: BitBoard = BitBoard(!BB_B4.0);
pub const BB_C4_I: BitBoard = BitBoard(!BB_C4.0);
pub const BB_D4_I: BitBoard = BitBoard(!BB_D4.0);
pub const BB_E4_I: BitBoard = BitBoard(!BB_E4.0);
pub const BB_F4_I: BitBoard = BitBoard(!BB_F4.0);
pub const BB_G4_I: BitBoard = BitBoard(!BB_G4.0);
pub const BB_H4_I: BitBoard = BitBoard(!BB_H4.0);

pub const BB_A5_I: BitBoard = BitBoard(!BB_A5.0);
pub const BB_B5_I: BitBoard = BitBoard(!BB_B5.0);
pub const BB_C5_I: BitBoard = BitBoard(!BB_C5.0);
pub const BB_D5_I: BitBoard = BitBoard(!BB_D5.0);
pub const BB_E5_I: BitBoard = BitBoard(!BB_E5.0);
pub const BB_F5_I: BitBoard = BitBoard(!BB_F5.0);
pub const BB_G5_I: BitBoard = BitBoard(!BB_G5.0);
pub const BB_H5_I: BitBoard = BitBoard(!BB_H5.0);

pub const BB_A6_I: BitBoard = BitBoard(!BB_A6.0);
pub const BB_B6_I: BitBoard = BitBoard(!BB_B6.0);
pub const BB_C6_I: BitBoard = BitBoard(!BB_C6.0);
pub const BB_D6_I: BitBoard = BitBoard(!BB_D6.0);
pub const BB_E6_I: BitBoard = BitBoard(!BB_E6.0);
pub const BB_F6_I: BitBoard = BitBoard(!BB_F6.0);
pub const BB_G6_I: BitBoard = BitBoard(!BB_G6.0);
pub const BB_H6_I: BitBoard = BitBoard(!BB_H6.0);

pub const BB_A7_I: BitBoard = BitBoard(!BB_A7.0);
pub const BB_B7_I: BitBoard = BitBoard(!BB_B7.0);
pub const BB_C7_I: BitBoard = BitBoard(!BB_C7.0);
pub const BB_D7_I: BitBoard = BitBoard(!BB_D7.0);
pub const BB_E7_I: BitBoard = BitBoard(!BB_E7.0);
pub const BB_F7_I: BitBoard = BitBoard(!BB_F7.0);
pub const BB_G7_I: BitBoard = BitBoard(!BB_G7.0);
pub const BB_H7_I: BitBoard = BitBoard(!BB_H7.0);

pub const BB_A8_I: BitBoard = BitBoard(!BB_A8.0);
pub const BB_B8_I: BitBoard = BitBoard(!BB_B8.0);
pub const BB_C8_I: BitBoard = BitBoard(!BB_C8.0);
pub const BB_D8_I: BitBoard = BitBoard(!BB_D8.0);
pub const BB_E8_I: BitBoard = BitBoard(!BB_E8.0);
pub const BB_F8_I: BitBoard = BitBoard(!BB_F8.0);
pub const BB_G8_I: BitBoard = BitBoard(!BB_G8.0);
pub const BB_H8_I: BitBoard = BitBoard(!BB_H8.0);

pub const FILE_A: BitBoard = BitBoard(0x0101010101010101);
pub const FILE_B: BitBoard = BitBoard(0x0202020202020202);
pub const FILE_C: BitBoard = BitBoard(0x0404040404040404);
pub const FILE_D: BitBoard = BitBoard(0x0808080808080808);
pub const FILE_E: BitBoard = BitBoard(0x1010101010101010);
pub const FILE_F: BitBoard = BitBoard(0x2020202020202020);
pub const FILE_G: BitBoard = BitBoard(0x4040404040404040);
pub const FILE_H: BitBoard = BitBoard(0x8080808080808080);
pub const RANK_1: BitBoard = BitBoard(0x00000000000000FF);
pub const RANK_2: BitBoard = BitBoard(0x000000000000FF00);
pub const RANK_3: BitBoard = BitBoard(0x0000000000FF0000);
pub const RANK_4: BitBoard = BitBoard(0x00000000FF000000);
pub const RANK_5: BitBoard = BitBoard(0x000000FF00000000);
pub const RANK_6: BitBoard = BitBoard(0x0000FF0000000000);
pub const RANK_7: BitBoard = BitBoard(0x00FF000000000000);
pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);

pub const FILE_A_I: BitBoard = BitBoard(!FILE_A.0);
pub const FILE_B_I: BitBoard = BitBoard(!FILE_B.0);
pub const FILE_C_I: BitBoard = BitBoard(!FILE_C.0);
pub const FILE_D_I: BitBoard = BitBoard(!FILE_D.0);
pub const FILE_E_I: BitBoard = BitBoard(!FILE_E.0);
pub const FILE_F_I: BitBoard = BitBoard(!FILE_F.0);
pub const FILE_G_I: BitBoard = BitBoard(!FILE_G.0);
pub const FILE_H_I: BitBoard = BitBoard(!FILE_H.0);
pub const RANK_1_I: BitBoard = BitBoard(!RANK_1.0);
pub const RANK_2_I: BitBoard = BitBoard(!RANK_2.0);
pub const RANK_3_I: BitBoard = BitBoard(!RANK_3.0);
pub const RANK_4_I: BitBoard = BitBoard(!RANK_4.0);
pub const RANK_5_I: BitBoard = BitBoard(!RANK_5.0);
pub const RANK_6_I: BitBoard = BitBoard(!RANK_6.0);
pub const RANK_7_I: BitBoard = BitBoard(!RANK_7.0);
pub const RANK_8_I: BitBoard = BitBoard(!RANK_8.0);

pub const FILE_A_I_RANK_1_I: BitBoard = BitBoard(FILE_A_I.0 & RANK_1_I.0);
pub const FILE_A_I_RANK_8_I: BitBoard = BitBoard(FILE_A_I.0 & RANK_8_I.0);
pub const FILE_H_I_RANK_1_I: BitBoard = BitBoard(FILE_H_I.0 & RANK_1_I.0);
pub const FILE_H_I_RANK_8_I: BitBoard = BitBoard(FILE_H_I.0 & RANK_8_I.0);
pub const FILE_A_I_RANK_8_I_RANK_7_I: BitBoard = BitBoard(FILE_A_I_RANK_8_I.0 & RANK_7_I.0);
pub const FILE_A_I_RANK_1_I_RANK_2_I: BitBoard = BitBoard(FILE_A_I_RANK_1_I.0 & RANK_2_I.0);
pub const FILE_H_I_RANK_8_I_RANK_7_I: BitBoard = BitBoard(FILE_H_I_RANK_8_I.0 & RANK_7_I.0);
pub const FILE_H_I_RANK_1_I_RANK_2_I: BitBoard = BitBoard(FILE_H_I_RANK_1_I.0 & RANK_2_I.0);
pub const FILE_A_I_RANK_8_I_FILE_B_I: BitBoard = BitBoard(FILE_A_I_RANK_8_I.0 & FILE_B_I.0);
pub const FILE_A_I_RANK_1_I_FILE_B_I: BitBoard = BitBoard(FILE_A_I_RANK_1_I.0 & FILE_B_I.0);
pub const FILE_H_I_RANK_8_I_FILE_G_I: BitBoard = BitBoard(FILE_H_I_RANK_8_I.0 & FILE_G_I.0);
pub const FILE_H_I_RANK_1_I_FILE_G_I: BitBoard = BitBoard(FILE_H_I_RANK_1_I.0 & FILE_G_I.0);

pub const FILE: [BitBoard; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];
pub const RANK: [BitBoard; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

pub const BOARD_INTERIOR: BitBoard = BitBoard(FILE_A_I.0 & FILE_H_I.0 & RANK_1_I.0 & RANK_8_I.0);

pub const DARK_SQUARES: BitBoard =
    BitBoard(FILE_A.0 ^ FILE_C.0 ^ FILE_E.0 ^ FILE_G.0 ^ RANK_2.0 ^ RANK_4.0 ^ RANK_6.0 ^ RANK_8.0);
pub const LIGHT_SQUARES: BitBoard = BitBoard(!DARK_SQUARES.0);

const fn diagonal(sq1: Square, sq2: Square) -> BitBoard {
    if sq1.file() > sq2.file() {
        return diagonal(sq2, sq1);
    }
    let (direction, range_right, range_left) = if sq2.rank() > sq1.rank() {
        (UP_RIGHT, FILE_H_I_RANK_8_I, FILE_A_I_RANK_1_I)
    } else {
        (DOWN_RIGHT, FILE_H_I_RANK_1_I, FILE_A_I_RANK_8_I)
    };

    let sq1_bb = BitBoard::from_square(sq1);
    let mut ans = sq1_bb;
    let mut bb = sq1_bb;
    while bb.intersects(range_right) {
        bb = bb.shift(direction);
        ans.0 |= bb.0;
    }

    bb = sq1_bb;
    while bb.intersects(range_left) {
        bb = bb.shift(-direction);
        ans.0 |= bb.0;
    }

    ans
}

const fn diagonal_segment(sq1: Square, sq2: Square) -> BitBoard {
    if sq1.file() > sq2.file() {
        return diagonal_segment(sq2, sq1);
    }
    let direction = if sq2.rank() > sq1.rank() {
        UP_RIGHT
    } else {
        DOWN_RIGHT
    };

    let sq1_bb = BitBoard::from_square(sq1);
    let sq2_bb = BitBoard::from_square(sq2);
    let mut ans = BitBoard::EMPTY;
    let mut bb = sq1_bb.shift(direction);
    while !bb.intersects(sq2_bb) {
        ans.0 |= bb.0;
        bb = bb.shift(direction);
    }

    ans
}

const fn calc_diagonals() -> [[BitBoard; 64]; 64] {
    let mut ans = [[BitBoard::EMPTY; 64]; 64];
    let mut sq1i = 0;
    while sq1i < 64 {
        let sq1 = Square(sq1i);
        let mut sq2i = 0;
        while sq2i < 64 {
            let sq2 = Square(sq2i);
            if same_diag(sq1, sq2) {
                ans[sq1i as usize][sq2i as usize] = diagonal(sq1, sq2);
                ans[sq2i as usize][sq1i as usize] = ans[sq1i as usize][sq2i as usize]
            }
            sq2i += 1;
        }
        sq1i += 1;
    }
    ans
}

pub const DIAGONAL: [[BitBoard; 64]; 64] = calc_diagonals();

const fn calc_diagonal_segments() -> [[BitBoard; 64]; 64] {
    let mut ans = [[BitBoard::EMPTY; 64]; 64];
    let mut sq1i = 0;
    while sq1i < 64 {
        let sq1 = Square(sq1i);
        let mut sq2i = 0;
        while sq2i < 64 {
            let sq2 = Square(sq2i);
            if sq1i != sq2i && same_diag(sq1, sq2) {
                ans[sq1i as usize][sq2i as usize] = diagonal_segment(sq1, sq2);
                ans[sq2i as usize][sq1i as usize] = ans[sq1i as usize][sq2i as usize]
            }
            sq2i += 1;
        }
        sq1i += 1;
    }
    ans
}

pub const DIAGONAL_SEGMENT: [[BitBoard; 64]; 64] = calc_diagonal_segments();

const fn calc_lines() -> [[BitBoard; 64]; 64] {
    let mut ans = [[BitBoard::EMPTY; 64]; 64];
    let mut sq1i = 0;
    while sq1i < 64 {
        let mut sq2i = 0;
        while sq2i < sq1i {
            let sq1 = Square(sq1i as u8);
            let sq2 = Square(sq2i as u8);

            let rank_sq1 = sq1.rank();
            if rank_sq1 == sq2.rank() {
                ans[sq1i][sq2i] = RANK[rank_sq1 as usize];
            } else {
                let file_sq1 = sq1.file();
                if file_sq1 == sq2.file() {
                    ans[sq1i][sq2i] = FILE[file_sq1 as usize];
                } else {
                    if same_diag(sq1, sq2) {
                        ans[sq1i][sq2i] = diagonal(sq1, sq2);
                    }
                }
            }
            ans[sq2i][sq1i] = ans[sq1i][sq2i];

            sq2i += 1;
        }
        sq1i += 1;
    }
    ans
}
pub const LINE: [[BitBoard; 64]; 64] = calc_lines();
pub const fn line(sq1: Square, sq2: Square) -> BitBoard {
    LINE[sq1.0 as usize][sq2.0 as usize]
}

const fn calc_line_segments() -> [[BitBoard; 64]; 64] {
    let mut ans = [[BitBoard::EMPTY; 64]; 64];
    let mut level = 0;
    while level < 8 {
        let mut start = 0;
        while start < 7 {
            let mut finish = start + 2;
            while finish < 8 {
                let h_start = Square::from_coords(start, level).0 as usize;
                let h_finish = Square::from_coords(finish, level).0 as usize;
                let h_previous = Square::from_coords(finish - 1, level).0 as usize;
                let h_finish_bb = BitBoard::from_coords(finish - 1, level).0;

                ans[h_start][h_finish].0 = ans[h_start][h_previous].0 | h_finish_bb;
                ans[h_finish][h_start] = ans[h_start][h_finish];

                let v_start = Square::from_coords(level, start).0 as usize;
                let v_finish = Square::from_coords(level, finish).0 as usize;
                let v_previous = Square::from_coords(level, finish - 1).0 as usize;
                let v_finish_bb = BitBoard::from_coords(level, finish - 1).0;

                ans[v_start][v_finish].0 = ans[v_start][v_previous].0 | v_finish_bb;
                ans[v_finish][v_start] = ans[v_start][v_finish];

                finish += 1
            }
            start += 1;
        }
        level += 1;
    }

    let mut sq1i = 0;
    while sq1i < 64 {
        let sq1 = Square(sq1i);
        let mut sq2i = 0;
        while sq2i < 64 {
            let sq2 = Square(sq2i);
            if sq1i != sq2i && same_diag(sq1, sq2) {
                ans[sq1i as usize][sq2i as usize] = diagonal_segment(sq1, sq2);
                ans[sq2i as usize][sq1i as usize] = ans[sq1i as usize][sq2i as usize]
            }
            sq2i += 1;
        }
        sq1i += 1;
    }
    ans
}
pub const LINE_SEGMENT: [[BitBoard; 64]; 64] = calc_line_segments();
pub const fn line_segment(sq1: Square, sq2: Square) -> BitBoard {
    LINE_SEGMENT[sq1.0 as usize][sq2.0 as usize]
}

#[cfg(test)]
mod tests {
    use crate::core::bitboard_constants::*;
    use crate::core::square_constants::*;

    #[test]
    pub fn test_diagonal_segment_1() {
        let segment = diagonal_segment(SQ_A1, SQ_D4);
        assert_eq!(segment, BB_B2 | BB_C3)
    }

    #[test]
    pub fn test_diagonal_segment_2() {
        let segment = diagonal_segment(SQ_A4, SQ_D1);
        assert_eq!(segment, BB_B3 | BB_C2)
    }

    #[test]
    pub fn test_diagonal_segment_3() {
        let segment = diagonal_segment(SQ_D4, SQ_A1);
        assert_eq!(segment, BB_B2 | BB_C3)
    }

    #[test]
    pub fn test_diagonal_segment_4() {
        let segment = diagonal_segment(SQ_A2, SQ_E6);
        assert_eq!(segment, BB_B3 | BB_C4 | BB_D5)
    }

    #[test]
    pub fn test_diagonal_segment_5() {
        assert_eq!(line_segment(SQ_A4, SQ_E8), BB_B5 | BB_C6 | BB_D7);
        assert_eq!(line_segment(SQ_E8, SQ_A4), BB_B5 | BB_C6 | BB_D7);
    }

    #[test]
    pub fn test_line_1() {
        let expected = BB_A5 | BB_B4 | BB_C3 | BB_D2 | BB_E1;
        assert_eq!(line(SQ_B4, SQ_A5), expected);
    }

    #[test]
    pub fn test_line_segment() {
        let board = line_segment(SQ_F2, SQ_F6);
        println!("{}", board);
        assert_eq!(board, BB_F3 | BB_F4 | BB_F5);
    }
}
