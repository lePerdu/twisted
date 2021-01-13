//! Helper functions for computing 3x3x3 cube coordinates.
//!
//! This are mostly concerned with distinguishing between E-slice edges and U-D slice edges.

use crate::cube::edge::EdgePos;
use crate::util::IntoEnumIterator;

pub fn in_e_slice(pos: EdgePos) -> bool {
    // E edges are last, DL is the last D edge
    pos > EdgePos::DL
}

pub fn ud_edges() -> impl Iterator<Item = EdgePos> + DoubleEndedIterator + ExactSizeIterator {
    // EdgePos is organized to have the UD edges first
    EdgePos::iter().take(8)
}

pub fn e_slice_edges() -> impl Iterator<Item = EdgePos> + DoubleEndedIterator + ExactSizeIterator {
    // EdgePos is organized to have the UD edges first
    EdgePos::iter().skip(8)
}
