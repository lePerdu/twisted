//! Cubie representation of a 3x3x3 cube puzzle
//!
//! Cube3Perm is a permutation of the corners and edges (position and orientation). It can
//! also be thought of as a cube state by imagining the permutation applied to
//! the solved state.

use crate::puzzle::{PuzzleMove, PuzzlePerm};
use crate::util::EnumIndex;

use super::corner::CornerPerm;
use super::edge::EdgePerm;

pub mod coord;
pub mod notation;
pub mod primitives;
pub mod symmetry;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cube3Perm {
    corners: CornerPerm,
    edges: EdgePerm,
}

impl Cube3Perm {
    pub const fn new(corners: CornerPerm, edges: EdgePerm) -> Self {
        Self { corners, edges }
    }

    pub fn corners(&self) -> &CornerPerm {
        &self.corners
    }

    pub fn corners_mut(&mut self) -> &mut CornerPerm {
        &mut self.corners
    }

    pub fn edges(&self) -> &EdgePerm {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut EdgePerm {
        &mut self.edges
    }
}

impl PuzzlePerm for Cube3Perm {
    fn identity() -> &'static Self {
        lazy_static! {
            static ref IDENTITY: Cube3Perm = Cube3Perm {
                corners: CornerPerm::default(),
                edges: EdgePerm::default(),
            };
        }

        &IDENTITY
    }

    fn sequence(&self, other: &Self) -> Self {
        Cube3Perm {
            corners: self.corners.sequence(&other.corners),
            edges: self.edges.sequence(&other.edges),
        }
    }

    fn invert(&self) -> Self {
        Cube3Perm {
            corners: self.corners.invert(),
            edges: self.edges.invert(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum CubeTurn {
    U,
    U2,
    UP,
    R,
    R2,
    RP,
    F,
    F2,
    FP,
    D,
    D2,
    DP,
    L,
    L2,
    LP,
    B,
    B2,
    BP,
}

impl EnumIndex for CubeTurn {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl PuzzleMove for CubeTurn {
    type Puzzle = Cube3Perm;

    fn permutation(&self) -> &Cube3Perm {
        use primitives::*;
        use CubeTurn::*;

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
            D => d(),
            D2 => d2(),
            DP => d_prime(),
            L => l(),
            L2 => l2(),
            LP => l_prime(),
            B => b(),
            B2 => b2(),
            BP => b_prime(),
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

        use CubeTurn::*;
        match (a, b) {
            (U, U2) | (U, UP) | (U2, UP) => true,
            (R, R2) | (R, RP) | (R2, RP) => true,
            (F, F2) | (F, FP) | (F2, FP) => true,
            (D, D2) | (D, DP) | (D2, DP) => true,
            (L, L2) | (L, LP) | (L2, LP) => true,
            (B, B2) | (B, BP) | (B2, BP) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum G1CubeTurn {
    U,
    U2,
    UP,
    D,
    D2,
    DP,
    R2,
    F2,
    L2,
    B2,
}

impl EnumIndex for G1CubeTurn {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl PuzzleMove for G1CubeTurn {
    type Puzzle = Cube3Perm;

    fn permutation(&self) -> &Cube3Perm {
        use primitives::*;
        use G1CubeTurn::*;

        match self {
            U => u(),
            U2 => u2(),
            UP => u_prime(),
            D => d(),
            D2 => d2(),
            DP => d_prime(),
            R2 => r2(),
            F2 => f2(),
            L2 => l2(),
            B2 => b2(),
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

        use G1CubeTurn::*;
        match (a, b) {
            (U, U2) | (U, UP) | (U2, UP) => true,
            (D, D2) | (D, DP) | (D2, DP) => true,
            _ => false,
        }
    }
}
