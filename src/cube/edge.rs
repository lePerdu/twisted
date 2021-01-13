//! Edge cubie representation

use std::ops::{Add, AddAssign, Index, IndexMut};

use super::face::Face;

use crate::puzzle::PuzzlePerm;
use crate::util::{EnumIndex, IntoEnumIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum EdgePos {
    UF,
    UL,
    UB,
    UR,
    DF,
    DR,
    DB,
    DL,
    FR,
    FL,
    BL,
    BR,
}

impl EnumIndex for EdgePos {
    fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum EdgeOrient {
    Oriented,
    NotOriented,
}

impl EnumIndex for EdgeOrient {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl EdgeOrient {
    pub fn from_bool(oriented: bool) -> Self {
        if oriented {
            EdgeOrient::Oriented
        } else {
            EdgeOrient::NotOriented
        }
    }
}

impl Add for EdgeOrient {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        EdgeOrient::from_bool(self == other)
    }
}

impl AddAssign for EdgeOrient {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

pub struct EdgeFaces {
    oriented: Face,
    not_oriented: Face,
}

impl EdgeFaces {
    pub const fn new(oriented: Face, not_oriented: Face) -> Self {
        Self {
            oriented,
            not_oriented,
        }
    }

    pub fn get_face(&self, orientation: EdgeOrient) -> Face {
        match orientation {
            EdgeOrient::Oriented => self.oriented,
            EdgeOrient::NotOriented => self.not_oriented,
        }
    }
}

impl EdgePos {
    pub fn get_faces(&self) -> EdgeFaces {
        use EdgePos::*;
        use Face::*;
        match self {
            UF => EdgeFaces::new(Up, Front),
            UL => EdgeFaces::new(Up, Left),
            UB => EdgeFaces::new(Up, Back),
            UR => EdgeFaces::new(Up, Right),
            DF => EdgeFaces::new(Down, Front),
            DR => EdgeFaces::new(Down, Right),
            DB => EdgeFaces::new(Down, Back),
            DL => EdgeFaces::new(Down, Left),
            FR => EdgeFaces::new(Front, Right),
            FL => EdgeFaces::new(Front, Left),
            BL => EdgeFaces::new(Back, Left),
            BR => EdgeFaces::new(Back, Right),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub pos: EdgePos,
    pub orient: EdgeOrient,
}

impl Edge {
    pub const fn new(pos: EdgePos, orient: EdgeOrient) -> Self {
        Self { pos, orient }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgePerm {
    // TODO Refactor so that this can be private (it's used in coordinate calculations)
    pub(crate) cubies: [Edge; 12],
}

impl EdgePerm {
    pub const fn new(cubies: [Edge; 12]) -> Self {
        Self { cubies }
    }

    fn empty() -> Self {
        Self {
            cubies: [Edge::new(EdgePos::UF, EdgeOrient::Oriented); 12],
        }
    }
}

impl Index<EdgePos> for EdgePerm {
    type Output = Edge;

    fn index(&self, pos: EdgePos) -> &Edge {
        &self.cubies[pos.index()]
    }
}

impl IndexMut<EdgePos> for EdgePerm {
    fn index_mut(&mut self, pos: EdgePos) -> &mut Edge {
        &mut self.cubies[pos.index()]
    }
}

// TODO This looks almost identical to the impl for CornerPerm

impl PuzzlePerm for EdgePerm {
    fn identity() -> &'static Self {
        lazy_static! {
            static ref IDENTITY: EdgePerm = {
                let mut res = EdgePerm::empty();
                for p in EdgePos::iter() {
                    res[p] = Edge::new(p, EdgeOrient::Oriented);
                }

                res
            };
        }

        &IDENTITY
    }

    fn sequence(&self, other: &Self) -> Self {
        let mut res = EdgePerm::empty();
        for pos in EdgePos::iter() {
            let final_src = self[other[pos].pos];
            res[pos] = Edge::new(final_src.pos, other[pos].orient + final_src.orient);
        }

        res
    }

    fn invert(&self) -> Self {
        let mut res = EdgePerm::empty();
        for pos in EdgePos::iter() {
            let dst_edge = self[pos];
            res[dst_edge.pos] = Edge::new(pos, dst_edge.orient);
        }

        res
    }
}

impl Default for EdgePerm {
    fn default() -> Self {
        Self::identity().clone()
    }
}
