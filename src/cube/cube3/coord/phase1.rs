use num_integer::binomial;

use crate::coord::Coord;
use crate::cube::corner::CornerPos;
use crate::cube::edge::{EdgePerm, EdgePos};
use crate::util::{rotate_left, EnumIndex, IntoEnumIterator};

use super::super::Cube3Perm;
use super::util::in_e_slice;

pub use crate::cube::coord::{corner::CornerOrientCoord, edge::EdgeOrientCoord};

impl Coord<Cube3Perm> for CornerOrientCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.corners())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(self.into_perm(), Default::default())
    }
}

impl Coord<Cube3Perm> for EdgeOrientCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.edges())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(Default::default(), self.into_perm())
    }
}

make_newtype_enum_index! {
    /// Positions of E slice edges, without regard to their order (0..495).
    ///
    /// This coordinate is 0 when all E slice edges are in the E slice.
    ///
    /// There are C(12, 4) = 495 values, where C(n, k) is the binomial coefficient.
    pub struct EEdgePosCoord(u16): 495;
}

impl Default for EEdgePosCoord {
    fn default() -> Self {
        // 0 means all edges are in the E slice
        Self(0)
    }
}

impl Coord<EdgePerm> for EEdgePosCoord {
    fn from_perm(perm: &EdgePerm) -> Self {
        let mut c = 0;
        // Number of E slice edges seen
        let mut k = 0;

        for (n, p) in EdgePos::iter().enumerate() {
            if in_e_slice(perm[p].pos) {
                k += 1;
            } else if k > 0 {
                c += binomial(n as u16, k - 1);
            }
        }

        EEdgePosCoord(c)
    }

    fn into_perm(self) -> EdgePerm {
        // Each highest coefficient is greater than the sum of the ones before it, so we can check
        // for each highest one.
        let mut c = self.0;
        // Remaining edges - 1
        let mut k = 3;

        let mut res = EdgePerm::default();
        for p in EdgePos::iter().rev() {
            let binom = binomial(p.index() as u16, k);
            if c < binom {
                // This position is in E
                if k == 0 {
                    // Last edge was found
                    break;
                }
                k -= 1;
            } else {
                c -= binom;
                // Shift a non-E edge into position p
                rotate_left(&mut res.cubies[..=p.index()]);
            }
        }

        res
    }
}

impl Coord<Cube3Perm> for EEdgePosCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.edges())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(Default::default(), self.into_perm())
    }
}

make_composite_coord! {
    #[derive(Default)]
    pub struct ESliceAndEOCoord<Cube3Perm>(u32) {
        e_slice: EEdgePosCoord,
        edge_orient: EdgeOrientCoord,
    }
}

impl Coord<EdgePerm> for ESliceAndEOCoord {
    fn from_perm(perm: &EdgePerm) -> Self {
        Self::new(
            EEdgePosCoord::from_perm(perm),
            EdgeOrientCoord::from_perm(perm),
        )
    }

    fn into_perm(self) -> EdgePerm {
        let mut e_slice_perm: EdgePerm = self.e_slice().into_perm();
        let eo_perm: EdgePerm = self.edge_orient().into_perm();

        // Apply orientations to the ESlice perm
        for p in EdgePos::iter() {
            e_slice_perm[p].orient = eo_perm[p].orient;
        }

        e_slice_perm
    }
}

impl Coord<Cube3Perm> for ESliceAndEOCoord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::from_perm(perm.edges())
    }

    fn into_perm(self) -> Cube3Perm {
        Cube3Perm::new(Default::default(), self.into_perm())
    }
}

make_composite_coord! {
    #[derive(Default)]
    pub struct Phase1Coord<Cube3Perm>(u32) {
        corners: CornerOrientCoord,
        edges: ESliceAndEOCoord,
    }
}

impl Coord<Cube3Perm> for Phase1Coord {
    fn from_perm(perm: &Cube3Perm) -> Self {
        Self::new(
            CornerOrientCoord::from_perm(perm),
            ESliceAndEOCoord::from_perm(perm),
        )
    }

    fn into_perm(self) -> Cube3Perm {
        let corners_perm: Cube3Perm = self.corners().into_perm();
        let mut edges_perm: Cube3Perm = self.edges().into_perm();

        // Apply corners to the edge perm
        for p in CornerPos::iter() {
            edges_perm.corners_mut()[p] = corners_perm.corners()[p];
        }

        edges_perm
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::coord::test::test_coord_into_from_perm;

    #[test]
    fn e_edge_pos_coord_into_from_perm() {
        test_coord_into_from_perm::<EdgePerm, EEdgePosCoord>();
    }

    #[test]
    fn eslice_and_eo_coord_into_from_perm() {
        test_coord_into_from_perm::<Cube3Perm, ESliceAndEOCoord>();
    }

    // TODO Test for Phase1Coord? it is quite large (2e9 elements)

    /*
    #[test]
    fn composite_iter_len() {
        use crate::util::EnumCount;
        assert_eq!(ESliceAndEOCoord::iter().count(), ESliceAndEOCoord::COUNT);
    }

    #[test]
    fn composite_iter_order() {
        for (index, coord) in ESliceAndEOCoord::iter().enumerate() {
            assert_eq!(coord.index(), index);
        }
    }

    #[test]
    fn composite_equals_full_table() {
        use crate::cube::cube3::CubeTurn;
        use crate::move_table::{BasicMoveTable, CompositeMoveTable, MoveTable};

        let eo_table: BasicMoveTable<Cube3Perm, EdgeOrientCoord, CubeTurn> =
            BasicMoveTable::create();
        let eslice_table: BasicMoveTable<Cube3Perm, EEdgePosCoord, CubeTurn> =
            BasicMoveTable::create();
        let edge_table_comp =
            CompositeMoveTable::<'_, ESliceAndEOCoord, _, _>::new(&eslice_table, &eo_table);
        let edge_table_basic: BasicMoveTable<Cube3Perm, ESliceAndEOCoord, CubeTurn> =
            BasicMoveTable::create();

        for coord in ESliceAndEOCoord::iter() {
            for m in CubeTurn::iter() {
                assert_eq!(
                    edge_table_comp.get_move(coord, m),
                    edge_table_basic.get_move(coord, m),
                    "Different coordaintes for input ({:?}, {:?})",
                    coord,
                    m
                );
            }
        }
    }
    */
}
