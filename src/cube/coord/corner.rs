use crate::coord::{parity as parity_coord, permutation as perm_coord, Coord};
use crate::cube::corner::{CornerOrient, CornerPerm, CornerPos};
use crate::util::IntoEnumIterator;

// TODO The corner coordinate calculations are identical to the 2x2x2 versions except for including
// 1 more corner

make_newtype_enum_index! {
    /// Orientation of all corners (0..2187).
    ///
    /// Computed as `co_1 + 3 * co_2 + 3^2 * co_3 + ... + 3^6 * co_7`. The first corner is excluded
    /// since the corner orientation parity of the full puzzle is always 0, so its orientation can
    /// be determined from the others.
    ///
    /// There are 3^6 = 2187 values.
    pub struct CornerOrientCoord(u16): 2187;
}

impl Default for CornerOrientCoord {
    fn default() -> Self {
        // 0 means all oriented
        Self(0)
    }
}

impl Coord<CornerPerm> for CornerOrientCoord {
    fn from_perm(perm: &CornerPerm) -> Self {
        let c = parity_coord::calculate_coord(CornerPos::iter().map(|p| perm[p].orient));
        CornerOrientCoord(c)
    }

    fn into_perm(self) -> CornerPerm {
        let mut res = CornerPerm::default();
        let orients = parity_coord::extract_from_coord(self.0, 0, CornerPos::iter(), |o| {
            CornerOrient::from_i8_unsafe(o as i8)
        });
        for (p, o) in orients {
            res[p].orient = o;
        }

        res
    }
}

make_newtype_enum_index! {
    /// Position of all corners (0..40320).
    ///
    /// This uses a Lehmer code, which is based off how many positions are in the
    /// wrong order. Particularly, this is computed as
    /// `l_1 * 1! + l_2 * 2! + ... + l_7 * 7!`, where each `l_i` is the number of corners before
    /// index `i` which belong after it. The first corner is excluded because there are no corners
    /// before it (alternatively, the last corner is ommitted) because there is only one place for
    /// it to go.
    ///
    /// There are 8! = 40320 values.
    pub struct CornerPosCoord(u16): 40320;
}

impl Default for CornerPosCoord {
    fn default() -> Self {
        // 0 means all corners are in order
        Self(0)
    }
}

impl Coord<CornerPerm> for CornerPosCoord {
    fn from_perm(perm: &CornerPerm) -> Self {
        let c =
            perm_coord::calculate_coord(CornerPos::iter(), CornerPos::iter().map(|p| perm[p].pos));

        CornerPosCoord(c)
    }

    fn into_perm(self) -> CornerPerm {
        let mut res = CornerPerm::default();
        perm_coord::apply_coord(self.0, CornerPos::iter(), &mut res.cubies);

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::coord::test::test_coord_into_from_perm;

    #[test]
    fn corner_orient_coord_into_from_perm() {
        test_coord_into_from_perm::<CornerPerm, CornerOrientCoord>();
    }

    #[test]
    fn corner_pos_coord_into_from_perm() {
        test_coord_into_from_perm::<CornerPerm, CornerPosCoord>();
    }
}
