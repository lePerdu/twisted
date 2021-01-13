use std::str::FromStr;

use super::{Cube3Perm, CubeTurn, G1CubeTurn};

use crate::notation::{NotationMove, NotationPrim};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumCount, EnumIter)]
pub enum Cube3Notation {
    U,
    R,
    F,
    D,
    L,
    B,
}

impl ToString for Cube3Notation {
    fn to_string(&self) -> String {
        use Cube3Notation::*;
        match self {
            U => "U",
            R => "R",
            F => "F",
            D => "D",
            L => "L",
            B => "B",
        }
        .to_string()
    }
}

impl FromStr for Cube3Notation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Cube3Notation::*;
        match s {
            "U" => Ok(U),
            "R" => Ok(R),
            "F" => Ok(F),
            "D" => Ok(D),
            "L" => Ok(L),
            "B" => Ok(B),
            _ => Err(()),
        }
    }
}

impl NotationPrim for Cube3Notation {
    type Puzzle = Cube3Perm;

    fn permutation(&self) -> &Cube3Perm {
        use super::primitives::*;
        use Cube3Notation::*;
        match self {
            U => u(),
            R => r(),
            F => f(),
            D => d(),
            L => l(),
            B => b(),
        }
    }
}

impl Into<NotationMove<Cube3Notation>> for CubeTurn {
    fn into(self) -> NotationMove<Cube3Notation> {
        use Cube3Notation::*;
        match self {
            CubeTurn::U => NotationMove::basic(U),
            CubeTurn::U2 => NotationMove::ntimes(U, 2),
            CubeTurn::UP => NotationMove::inverse(U),
            CubeTurn::R => NotationMove::basic(R),
            CubeTurn::R2 => NotationMove::ntimes(R, 2),
            CubeTurn::RP => NotationMove::inverse(R),
            CubeTurn::F => NotationMove::basic(F),
            CubeTurn::F2 => NotationMove::ntimes(F, 2),
            CubeTurn::FP => NotationMove::inverse(F),
            CubeTurn::D => NotationMove::basic(D),
            CubeTurn::D2 => NotationMove::ntimes(D, 2),
            CubeTurn::DP => NotationMove::inverse(D),
            CubeTurn::L => NotationMove::basic(L),
            CubeTurn::L2 => NotationMove::ntimes(L, 2),
            CubeTurn::LP => NotationMove::inverse(L),
            CubeTurn::B => NotationMove::basic(B),
            CubeTurn::B2 => NotationMove::ntimes(B, 2),
            CubeTurn::BP => NotationMove::inverse(B),
        }
    }
}

impl Into<NotationMove<Cube3Notation>> for G1CubeTurn {
    fn into(self) -> NotationMove<Cube3Notation> {
        use Cube3Notation::*;
        match self {
            G1CubeTurn::U => NotationMove::basic(U),
            G1CubeTurn::U2 => NotationMove::ntimes(U, 2),
            G1CubeTurn::UP => NotationMove::inverse(U),
            G1CubeTurn::R2 => NotationMove::ntimes(R, 2),
            G1CubeTurn::F2 => NotationMove::ntimes(F, 2),
            G1CubeTurn::D => NotationMove::basic(D),
            G1CubeTurn::D2 => NotationMove::ntimes(D, 2),
            G1CubeTurn::DP => NotationMove::inverse(D),
            G1CubeTurn::L2 => NotationMove::ntimes(L, 2),
            G1CubeTurn::B2 => NotationMove::ntimes(B, 2),
        }
    }
}
