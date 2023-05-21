use std::fmt::{Display, Formatter, Write};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Square(pub(crate) u8);

impl Square {
    pub const fn shift(&self, direction: i8) -> Square {
        Square((self.0 as i8 + direction) as u8)
    }
    pub const fn rank(&self) -> u8 {
        self.0 >> 3
    }

    pub const fn file(&self) -> u8 {
        self.0 & 7
    }

    pub const fn from_coords(file: u8, rank: u8) -> Square {
        Square(file + (rank << 3))
    }

    pub const fn as_byte(&self) -> u8 {
        self.0
    }
}

pub const fn rank_dist(sq1: Square, sq2: Square) -> u8 {
    let d = sq1.rank() as i8 - sq2.rank() as i8;
    if d < 0 {
        (-d) as u8
    } else {
        d as u8
    }
}

pub const fn file_dist(sq1: Square, sq2: Square) -> u8 {
    let d = sq1.file() as i8 - sq2.file() as i8;
    if d < 0 {
        (-d) as u8
    } else {
        d as u8
    }
}

pub const fn same_diag(sq1: Square, sq2: Square) -> bool {
    file_dist(sq1, sq2) == rank_dist(sq1, sq2)
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(('a' as u8 + self.file()) as char)?;
        f.write_char(('1' as u8 + self.rank()) as char)
    }
}

impl From<&str> for Square {
    fn from(value: &str) -> Self {
        Square::from_coords(
            value.as_bytes()[0] - 'a' as u8,
            value.as_bytes()[1] - '1' as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::square::Square;
    #[test]
    fn display_square() {
        assert_eq!(Square::from("a1").file(), 0);
        assert_eq!(Square::from("b1").file(), 1);
        assert_eq!(Square::from("c1").file(), 2);
        assert_eq!(Square::from("d1").file(), 3);
        assert_eq!(Square::from("e1").file(), 4);
        assert_eq!(Square::from("f1").file(), 5);
        assert_eq!(Square::from("g1").file(), 6);
        assert_eq!(Square::from("h1").file(), 7);
    }

    #[test]
    fn square_test() {
        let size: u8 = 8;
        for file in 0..size {
            for rank in 0..size {
                let sq = Square::from_coords(file, rank);
                assert_eq!(sq.file(), file);
                assert_eq!(sq.rank(), rank);
                let descr = String::from(('a' as u8 + file) as char)
                    + String::from(('1' as u8 + rank) as char).as_str();
                assert_eq!(format!("{}", sq), descr)
            }
        }
    }
}
