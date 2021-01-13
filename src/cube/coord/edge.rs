use crate::coord::{parity as parity_coord, Coord};
use crate::cube::edge::{EdgeOrient, EdgePerm, EdgePos};
use crate::util::IntoEnumIterator;

// TODO The edge coordinate calculations are identical to edge ones

make_newtype_enum_index! {
    /// Orientation of all edges (0..2048).
    ///
    /// Computed as `eo_1 + 2 * eo_2 + 2^2 * eo_3 + ... + 2^10 * eo_11`. The first edge is
    /// excluded since the edge orientation parity of the full puzzle is always 0, so its
    /// orientation can be determined from the others.
    ///
    /// There are 2^11 = 2048 values.
    pub struct EdgeOrientCoord(u16): 2048;
}

impl Default for EdgeOrientCoord {
    fn default() -> Self {
        // 0 means all oriented
        Self(0)
    }
}

impl Coord<EdgePerm> for EdgeOrientCoord {
    fn from_perm(perm: &EdgePerm) -> Self {
        let c = parity_coord::calculate_coord(EdgePos::iter().map(|p| perm[p].orient));
        EdgeOrientCoord(c)
    }

    fn into_perm(self) -> EdgePerm {
        /*
                let orients = parity_coord::apply_coord(2, 0, 12, self.0)
                    .map(|o| EdgeOrient::from_bool(o == 0));

                let mut res = EdgePerm::default();
                for (p, o) in EdgePos::iter().zip(orients) {
                    res[p].orient = o;
                }
        val
                res
                */

        let mut res = EdgePerm::default();

        let orients = parity_coord::extract_from_coord(self.0, 0, EdgePos::iter(), |o| {
            EdgeOrient::from_bool(o == 0)
        });
        for (p, o) in orients {
            res[p].orient = o;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::coord::test::test_coord_into_from_perm;

    #[test]
    fn edge_orient_coord_into_from_perm() {
        test_coord_into_from_perm::<EdgePerm, EdgeOrientCoord>();
    }
}
