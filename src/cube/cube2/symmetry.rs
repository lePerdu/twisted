//! Describes symmetries of a 2x2x2 cube puzzle.

use crate::cube::corner::{Corner, CornerOrient, CornerPerm, CornerPos};
use crate::puzzle::PuzzlePerm;

/// A symmetry permutation, excluding reflections.
///
/// This is built from 3 components:
///
/// - `urf_rot`: 120 degree clockwise rotation about the axis through URF - DLB
/// - `y_rot`: 90 degree clockwise rotation about the y-axis (U - D)
/// - `x2_rot`: 180 degree rotation about the x-axis (F - B)
///
/// Rotations are applied in the order listed. In all, there are 24 symmetries (3 for URF, 4 for y,
/// and 2 for x2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symmetry(u8);

impl Default for Symmetry {
    fn default() -> Self {
        Symmetry(0)
    }
}

impl Symmetry {
    pub const COUNT: usize = 24;

    pub fn all() -> impl Iterator<Item = Self> {
        (0..Symmetry::COUNT).into_iter().map(|n| Symmetry(n as u8))
    }

    pub fn new(urf_rot: u8, y_rot: u8, x2_rot: u8) -> Self {
        // TODO Use enums instead of u8s? Use debug_asserts?
        assert!(urf_rot < 3);
        assert!(y_rot < 4);
        assert!(x2_rot < 2);

        Symmetry(urf_rot * 8 + y_rot * 2 + x2_rot)
    }

    // TODO FromPrimitive instead?
    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn permutation(&self) -> &CornerPerm {
        &permutations::SYMMETRY_PERMS[self.index() as usize]
    }

    pub fn inverse(&self) -> Symmetry {
        lazy_static! {
            /// Pre-computed table of inverse symmetry transformations
            pub static ref SYMMETRY_INVERSES: [Symmetry; Symmetry::COUNT] = {
                let mut inverses: [Symmetry; Symmetry::COUNT] = Default::default();

                for sym in Symmetry::all() {
                    let perm = sym.permutation();

                    // Look for all of the permutations to find one that inverts this one
                    let mut inv_found = false;
                    for inv_sym in Symmetry::all() {
                        if perm.sequence(inv_sym.permutation()).is_identity() {
                            inverses[sym.index() as usize] = inv_sym;
                            inv_found = true;
                            break;
                        }
                    }

                    if !inv_found {
                        panic!("No inverse symmetry found");
                    }
                }

                inverses
            };
        }

        SYMMETRY_INVERSES[self.index() as usize]
    }

    pub fn is_identity(&self) -> bool {
        self.0 == 0
    }
}

mod permutations {
    use super::*;

    use CornerOrient::*;
    use CornerPos::*;

    // Primitive components
    const URF_ROT: CornerPerm = CornerPerm::new([
        Corner::new(DLF, Clockwise),
        Corner::new(UFL, AntiClockwise),
        Corner::new(URF, Clockwise),
        Corner::new(DFR, AntiClockwise),
        Corner::new(DRB, Clockwise),
        Corner::new(UBR, AntiClockwise),
        Corner::new(ULB, Clockwise),
        Corner::new(DBL, AntiClockwise),
    ]);

    const Y_ROT: CornerPerm = CornerPerm::new([
        Corner::new(UFL, Oriented),
        Corner::new(ULB, Oriented),
        Corner::new(UBR, Oriented),
        Corner::new(URF, Oriented),
        Corner::new(DFR, Oriented),
        Corner::new(DRB, Oriented),
        Corner::new(DBL, Oriented),
        Corner::new(DLF, Oriented),
    ]);

    const X2_ROT: CornerPerm = CornerPerm::new([
        Corner::new(DRB, Oriented),
        Corner::new(DBL, Oriented),
        Corner::new(DLF, Oriented),
        Corner::new(DFR, Oriented),
        Corner::new(URF, Oriented),
        Corner::new(UFL, Oriented),
        Corner::new(ULB, Oriented),
        Corner::new(UBR, Oriented),
    ]);

    lazy_static! {
        pub static ref SYMMETRY_PERMS: [CornerPerm; Symmetry::COUNT] = {
            let mut perms: [CornerPerm; Symmetry::COUNT] = Default::default();

            for sym in Symmetry::all() {
                let idx = sym.index();
                let urf = idx / 8;
                let y = (idx / 2) % 4;
                let x2 = idx % 2;

                perms[idx as usize] = (URF_ROT.ntimes(urf))
                    .sequence(&Y_ROT.ntimes(y))
                    .sequence(&X2_ROT.ntimes(x2));
            }

            perms
        };
    }
}

/// Find a symmetry transformation which solves the DBL corner
pub fn fix_dbl_corner(perm: &CornerPerm) -> (Symmetry, CornerPerm) {
    for sym in Symmetry::all() {
        let transformed = sym.permutation().sequence(perm);

        const SOLVED_DBL: Corner = Corner::new(CornerPos::DBL, CornerOrient::Oriented);
        if transformed[CornerPos::DBL] == SOLVED_DBL {
            return (sym, transformed);
        }
    }

    // TODO Return Option instead?
    // There are no valid cases in which the DBL corner cannot be solved by rotations
    panic!("Unable to find symmetry transformation");
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::primitives;

    #[test]
    fn fix_dbl_of_identity_is_identity() {
        assert_eq!(
            fix_dbl_corner(&CornerPerm::default()),
            (Symmetry::default(), CornerPerm::default())
        );
    }

    #[test]
    fn fix_dbl_is_identity_when_dbl_in_place() {
        use primitives::*;
        assert_eq!(fix_dbl_corner(u()), (Symmetry::default(), u().clone()));
    }

    #[test]
    fn fix_dbl_correct_when_dbl_moves_simple() {
        use primitives::*;
        assert_eq!(fix_dbl_corner(d()), (Symmetry::new(0, 1, 0), u().clone()));
    }

    #[test]
    fn fix_dbl_correct_when_dbl_moves_complex() {
        use primitives::*;
        assert_eq!(fix_dbl_corner(b()), (Symmetry::new(2, 1, 1), f().clone()));
    }
}
