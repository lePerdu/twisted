//! Coordinates are integer representations of a permutation or set of
//! permutations.
//!
//! Some examples of coordinates include: corner/edge permutation/orientation or
//! the locations of the D layer corners.
//!
//! A coordinate can be seen as a set of right cosets (sub-groups of the
//! form `{ h g | h \in H }` where `H` is a sub-group defining the coordinate
//! and `g` is any permutation). In this case, `H` is the sub-group of all
//! permutations which do not change the coordinate value and `g` is any
//! arbitrary permutation which has a particular coordinate value.
//!
//! For example, `H` for the corner orientation coordinate is the set of all
//! permutations which do not change corner orientation. If a corner
//! orientation coordinate value of of `x` means that the URF corner is oriented
//! clockwise and the UFL corner is oriented anti-clockwise, then the coset for
//! that coordinate value would be `H g` where `g` is some permutation which has
//! those corner orientations.
//!
//! Because coordinates can be defined by cosets, coordinates are orthogonal:
//! they can be computed separately from other coordinates, even when applying a
//! complex permutation. If two permutations `a` and `b` have the same
//! coordinate value `x`, then they both belong to the coset `H g` for some `g`.
//! Then if a permutation `r` is applied on top of `a` and `b`, then `a' = a r
//! \in (H g) r = H (g r)` and `b' = b r \in (H g) r = H (g r)` will have the
//! same coordinate `x'`, the one defined by the coset `H (g r)`.
//!
//! By storing a permutation using a few coordinates, move tables can be
//! constructed for each coordinate to perform a set of "generator" permutations
//! (usually the basic turns that can be made on a puzzle) very efficiently.
//! Since coordinates are orthogonal as described above, the size of
//! the tables necessary is proportional to the _sum_ of the coordinate sizes
//! rather than the _product_ of the coordinate sizes (which would be the case
//! if a single coordinate for the whole puzzle state was used).

use crate::puzzle::PuzzlePerm;
use crate::util::EnumIndex;

pub mod parity;
pub mod permutation;

/// A puzzle coordinate
///
/// This trait requires bounds for valid coordinate values and convertions
/// to/from integers/permutations.
pub trait Coord<P>: EnumIndex
where
    P: PuzzlePerm,
{
    fn from_perm(perm: &P) -> Self;

    fn into_perm(self) -> P;
}

pub trait CompositeCoord<P: PuzzlePerm>: Copy {
    type CoordA: Coord<P>;
    type CoordB: Coord<P>;

    fn from_coords(a: Self::CoordA, b: Self::CoordB) -> Self;

    fn into_coords(self) -> (Self::CoordA, Self::CoordB);
}

// TODO Make this a derive macro instead? Right now this can't impl Coord for sub-coordinates
// outside of this crate without making a new type wrapper which implements a bunch of other traits.

#[macro_export]
macro_rules! make_composite_coord {
    (
        $( #[ $attrs:meta ] )*
        $v:vis struct $newtype:ident < $puzzle:ty > (
            $inner:ty
        ) {
            $a:ident : $a_type:ty ,
            $b:ident : $b_type:ty $(,)?
        }
    ) => {
        make_newtype_enum_index! {
            $( #[$attrs] )*
            $v struct $newtype($inner) : (
                <$a_type as $crate::util::EnumCount>::COUNT *
                <$b_type as $crate::util::EnumCount>::COUNT
            );
        }

        impl $newtype {
            $v fn new(a: $a_type, b: $b_type) -> Self {
                use $crate::util::{EnumCount, EnumIndex};
                Self((a.index() * <$b_type>::COUNT + b.index()) as $inner)
            }

            $v fn $a(&self) -> $a_type {
                use num_traits::FromPrimitive;
                use $crate::util::EnumCount;
                <$a_type>::from_usize(self.0 as usize / <$b_type>::COUNT).unwrap()
            }

            $v fn $b(&self) -> $b_type {
                use num_traits::FromPrimitive;
                use $crate::util::EnumCount;
                <$b_type>::from_usize(self.0 as usize % <$b_type>::COUNT).unwrap()
            }
        }

        impl $crate::coord::CompositeCoord<$puzzle> for $newtype {
            type CoordA = $a_type;
            type CoordB = $b_type;

            fn from_coords(a: $a_type, b: $b_type) -> Self {
                Self::new(a, b)
            }

            fn into_coords(self) -> ($a_type, $b_type) {
                (self.$a(), self.$b())
            }
        }
    };
}

#[cfg(test)]
pub mod test {
    use super::*;

    use std::fmt::Debug;

    /// Tests the the conversion from coord -> perm -> coord is the identity
    /// function for all coord values.
    pub fn test_coord_into_from_perm<P: PuzzlePerm, C: Coord<P> + Debug>() {
        for coord in C::iter() {
            assert_eq!(
                coord,
                C::from_perm(&coord.into_perm()),
                "Bad conversion for coord value {:?}",
                coord,
            );
        }
    }
}
