use crate::coord::{permutation as perm_coord, Coord};
use crate::cube::corner::CornerPos;
use crate::cube::edge::EdgePerm;
use crate::util::{EnumIndex, IntoEnumIterator};

use super::super::Cube3Perm;
use super::util::{e_slice_edges, in_e_slice, ud_edges};

pub use crate::cube::coord::corner::CornerPosCoord;

impl Coord<Cube3Perm> for CornerPosCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.corners())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(self.into_perm(), Default::default())
    }
}

make_newtype_enum_index! {
    /// Position of E slice edges in the E slice.
    ///
    /// Note: This coordinate is only valid if all of the E edges are in the E slice.
    ///
    /// There are 4! = 24 values.
    pub struct ESliceEdgePosCoord(u8): 24;
}

impl Default for ESliceEdgePosCoord {
    fn default() -> Self {
        Self(0)
    }
}

impl Coord<EdgePerm> for ESliceEdgePosCoord {
    fn from_perm(perm: &EdgePerm) -> Self {
        let c = perm_coord::calculate_coord(
            e_slice_edges(),
            e_slice_edges().map(|p| {
                let pos = perm[p].pos;
                assert!(
                    in_e_slice(pos),
                    "E slice edges must be in the E slice to use ESliceEdgePosCoord"
                );
                pos
            }),
        );

        ESliceEdgePosCoord(c)
    }

    fn into_perm(self) -> EdgePerm {
        let mut res = EdgePerm::default();
        // Need this edge to get its index
        let first_e_edge = e_slice_edges().next().unwrap();
        perm_coord::apply_coord(
            self.0,
            e_slice_edges(),
            &mut res.cubies[first_e_edge.index()..],
        );

        res
    }
}

impl Coord<Cube3Perm> for ESliceEdgePosCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.edges())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(Default::default(), self.into_perm())
    }
}

make_newtype_enum_index! {
    /// Position of U and D edges.
    ///
    /// This uses a Lehmer code, which is based off how many positions are in the
    /// wrong order.
    ///
    /// Note: This coordinate is only valid if every U or D edge is in the U or D layer (i.e. the E
    /// slice edges are in the E slice).
    ///
    /// There are 8! = 40,320 values.
    pub struct UdEdgePosCoord(u16): 40_320;
}

impl Default for UdEdgePosCoord {
    fn default() -> Self {
        // 0 means all edges are in order
        Self(0)
    }
}

impl Coord<EdgePerm> for UdEdgePosCoord {
    fn from_perm(perm: &EdgePerm) -> Self {
        let c = perm_coord::calculate_coord(ud_edges(), ud_edges().map(|p| perm[p].pos));

        UdEdgePosCoord(c)
    }

    fn into_perm(self) -> EdgePerm {
        let mut res = EdgePerm::default();
        perm_coord::apply_coord(self.0, ud_edges(), &mut res.cubies);

        res
    }
}

impl Coord<Cube3Perm> for UdEdgePosCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.edges())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(Default::default(), self.into_perm())
    }
}

make_composite_coord! {
    /// Coordinate for phase 2, excluding the E slice edges.
    ///
    /// This is used to build the phase 2 pruning table.
    ///
    /// This includes the positions of all edges, assuming the E slice edges are in the E slice (
    /// and hence the U and D edges are in the U and D slices).
    ///
    /// There are 8! * 8! = 1,625,702,400 values.
    pub struct Phase2MinusECoord<Cube3Perm>(u32) {
        corners: CornerPosCoord,
        edges: UdEdgePosCoord,
    }
}

impl Default for Phase2MinusECoord {
    fn default() -> Self {
        Self(0)
    }
}

impl Coord<Cube3Perm> for Phase2MinusECoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::new(
            CornerPosCoord::from_perm(perm),
            UdEdgePosCoord::from_perm(perm),
        )
    }

    fn into_perm(self) -> Cube3Perm {
        let corners_perm: Cube3Perm = self.corners().into_perm();
        let mut edges_perm: Cube3Perm = self.edges().into_perm();

        // Apply corners to UD edges perm
        for p in CornerPos::iter() {
            edges_perm.corners_mut()[p] = corners_perm.corners()[p];
        }

        edges_perm
    }
}

make_composite_coord! {
    /// Coordinate for phase 2.
    ///
    /// This includes the positions of all edges, assuming the E slice edges are in the E slice (
    /// and hence the U and D edges are in the U and D slices).
    ///
    /// There are 8! * 8! * 4! = 39,016,857,600 values.
    pub struct Phase2Coord<Cube3Perm>(u64) {
        ud_cubies: Phase2MinusECoord,
        e_edges: ESliceEdgePosCoord,
    }
}

impl Default for Phase2Coord {
    fn default() -> Self {
        Self(0)
    }
}

impl Coord<Cube3Perm> for Phase2Coord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::new(
            Phase2MinusECoord::from_perm(perm),
            ESliceEdgePosCoord::from_perm(perm),
        )
    }

    fn into_perm(self) -> Cube3Perm {
        let e_slice_perm: Cube3Perm = self.e_edges().into_perm();
        let mut rest_perm: Cube3Perm = self.ud_cubies().into_perm();

        // Apply E slice edges to the rest
        for p in e_slice_edges() {
            rest_perm.edges_mut()[p] = e_slice_perm.edges()[p];
        }

        rest_perm
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::coord::test::test_coord_into_from_perm;

    #[test]
    fn e_slice_edge_pos_coord_into_from_perm() {
        test_coord_into_from_perm::<EdgePerm, ESliceEdgePosCoord>();
    }

    #[test]
    fn ud_edge_pos_coord_into_from_perm() {
        test_coord_into_from_perm::<EdgePerm, UdEdgePosCoord>();
    }

    // TODO Tests for the rest? They are pretty big (spot tests will probably be better)
}
