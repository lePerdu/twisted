use std::marker::PhantomData;

use crate::coord::Coord;
use crate::puzzle::{PuzzleMove, PuzzlePerm};
use crate::util::{EnumCount, EnumIndex, IntoEnumIterator};

/// A subgorup of puzzle permutations which separate a puzzle permutation into a set of equivalence
/// classes.
///
/// This will usually correspond to rotations or reflections of a puzzle.
///
/// Note: Symmetry groups are restricted to having no more than 64 elements (including the identity)
/// because they are stored as a bitmask in some scenarios.
pub trait Symmetry: PuzzlePerm + PuzzleMove {}

/// Symmetry-reduced coordinate, constructed from a regular coordinate and a symmetry.
pub trait SymCoord<P: PuzzlePerm>: EnumIndex {
    type BaseCoord: Coord<P>;
    type Symmetry: Symmetry<Puzzle = P>;
    type EquivClass: EnumIndex;

    fn from_sym_and_class(sym: Self::Symmetry, equiv_class: Self::EquivClass) -> Self;

    fn symmetry(&self) -> Self::Symmetry;

    fn equiv_class(&self) -> Self::EquivClass;
}

#[macro_export]
macro_rules! make_symmetry_coord {
    (
        $( #[ $attrs:meta ] )*
        $v:vis struct $newtype:ident < $sym:ty, $coord:ty $(,)? >
        ( $equivclass:ident ( $inner:ty ) ) : $count:expr ;
    ) => {
        make_newtype_enum_index! {
            $v struct $equivclass($inner): $count;
        }

        make_newtype_enum_index! {
            $( #[$attrs] )*
            $v struct $newtype($inner): (($count) * <$sym>::COUNT);
        }

        impl $crate::symmetry::SymCoord<<$sym>::Puzzle> for $newtype {
            type BaseCoord = $coord;
            type Symmetry = $sym;
            type EquivClass = $equivclass;

            fn from_sym_and_class(sym: $sym, equiv_class: Self::EquivClass) -> Self {
                use $crate::util::{EnumIndex, EnumCount};
                Self((equiv_class.index() * <$sym>::COUNT + sym.index()) as $inner)
            }

            fn symmetry(&self) -> $sym {
                use num_traits::FromPrimitive;
                use $crate::util::EnumCount;
                <$sym>::from_usize(self.0 as usize % <$sym>::COUNT).unwrap()
            }

            fn equiv_class(&self) -> Self::EquivClass {
                use $crate::util::EnumCount;
                $equivclass(self.0 / <$sym>::COUNT as $inner)
            }
        }
    };
}

#[derive(Clone, Debug)]
struct RepresentantEntry<C> {
    coord: C,
    // TODO Use a wider / generic type that is guarunteed to fit all symmetries.
    symmetry_bitmask: u64,
}

/// Table of equivalent class representants under a symmetry.
///
/// This is used for creating and using symmetry-reduced coordinates.
#[derive(Clone, Debug)]
pub struct RepresentantTable<P: PuzzlePerm, S: SymCoord<P>> {
    table: Box<[RepresentantEntry<S::BaseCoord>]>,
    _sym: PhantomData<S>,
}

/// Checks whether a permutation is in an equivalence class.
fn sym_equivalent_perm<S: Symmetry, C: Coord<S::Puzzle>>(a: C, b_perm: &S::Puzzle) -> Option<S> {
    S::iter().find(|sym| {
        let conj_perm = sym
            .permutation()
            .sequence(b_perm)
            .sequence(sym.invert().permutation());
        C::from_perm(&conj_perm) == a
    })
}

/// Checks whether 2 coordinate values are equivalent under a symmetry.
fn sym_equivalent<S: Symmetry, C: Coord<S::Puzzle>>(a: C, b: C) -> Option<S> {
    sym_equivalent_perm(a, &b.into_perm())
}

impl<P: PuzzlePerm, S: SymCoord<P>> RepresentantTable<P, S> {
    pub fn create() -> Self {
        assert!(
            S::COUNT <= 64,
            "Cannot calculate representant table for symmetries with more than 64 elements"
        );

        // Get a list of all coordinate values
        // TODO Use an invalid value instead of Option to save space?
        let mut coord_values = S::BaseCoord::iter()
            .map(|c| Some(c))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        // Will be slightly larger, but this is a good starting point
        let mut table = Vec::with_capacity(S::BaseCoord::COUNT / S::COUNT);

        for i in 0..S::BaseCoord::COUNT {
            if let Some(repr_coord) = coord_values[i] {
                let mut entry = RepresentantEntry {
                    coord: repr_coord,
                    // Start with all symmetries, and narrow it down when a distinct permutation is
                    // found to be equal to the representant under some symmetry
                    symmetry_bitmask: std::u64::MAX,
                };

                // Look for coordinates which belong to the same equivalence class (i.e. are equal
                // to the representant under some symmety)
                // Since this only looks at later coordiantes, the representant is always the
                // smallest
                for j in i + 1..S::BaseCoord::COUNT {
                    if let Some(coord) = coord_values[j] {
                        // Remove the coordinate if it belongs to this equivalence class
                        if let Some(sym) =
                            sym_equivalent::<S::Symmetry, S::BaseCoord>(repr_coord, coord)
                        {
                            coord_values[j] = None;
                            // Mask out the symmetry
                            entry.symmetry_bitmask &= !(1 << sym.index());
                            break;
                        }
                    }
                }

                table.push(entry);
            }
        }

        Self {
            table: table.into_boxed_slice(),
            _sym: PhantomData::default(),
        }
    }

    /// Find the equivalence class of a coordinate and the symmetry which takes it to the
    /// representant of the class.
    pub fn get_equiv_class(&self, coord: S::BaseCoord) -> (S::Symmetry, S::EquivClass) {
        let perm = coord.into_perm();
        self.table
            .iter()
            .zip(S::EquivClass::iter())
            .find_map(|(representant, index)| {
                sym_equivalent_perm(representant.coord, &perm).map(|sym| (sym, index))
            })
            .unwrap() // Will always exist
    }

    /// Get the representant for an equivalence class
    pub fn get_representant(&self, equiv_class: S::EquivClass) -> S::BaseCoord {
        self.table[equiv_class.index()].coord
    }

    /// Returns the class symmetries of an equivalence class.
    ///
    /// These are the symmetries which have no effect when used to conjugate an element in this
    /// equivalence class.
    pub fn get_class_symmetries(&self, equiv_class: S::EquivClass) -> impl Iterator<Item = S> {
        let mut bitmask = self.table[equiv_class.index()].symmetry_bitmask;
        let mut iter = S::iter();
        std::iter::from_fn(move || {
            if bitmask > 0 {
                // Trailing zeros is how many to skip until the next one
                let n = bitmask.trailing_zeros();
                bitmask >>= n + 1;
                // Consumes n and returns and consumes the next
                iter.nth(n as usize)
            } else {
                None
            }
        })
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = S::BaseCoord> + 'a {
        self.table.iter().map(|e| e.coord)
    }
}
