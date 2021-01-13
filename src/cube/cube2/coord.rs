use crate::coord::{parity as parity_coord, permutation as perm_coord, Coord};
use crate::cube::corner::{CornerOrient, CornerPerm, CornerPos};
use crate::util::IntoEnumIterator;

make_newtype_enum_index! {
    /// Orientation of all but 1 corners (0..729).
    ///
    /// Computed as `co_1 + 3 * co_2 + 3^2 * co_3 + ... + 3^5 * co_6`. The last corner (DBL) is
    /// excluded because it is assumed to be solved. The first corner is excluded since the corner
    /// orientation parity of the full puzzle is always 0, so its orientation can be determined from
    /// the others.
    ///
    /// Ranges from 0 to 728 (3^6 - 1).
    pub struct CornerOrient7Coord(u16): 729;
}

impl Default for CornerOrient7Coord {
    fn default() -> Self {
        // 0 means all oriented
        Self(0)
    }
}

fn important_corners() -> impl Iterator<Item = CornerPos> + DoubleEndedIterator + ExactSizeIterator
{
    CornerPos::iter().take(7)
}

impl Coord<CornerPerm> for CornerOrient7Coord {
    fn from_perm(perm: &CornerPerm) -> Self {
        let c = parity_coord::calculate_coord(important_corners().map(|p| perm[p].orient));
        CornerOrient7Coord(c)
    }

    fn into_perm(self) -> CornerPerm {
        let mut res = CornerPerm::default();
        let orients = parity_coord::extract_from_coord(self.0, 0, important_corners(), |o| {
            CornerOrient::from_i8_unsafe(o as i8)
        });
        for (p, o) in orients {
            res[p].orient = o;
        }

        res
    }
}

make_newtype_enum_index! {
    /// Position of all but 1 corners (0..5040).
    ///
    /// This uses a Lehmer code, which is based off how many positions are in the
    /// wrong order. Particularly, this is computed as
    /// `l_1 * 1! + l_2 * 2! + ... + l_6 * 6!`, where each `l_i` is the number of
    /// corners before index `i` which belong after it. The last corner (DBL) is excluded because it
    /// is assumed to be in place. The first corner is excluded because there are no corners before
    /// it (alternatively, the last corner is ommitted) because there is only one place for it to
    /// go.
    ///
    /// Ranges from 0 to 5039 (7! - 1).
    pub struct CornerPos7Coord(u16): 5040;
}

impl Default for CornerPos7Coord {
    fn default() -> Self {
        // 0 means all corners are in order
        Self(0)
    }
}

impl Coord<CornerPerm> for CornerPos7Coord {
    fn from_perm(perm: &CornerPerm) -> Self {
        let c = perm_coord::calculate_coord(
            important_corners(),
            important_corners().map(|p| perm[p].pos),
        );

        CornerPos7Coord(c)
    }

    fn into_perm(self) -> CornerPerm {
        let mut res = CornerPerm::default();
        perm_coord::apply_coord(self.0, important_corners(), &mut res.cubies);

        res
    }
}

make_composite_coord! {
    #[derive(Default)]
    pub struct Corner7Coord<CornerPerm>(u32) {
        orient: CornerOrient7Coord,
        pos: CornerPos7Coord,
    }
}

impl Coord<CornerPerm> for Corner7Coord {
    fn from_perm(perm: &CornerPerm) -> Self {
        Corner7Coord::new(
            CornerOrient7Coord::from_perm(perm),
            CornerPos7Coord::from_perm(perm),
        )
    }

    fn into_perm(self) -> CornerPerm {
        let orientations = self.orient().into_perm();
        let mut positions = self.pos().into_perm();

        // Apply orientations to the positions
        for p in CornerPos::iter() {
            positions[p].orient = orientations[p].orient;
        }

        positions
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::coord::test::test_coord_into_from_perm;

    #[test]
    fn corner_orient_coord_into_from_perm() {
        test_coord_into_from_perm::<CornerPerm, CornerOrient7Coord>();
    }

    #[test]
    fn corner_pos_coord_into_from_perm() {
        test_coord_into_from_perm::<CornerPerm, CornerPos7Coord>();
    }

    #[test]
    fn corner_coord_into_from_perm() {
        test_coord_into_from_perm::<CornerPerm, Corner7Coord>();
    }
}
