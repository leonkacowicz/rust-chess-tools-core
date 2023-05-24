use crate::core::bitboard::*;
use crate::core::bitboard_constants::*;
use crate::core::magic_bitboard_magic_numbers::*;
use crate::core::square::Square;
use crate::core::*;

fn partitions(attack_mask: BitBoard) -> Vec<BitBoard> {
    let mut ans = vec![BitBoard::EMPTY; 1 << attack_mask.num_squares()];

    let mut count = 0;
    let mut bb = BitBoard::EMPTY;
    loop {
        ans[count] = bb;
        bb = BitBoard(bb.0.wrapping_sub(attack_mask.0) & attack_mask.0);
        count += 1;
        if bb.empty() {
            return ans;
        }
    }
}

fn rook_occupancies() -> Vec<Vec<BitBoard>> {
    let mut ans = vec![vec![]; 64];
    for i in 0..64 {
        let sq = Square(i);
        let rank = RANK[sq.rank() as usize];
        let file = FILE[sq.file() as usize];
        let range = ((RANK_8_I & RANK_1_I) | rank) & ((FILE_A_I & FILE_H_I) | file);
        let attack_mask = (file | rank) & range - sq;
        ans[i as usize] = partitions(attack_mask);
    }
    ans
}

fn bishop_occupancies() -> Vec<Vec<BitBoard>> {
    let mut ans = vec![vec![]; 64];
    for i in 0..64 {
        let sq = Square(i);
        let origin = BitBoard::from_square(sq);
        let rank = RANK[sq.rank() as usize];
        let file = FILE[sq.file() as usize];
        let range = ((RANK_8_I & RANK_1_I) | rank) & ((FILE_A_I & FILE_H_I) | file);
        let attack_mask = calc_bishop_attacks(origin, BitBoard::EMPTY) & range - sq;
        ans[i as usize] = partitions(attack_mask);
    }
    ans
}

const fn shift_attacks(
    origin: BitBoard,
    range: BitBoard,
    occupancy: BitBoard,
    direction: i8,
) -> BitBoard {
    let mut bb = origin;
    let mut ans = 0 as u64;
    while range.intersects(bb) {
        bb = bb.shift(direction);
        ans |= bb.0;
        if occupancy.intersects(bb) {
            break;
        }
    }
    BitBoard(ans)
}

const fn shift_attacks_left(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_A_I, occupancy, LEFT)
}

const fn shift_attacks_right(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_H_I, occupancy, RIGHT)
}

const fn shift_attacks_up(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, RANK_8_I, occupancy, UP)
}

const fn shift_attacks_down(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, RANK_1_I, occupancy, DOWN)
}

const fn shift_attacks_up_left(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_A_I_RANK_8_I, occupancy, UP_LEFT)
}

const fn shift_attacks_up_right(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_H_I_RANK_8_I, occupancy, UP_RIGHT)
}

const fn shift_attacks_down_left(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_A_I_RANK_1_I, occupancy, DOWN_LEFT)
}

const fn shift_attacks_down_right(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    shift_attacks(origin, FILE_H_I_RANK_1_I, occupancy, DOWN_RIGHT)
}

const fn calc_rook_attacks(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    BitBoard(
        shift_attacks_up(origin, occupancy).0
            | shift_attacks_down(origin, occupancy).0
            | shift_attacks_left(origin, occupancy).0
            | shift_attacks_right(origin, occupancy).0,
    )
}

