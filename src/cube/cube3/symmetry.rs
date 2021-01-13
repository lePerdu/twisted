//! Symmetries of a 3x3x3 cube.

use crate::puzzle::{PuzzleMove, PuzzlePerm};
use crate::util::{EnumCount, EnumIndex, IntoEnumIterator};

use super::Cube3Perm;

make_newtype_enum_index! {
    /// The 16 cube symmetries which preserve the E slice.
    pub struct SymmetryE(u8) : 16;
}

impl Default for SymmetryE {
    fn default() -> Self {
        Self(0)
    }
}

impl SymmetryE {
    pub fn new(y_rot: u8, x2_rot: u8, m_refl: u8) -> Self {
        debug_assert!(y_rot < 4);
        debug_assert!(x2_rot < 2);
        debug_assert!(m_refl < 2);

        Self(y_rot * 4 + x2_rot * 2 + m_refl)
    }

    pub const fn y_rot(&self) -> u8 {
        self.0 / 4
    }

    pub const fn x2_rot(&self) -> u8 {
        (self.0 / 2) % 2
    }

    pub const fn m_refl(&self) -> u8 {
        self.0 % 2
    }
}

impl PuzzleMove for SymmetryE {
    type Puzzle = Cube3Perm;

    fn permutation(&self) -> &Cube3Perm {
        lazy_static! {
            static ref TABLE: [Cube3Perm; SymmetryE::COUNT] = {
                let mut table: [Cube3Perm; SymmetryE::COUNT] = Default::default();

                for sym in SymmetryE::iter() {
                    use primitives::*;
                    table[sym.index()] = Y_ROT
                        .ntimes(sym.y_rot())
                        .sequence(&X2_ROT.ntimes(sym.x2_rot()))
                        .sequence(&M_REFL.ntimes(sym.m_refl()));
                }

                table
            };
        }

        &TABLE[self.0 as usize]
    }

    fn combines_with(&self, _other: &Self) -> bool {
        true
    }
}

// Helper function since lifetype annotations don't work in closures
fn sym_perm(sym: &SymmetryE) -> &Cube3Perm {
    sym.permutation()
}

impl_puzzle_perm_with_tables!(
    SymmetryE,
    || {
        const IDENTITY: SymmetryE = Self(0);
        &IDENTITY
    },
    sym_perm
);

mod primitives {
    use super::*;
    use crate::cube::corner::{Corner, CornerOrient, CornerPerm, CornerPos::*};
    use crate::cube::edge::{Edge, EdgeOrient, EdgePerm, EdgePos::*};

    /// 90 degree rotation about the Y axis (U face)
    pub const Y_ROT: Cube3Perm = Cube3Perm::new(
        CornerPerm::new([
            Corner::new(UFL, CornerOrient::Oriented),
            Corner::new(ULB, CornerOrient::Oriented),
            Corner::new(UBR, CornerOrient::Oriented),
            Corner::new(URF, CornerOrient::Oriented),
            Corner::new(DFR, CornerOrient::Oriented),
            Corner::new(DRB, CornerOrient::Oriented),
            Corner::new(DBL, CornerOrient::Oriented),
            Corner::new(DLF, CornerOrient::Oriented),
        ]),
        EdgePerm::new([
            Edge::new(UR, EdgeOrient::Oriented),
            Edge::new(UB, EdgeOrient::Oriented),
            Edge::new(UL, EdgeOrient::Oriented),
            Edge::new(UF, EdgeOrient::Oriented),
            Edge::new(DR, EdgeOrient::Oriented),
            Edge::new(DB, EdgeOrient::Oriented),
            Edge::new(DL, EdgeOrient::Oriented),
            Edge::new(DF, EdgeOrient::Oriented),
            Edge::new(BR, EdgeOrient::NotOriented),
            Edge::new(BL, EdgeOrient::NotOriented),
            Edge::new(FL, EdgeOrient::NotOriented),
            Edge::new(FR, EdgeOrient::NotOriented),
        ]),
    );

    /// 180 degree rotation about the X axis (F face)
    pub const X2_ROT: Cube3Perm = Cube3Perm::new(
        CornerPerm::new([
            Corner::new(DRB, CornerOrient::Oriented),
            Corner::new(DBL, CornerOrient::Oriented),
            Corner::new(DLF, CornerOrient::Oriented),
            Corner::new(DFR, CornerOrient::Oriented),
            Corner::new(URF, CornerOrient::Oriented),
            Corner::new(UFL, CornerOrient::Oriented),
            Corner::new(ULB, CornerOrient::Oriented),
            Corner::new(UBR, CornerOrient::Oriented),
        ]),
        EdgePerm::new([
            Edge::new(DF, EdgeOrient::Oriented),
            Edge::new(DR, EdgeOrient::Oriented),
            Edge::new(DB, EdgeOrient::Oriented),
            Edge::new(DL, EdgeOrient::Oriented),
            Edge::new(UF, EdgeOrient::Oriented),
            Edge::new(UL, EdgeOrient::Oriented),
            Edge::new(UB, EdgeOrient::Oriented),
            Edge::new(UR, EdgeOrient::Oriented),
            Edge::new(FL, EdgeOrient::Oriented),
            Edge::new(FR, EdgeOrient::Oriented),
            Edge::new(BR, EdgeOrient::Oriented),
            Edge::new(BL, EdgeOrient::Oriented),
        ]),
    );

    /// Reflection about the M slice
    pub const M_REFL: Cube3Perm = Cube3Perm::new(
        CornerPerm::new([
            Corner::new(UBR, CornerOrient::Oriented),
            Corner::new(ULB, CornerOrient::Oriented),
            Corner::new(UFL, CornerOrient::Oriented),
            Corner::new(URF, CornerOrient::Oriented),
            Corner::new(DFR, CornerOrient::Oriented),
            Corner::new(DLF, CornerOrient::Oriented),
            Corner::new(DBL, CornerOrient::Oriented),
            Corner::new(DRB, CornerOrient::Oriented),
        ]),
        EdgePerm::new([
            Edge::new(UF, EdgeOrient::Oriented),
            Edge::new(UR, EdgeOrient::Oriented),
            Edge::new(UB, EdgeOrient::Oriented),
            Edge::new(UL, EdgeOrient::Oriented),
            Edge::new(DF, EdgeOrient::Oriented),
            Edge::new(DL, EdgeOrient::Oriented),
            Edge::new(DB, EdgeOrient::Oriented),
            Edge::new(DR, EdgeOrient::Oriented),
            Edge::new(FL, EdgeOrient::Oriented),
            Edge::new(FR, EdgeOrient::Oriented),
            Edge::new(BR, EdgeOrient::Oriented),
            Edge::new(BL, EdgeOrient::Oriented),
        ]),
    );
}
