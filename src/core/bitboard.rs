use crate::core::square::Square;
use std::fmt::{Display, Formatter, Write};
use std::ops;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BitBoard(pub(crate) u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);
    pub const FULL: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);

    #[inline]
    pub const fn num_squares(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub const fn intersects(self, other: BitBoard) -> bool {
        self.intersection(other).0 != 0
    }

    #[inline]
    pub const fn intersects2(self, other: BitBoard, other2: BitBoard) -> bool {
        self.intersection(other).intersection(other2).0 != 0
    }

    #[inline]
    pub const fn intersection(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }

    #[inline]
    pub const fn empty(self) -> bool {
        self.0 == 0
    }

    pub const fn shift(self, amount: i8) -> BitBoard {
        if amount > 0 {
            BitBoard(self.0 << amount)
        } else if amount < 0 {
            BitBoard(self.0 >> -amount)
        } else {
            self
        }
    }

    pub const fn iter(self) -> BitBoardIterator {
        BitBoardIterator(self)
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Square {
        let sq = Square(self.0.trailing_zeros() as u8);
        self.0 &= self.0 - 1;
        return sq;
    }

    #[inline(always)]
    pub const fn from_coords(file: u8, rank: u8) -> BitBoard {
        BitBoard((1 as u64) << (8 * rank + file))
    }

    #[inline]
    pub const fn from_square(square: Square) -> BitBoard {
        BitBoard(1 << square.0)
    }
}

pub struct BitBoardIterator(BitBoard);

impl Iterator for BitBoardIterator {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == BitBoard::EMPTY {
            None
        } else {
            let ans = BitBoard(1 << self.0 .0.trailing_zeros());
            self.0 .0 &= self.0 .0 - 1;
            Some(ans)
        }
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard::from_square(value)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..=7).rev() {
            f.write_fmt(format_args!(" {} ", y + 1))?;
            for x in 0..=7 {
                let sq = BitBoard::from_coords(x, y);
                if sq.0 & self.0 != 0 {
                    f.write_str(" x")?;
                } else {
                    f.write_str(" .")?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_str("\n    a b c d e f g h\n")
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}
impl ops::Mul for BitBoard {
    type Output = bool;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 & rhs.0) != 0
    }
}

impl ops::Mul<Square> for BitBoard {
    type Output = bool;

    fn mul(self, rhs: Square) -> Self::Output {
        self * BitBoard::from_square(rhs)
    }
}

impl ops::Mul<BitBoard> for Square {
    type Output = bool;

    fn mul(self, rhs: BitBoard) -> Self::Output {
        BitBoard::from_square(self) * rhs
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOr<Square> for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Square) -> Self::Output {
        BitBoard(self.0 | 1 << rhs.0)
    }
}

impl Add<BitBoard> for BitBoard {
    type Output = Self;

    fn add(self, rhs: BitBoard) -> Self::Output {
        self | rhs
    }
}

impl Add<Square> for BitBoard {
    type Output = Self;

    fn add(self, rhs: Square) -> Self::Output {
        self | rhs
    }
}

impl Sub<BitBoard> for BitBoard {
    type Output = Self;

    fn sub(self, rhs: BitBoard) -> Self::Output {
        self & !rhs
    }
}

impl SubAssign<BitBoard> for BitBoard {
    fn sub_assign(&mut self, rhs: BitBoard) {
        self.0 &= !rhs.0;
    }
}
impl BitXor<BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 ^= rhs.0
    }
}

impl Sub<Square> for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Square) -> Self::Output {
        BitBoard(self.0 & !(1 << rhs.0))
    }
}

impl SubAssign<Square> for BitBoard {
    fn sub_assign(&mut self, rhs: Square) {
        self.0 &= !(1 << rhs.0)
    }
}

impl BitOrAssign<BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bitboard::BitBoard;
    use crate::core::bitboard_constants::{FILE, RANK};
    use crate::core::square::Square;
    use rand::{RngCore, SeedableRng};

    #[test]
    pub fn test_struct_size() {
        assert_eq!(std::mem::size_of_val(&BitBoard::EMPTY), 8);
    }

    #[test]
    pub fn test_bitboard_consistency() {
        for file in 0..8 {
            for rank in 0..8 {
                let bitboard = BitBoard::from_coords(file, rank);
                let bitboard_sq = BitBoard::from(Square::from_coords(file, rank));
                assert_eq!(bitboard, bitboard_sq);
                assert_ne!(
                    bitboard & FILE[file as usize] & RANK[rank as usize],
                    BitBoard::EMPTY
                );
                for k in 0..8 {
                    if k != file {
                        assert_eq!(bitboard & FILE[k as usize], BitBoard::EMPTY);
                    }
                    if k != rank {
                        assert_eq!(bitboard & RANK[k as usize], BitBoard::EMPTY);
                    }
                }
            }
        }
    }

    #[test]
    pub fn perf_test_mul() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        for _i in 0..3000000 {
            let bb1 = BitBoard(rng.next_u64());
            let bb2 = BitBoard(rng.next_u64());
            assert_eq!(bb1 * bb2, bb1.0 & bb2.0 != 0);
        }
    }

    #[test]
    pub fn perf_test_intersects() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        for _i in 0..3000000 {
            let bb1 = BitBoard(rng.next_u64());
            let bb2 = BitBoard(rng.next_u64());
            assert_eq!(bb1.intersects(bb2), bb1.0 & bb2.0 != 0);
        }
    }
}