const fn calc_bishop_attacks(origin: BitBoard, occupancy: BitBoard) -> BitBoard {
    BitBoard(
        shift_attacks_up_left(origin, occupancy).0
            | shift_attacks_up_right(origin, occupancy).0
            | shift_attacks_down_left(origin, occupancy).0
            | shift_attacks_down_right(origin, occupancy).0,
    )
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct SquareMagic {
    attack_table_start: usize,
    attack_mask: BitBoard,
    magic_number: u64,
    shift: i32,
}

impl SquareMagic {
    pub const fn new() -> SquareMagic {
        SquareMagic {
            attack_table_start: 0,
            attack_mask: BitBoard::EMPTY,
            magic_number: 0,
            shift: 0,
        }
    }

    pub const fn with(
        attack_table_start: usize,
        attack_mask: u64,
        magic_number: u64,
        shift: i32,
    ) -> SquareMagic {
        SquareMagic {
            attack_table_start,
            attack_mask: BitBoard(attack_mask),
            magic_number,
            shift,
        }
    }
}

pub const ATTACK_TABLE_SIZE: usize = 262144;

#[derive(Debug)]
pub struct MagicTable {
    attack_table: Vec<BitBoard>,
    square_magics: [SquareMagic; 64],
}

impl MagicTable {
    fn with_size(size: usize) -> MagicTable {
        MagicTable {
            attack_table: vec![BitBoard::EMPTY; size],
            square_magics: [SquareMagic::new(); 64],
        }
    }
    #[inline(always)]
    const fn get_table_index(&self, origin: Square, occupancy: BitBoard) -> usize {
        let square_index = origin.0 as usize;
        let magic = &self.square_magics[square_index];
        magic.attack_table_start
            + ((occupancy.0 & magic.attack_mask.0).wrapping_mul(magic.magic_number) >> magic.shift)
                as usize
    }
    #[inline(always)]
    pub fn attacks(&self, origin: Square, occupancy: BitBoard) -> BitBoard {
        self.attack_table[self.get_table_index(origin, occupancy)]
    }

    #[inline(always)]
    pub fn attacks_empty(&self, origin: Square) -> BitBoard {
        let square_index = origin.0 as usize;
        let magic = &self.square_magics[square_index];
        self.attack_table[magic.attack_table_start]
    }
}

fn calc_all_rook_attacks(occupancies: &Vec<Vec<BitBoard>>) -> Vec<Vec<BitBoard>> {
    let mut ans = vec![vec![]; 64];
    for (i, origin) in BitBoard::FULL.iter().enumerate() {
        ans[i] = vec![BitBoard::EMPTY; occupancies[i].len()];
        for (j, occupancy) in occupancies[i].iter().enumerate() {
            ans[i][j] = calc_rook_attacks(origin, *occupancy);
        }
    }
    ans
}

fn calc_all_bishop_attacks(occupancies: &Vec<Vec<BitBoard>>) -> Vec<Vec<BitBoard>> {
    let mut ans = vec![vec![]; 64];
    for (i, origin) in BitBoard::FULL.iter().enumerate() {
        ans[i] = vec![BitBoard::EMPTY; occupancies[i].len()];
        for (j, occupancy) in occupancies[i].iter().enumerate() {
            ans[i][j] = calc_bishop_attacks(origin, *occupancy);
        }
    }
    ans
}

pub fn create_magic_table(
    occupancies: &Vec<Vec<BitBoard>>,
    all_attacks: &Vec<Vec<BitBoard>>,
    magic_numbers: &[u64; 64],
) -> MagicTable {
    let table_size = occupancies.iter().map(|x| x.len()).sum();
    let mut ans: MagicTable = MagicTable::with_size(table_size);
    let mut last_size: usize = 0;
    for square_index in 0..64 {
        let square = Square(square_index as u8);
        let rank = RANK[square.rank() as usize];
        let file = FILE[square.file() as usize];
        let range = ((RANK_8_I & RANK_1_I) | rank) & ((FILE_A_I & FILE_H_I) | file);

        let magic = &mut ans.square_magics[square_index];
        magic.attack_mask = all_attacks[square_index][0].intersection(range);
        magic.attack_table_start = last_size;
        magic.shift = 64 - magic.attack_mask.num_squares() as i32;
        magic.magic_number = magic_numbers[square_index];
        last_size += occupancies[square_index].len();

        // fill the table
        for (occup_idx, occupancy) in occupancies[square_index].iter().enumerate() {
            let table_index = ans.get_table_index(square, *occupancy);
            ans.attack_table[table_index] = all_attacks[square_index][occup_idx];
        }
    }
    return ans;
}

pub fn bishop_magic_table() -> MagicTable {
    let occupancies = bishop_occupancies();
    let attacks = calc_all_bishop_attacks(&occupancies);
    create_magic_table(&occupancies, &attacks, &BISHOP_MAGIC_NUMBERS)
}

pub fn rook_magic_table() -> MagicTable {
    let occupancies = rook_occupancies();
    let attacks = calc_all_rook_attacks(&occupancies);
    create_magic_table(&occupancies, &attacks, &ROOK_MAGIC_NUMBERS)
}

pub struct MagicTables {
    pub rook_table: MagicTable,
    pub bishop_table: MagicTable,
}

pub fn magic_tables() -> MagicTables {
    MagicTables {
        rook_table: rook_magic_table(),
        bishop_table: bishop_magic_table(),
    }
}

pub const fn rook_attacks_empty(origin: Square) -> BitBoard {
    let magic = ROOK_SQUARE_MAGICS[origin.0 as usize];
    BitBoard(ROOK_ATTACK_TABLE[magic.attack_table_start])
}

pub const fn rook_attacks(origin: Square, occupancy: BitBoard) -> BitBoard {
    let magic = ROOK_SQUARE_MAGICS[origin.0 as usize];
    BitBoard(
        ROOK_ATTACK_TABLE[magic.attack_table_start
            + ((occupancy.0 & magic.attack_mask.0).wrapping_mul(magic.magic_number) >> magic.shift)
                as usize],
    )
}

pub const fn bishop_attacks_empty(origin: Square) -> BitBoard {
    let magic = BISHOP_SQUARE_MAGICS[origin.0 as usize];
    BitBoard(BISHOP_ATTACK_TABLE[magic.attack_table_start])
}

pub const fn bishop_attacks(origin: Square, occupancy: BitBoard) -> BitBoard {
    let magic = BISHOP_SQUARE_MAGICS[origin.0 as usize];
    BitBoard(
        BISHOP_ATTACK_TABLE[magic.attack_table_start
            + ((occupancy.0 & magic.attack_mask.0).wrapping_mul(magic.magic_number) >> magic.shift)
                as usize],
    )
}

#[cfg(test)]
mod tests {
    use crate::core::bitboard_constants::*;
    use crate::core::magic_bitboard::*;
    use crate::core::square_constants::*;
    use lazy_static::lazy_static;
    use std::io::{Error, Write};

    lazy_static! {
        pub static ref ROOK_TABLE: MagicTable = rook_magic_table();
        pub static ref BISHOP_TABLE: MagicTable = bishop_magic_table();
    }

    #[test]
    pub fn table_size() -> Result<(), Error> {
        println!("Rook table size: {}", ROOK_TABLE.attack_table.len() * 8);
        println!("Bishop table size: {}", BISHOP_TABLE.attack_table.len() * 8);

        let mut f = std::fs::File::create("rook.rs").unwrap();

        f.write(b"pub const BISHOP_SQUARE_MAGICS: [SquareMagic; 64] = [\n")?;
        for i in 0..64 {
            let sm = &BISHOP_TABLE.square_magics[i];
            f.write_fmt(format_args!(
                "SquareMagic::with({}, {}, {}, {}),\n",
                sm.attack_table_start, sm.attack_mask.0, sm.magic_number, sm.shift,
            ))?;
        }
        f.write(b"];\n")?;

        f.write_fmt(format_args!(
            "pub const BISHOP_ATTACK_TABLE: [u64; {}] = [",
            BISHOP_TABLE.attack_table.len()
        ))?;

        for i in 0..BISHOP_TABLE.attack_table.len() {
            let sm = &BISHOP_TABLE.attack_table[i];
            f.write_fmt(format_args!("0x{:x},\n", sm.0))?;
        }
        f.write(b"];")?;

        f.write(b"pub const ROOK_SQUARE_MAGICS: [SquareMagic; 64] = [\n")?;
        for i in 0..64 {
            let sm = &ROOK_TABLE.square_magics[i];
            f.write_fmt(format_args!(
                "SquareMagic::with({}, {}, {}, {}),\n",
                sm.attack_table_start, sm.attack_mask.0, sm.magic_number, sm.shift,
            ))?;
        }
        f.write(b"];\n")?;

        f.write_fmt(format_args!(
            "pub const ROOK_ATTACK_TABLE: [u64; {}] = [",
            ROOK_TABLE.attack_table.len()
        ))?;

        for i in 0..ROOK_TABLE.attack_table.len() {
            let sm = &ROOK_TABLE.attack_table[i];
            f.write_fmt(format_args!("0x{:x},\n", sm.0))?;
        }
        f.write(b"];")?;
        Ok(())
    }

    #[test]
    pub fn shift_attacks_test() {
        let origin = BB_D4;
        let occupancy = BB_D7 | BB_F4;
        assert_eq!(shift_attacks_left(origin, occupancy), BB_A4 | BB_B4 | BB_C4);
    }

    #[test]
    pub fn rook_attacks_test1() {
        let origin = BB_D4;
        let occupancy = BB_D7 | BB_F4;
        let attacks = calc_rook_attacks(origin, occupancy);
        let expected = (FILE_D | RANK_4) - BB_D4 - BB_D8 - BB_G4 - BB_H4;
        assert_eq!(attacks, expected);
    }

    #[test]
    pub fn rook_attacks_test2() {
        let origin = BB_D4;
        let occupancy = BB_B4 | BB_D2;
        let attacks = calc_rook_attacks(origin, occupancy);
        let expected = (FILE_D | RANK_4) - BB_D4 - BB_D1 - BB_A4;
        assert_eq!(attacks, expected);
    }

    #[test]
    pub fn bishop_attaks_test() {
        let origin = BB_C1;
        let occupancy = RANK_2;
        assert_eq!(calc_bishop_attacks(origin, occupancy), BB_B2 | BB_D2);
        let bishop_magic = bishop_magic_table();
        assert_eq!(bishop_magic.attacks(SQ_C1, occupancy), BB_B2 | BB_D2);
    }
    #[test]
    pub fn rook_magic_works() {
        let rook_occupancies = rook_occupancies();
        let all_rook_attacks = calc_all_rook_attacks(&rook_occupancies);
        let rook_magic =
            create_magic_table(&rook_occupancies, &all_rook_attacks, &ROOK_MAGIC_NUMBERS);

        for origin in 0..64 {
            let origin_sq = Square(origin);
            let origin_bb = BitBoard::from_square(origin_sq);
            for occupancy in rook_occupancies[origin as usize].iter() {
                let actual = rook_magic.attacks(origin_sq, *occupancy);
                let expected = calc_rook_attacks(origin_bb, *occupancy);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    pub fn bishop_magic_works() {
        let occupancies = bishop_occupancies();
        let all_attacks = calc_all_bishop_attacks(&occupancies);
        let bishop_magic = create_magic_table(&occupancies, &all_attacks, &BISHOP_MAGIC_NUMBERS);

        for origin in 0..64 {
            let origin_sq = Square(origin);
            let origin_bb = BitBoard::from_square(origin_sq);
            for occupancy in occupancies[origin as usize].iter() {
                let actual = bishop_magic.attacks(origin_sq, *occupancy);
                let expected = calc_bishop_attacks(origin_bb, *occupancy);
                assert_eq!(actual, expected);
            }
        }
    }
}
