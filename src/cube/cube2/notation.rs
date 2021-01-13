//! 2x2x2 cube puzzle notation primitive

use std::str::FromStr;

use super::{primitives, CornerPerm, UrfTurn};
use crate::notation::{NotationPrim, NotationMove};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cube2Notation {
    U,
    R,
    F,
    D,
    L,
    B,
}

// TODO Make macro for implementing all this

impl FromStr for Cube2Notation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        use Cube2Notation::*;
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

impl ToString for Cube2Notation {
    fn to_string(&self) -> String {
        use Cube2Notation::*;
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

impl NotationPrim for Cube2Notation {
    type Puzzle = CornerPerm;

    fn permutation(&self) -> &CornerPerm {
        use primitives::*;
        use Cube2Notation::*;
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

// TODO Auto impl From<Vec<Into<Cube2Notation>> for NotationStr (not possible?)
impl From<UrfTurn> for NotationMove<Cube2Notation> {
    fn from(turn: UrfTurn) -> Self {
        use Cube2Notation::*;
        match turn {
            UrfTurn::U => NotationMove::basic(U),
            UrfTurn::U2 => NotationMove::ntimes(U, 2),
            UrfTurn::UP => NotationMove::inverse(U),
            UrfTurn::R => NotationMove::basic(R),
            UrfTurn::R2 => NotationMove::ntimes(R, 2),
            UrfTurn::RP => NotationMove::inverse(R),
            UrfTurn::F => NotationMove::basic(F),
            UrfTurn::F2 => NotationMove::ntimes(F, 2),
            UrfTurn::FP => NotationMove::inverse(F),
        }
    }
}
