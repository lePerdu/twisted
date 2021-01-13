//! Corner cubie representation

use std::ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign};

use crate::puzzle::PuzzlePerm;
use crate::util::{EnumIndex, IntoEnumIterator};

use super::face::Face;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum CornerPos {
    ULB,
    UBR,
    URF,
    UFL,
    DLF,
    DFR,
    DRB,
    DBL,
}

impl EnumIndex for CornerPos {
    fn index(&self) -> usize {
        *self as usize
    }
}

/// Structure for storing the faces of a corner cubie
#[derive(Clone, Copy, Debug)]
pub struct CornerFaces {
    pub oriented: Face,
    pub clockwise: Face,
    pub anti_clockwise: Face,
}

impl CornerFaces {
    pub const fn new(oriented: Face, clockwise: Face, anti_clockwise: Face) -> Self {
        Self {
            oriented,
            clockwise,
            anti_clockwise,
        }
    }

    /// Gets the face at the specified orientation
    pub fn get_face(&self, orientation: CornerOrient) -> Face {
        match orientation {
            CornerOrient::Oriented => self.oriented,
            CornerOrient::Clockwise => self.clockwise,
            CornerOrient::AntiClockwise => self.anti_clockwise,
        }
    }
}

impl CornerPos {
    pub fn get_faces(&self) -> CornerFaces {
        use CornerPos::*;
        use Face::*;
        match self {
            ULB => CornerFaces::new(Up, Left, Back),
            UBR => CornerFaces::new(Up, Back, Right),
            UFL => CornerFaces::new(Up, Front, Left),
            URF => CornerFaces::new(Up, Right, Front),
            DLF => CornerFaces::new(Down, Left, Front),
            DFR => CornerFaces::new(Down, Front, Right),
            DRB => CornerFaces::new(Down, Right, Back),
            DBL => CornerFaces::new(Down, Back, Left),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter, FromPrimitive)]
pub enum CornerOrient {
    Oriented = 0,
    Clockwise = 1,
    AntiClockwise = 2,
}

impl CornerOrient {
    /// Unsafe function for conversion from an integer < 3
    ///
    /// For internal use only
    pub(crate) fn from_i8_unsafe(n: i8) -> Self {
        match n {
            0 => CornerOrient::Oriented,
            1 => CornerOrient::Clockwise,
            2 => CornerOrient::AntiClockwise,
            _ => panic!("Invalid corner orient value"),
        }
    }
}

impl EnumIndex for CornerOrient {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl Add for CornerOrient {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let val = ((self as i8) + (other as i8)) % 3;
        CornerOrient::from_i8_unsafe(val)
    }
}

impl Neg for CornerOrient {
    type Output = Self;

    fn neg(self) -> Self {
        CornerOrient::from_i8_unsafe((3 - self as i8) % 3)
    }
}

impl Sub for CornerOrient {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let val = (3 + (self as i8) - (other as i8)) % 3;
        CornerOrient::from_i8_unsafe(val)
    }
}

impl AddAssign for CornerOrient {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for CornerOrient {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// A corner "facelet" (sticker on a cube)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corner {
    pub pos: CornerPos,
    pub orient: CornerOrient,
}

impl Corner {
    pub const fn new(pos: CornerPos, orient: CornerOrient) -> Self {
        Self { pos, orient }
    }
}

/// Permutation on a corners of a cube puzzle.
///
/// This stores a permutation as a mapping from corner positions to the corner
/// which is sent to that position and its orientation change.
///
/// This type implements `Index` and `IndexMut` for getting the corner which
/// goes to the indexed position. As said above, the returned value is the cubie
/// which _replaces_ the queried position, not the cubie to which the queried
/// position _replaces_.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CornerPerm {
    // TODO Refactor so that this can be private (it's used in coordinate calculations)
    pub(crate) cubies: [Corner; 8],
}

impl CornerPerm {
    pub const fn new(cubies: [Corner; 8]) -> Self {
        Self { cubies }
    }

    /// Creates an empty (but invalid) CornerPerm.
    ///
    /// This should only be used when constructing new permutations, never
    /// directly, since it is not a valid permutation.
    fn empty() -> Self {
        CornerPerm {
            cubies: [Corner::new(CornerPos::URF, CornerOrient::Oriented); 8],
        }
    }

    /// Gets the face at a particular facelet.
    pub fn get_face(&self, facelet: Corner) -> Face {
        let corner = self[facelet.pos];
        // Orientation is offset by the current orientation
        corner
            .pos
            .get_faces()
            .get_face(facelet.orient - corner.orient)
    }

    pub fn iter(&self) -> impl Iterator<Item = (CornerPos, &Corner)> {
        CornerPos::iter().map(move |p| (p, &self[p]))
    }
}

impl Default for CornerPerm {
    fn default() -> Self {
        Self::identity().clone()
    }
}

// Indexing gives the cubie position and orientation which is sent to the index.

impl Index<CornerPos> for CornerPerm {
    type Output = Corner;

    fn index(&self, pos: CornerPos) -> &Self::Output {
        &self.cubies[pos as usize]
    }
}

impl IndexMut<CornerPos> for CornerPerm {
    fn index_mut(&mut self, pos: CornerPos) -> &mut Self::Output {
        &mut self.cubies[pos as usize]
    }
}

impl PuzzlePerm for CornerPerm {
    fn identity() -> &'static Self {
        use CornerOrient::*;
        use CornerPos::*;

        const IDENTITY: CornerPerm = CornerPerm {
            cubies: [
                Corner::new(ULB, Oriented),
                Corner::new(UBR, Oriented),
                Corner::new(URF, Oriented),
                Corner::new(UFL, Oriented),
                Corner::new(DLF, Oriented),
                Corner::new(DFR, Oriented),
                Corner::new(DRB, Oriented),
                Corner::new(DBL, Oriented),
            ],
        };

        &IDENTITY
    }

    fn sequence(&self, other: &Self) -> Self {
        let mut res = CornerPerm::empty();
        for pos in CornerPos::iter() {
            let final_src = self[other[pos].pos];
            res[pos] = Corner::new(final_src.pos, other[pos].orient + final_src.orient);
        }

        res
    }

    fn invert(&self) -> Self {
        let mut res = CornerPerm::empty();
        for pos in CornerPos::iter() {
            let dst_corner = self[pos];
            res[dst_corner.pos] = Corner::new(pos, -dst_corner.orient);
        }

        res
    }
}
