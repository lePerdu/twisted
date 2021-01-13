//! Cubie representation of a 2x2x2 cube puzzle
//!
//! CornerPerm is a permutation of the corners (position and orientation). It can
//! also be thought of as a cube state by imagining the permutation applied to
//! the solved state.

use crate::puzzle::PuzzleMove;
use crate::util::EnumIndex;

use super::corner::CornerPerm;

pub mod coord;
pub mod notation;
pub mod primitives;
pub mod render;
pub mod symmetry;

pub use notation::Cube2Notation;

pub type Cube2Perm = CornerPerm;

/// Turns about the U, R, and F faces
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum UrfTurn {
    U,
    U2,
    UP,
    R,
    R2,
    RP,
    F,
    F2,
    FP,
}

impl EnumIndex for UrfTurn {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl PuzzleMove for UrfTurn {
    type Puzzle = CornerPerm;

    fn permutation(&self) -> &CornerPerm {
        use primitives::*;
        use UrfTurn::*;

        match self {
            U => u(),
            U2 => u2(),
            UP => u_prime(),
            R => r(),
            R2 => r2(),
            RP => r_prime(),
            F => f(),
            F2 => f2(),
            FP => f_prime(),
        }
    }

    fn combines_with(&self, other: &Self) -> bool {
        // Order them to reduce the number of patterns to check
        let (a, b) = if self >= other {
            (self, other)
        } else {
            (other, self)
        };
        if a == b {
            return true;
        }

        use UrfTurn::*;
        match (a, b) {
            (U, U2) | (U, UP) | (U2, UP) => true,
            (R, R2) | (R, RP) | (R2, RP) => true,
            (F, F2) | (F, FP) | (F2, FP) => true,
            _ => false,
        }
    }
}
