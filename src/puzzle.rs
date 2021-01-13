//! Generic interface for puzzles.
//!

use num_traits::PrimInt;

use crate::util::EnumIndex;

/// A "twisty puzzle".
///
/// This trait essentially defines a Group, where "sequence" is the group operation.
///
/// TODO Use operator overloads?
pub trait PuzzlePerm: 'static + Clone + Eq {
    /// Identity (no-op) permutation on the puzzle.
    fn identity() -> &'static Self;

    /// Combine 2 permutations by sequencing them.
    fn sequence(&self, other: &Self) -> Self;

    /// Invert a permutation.
    fn invert(&self) -> Self;

    /// Tests whether a permutation is the identity.
    fn is_identity(&self) -> bool {
        self == Self::identity()
    }

    /// Sequence a permutation multiple times with itself.
    ///
    /// Can be given a custom implementation if there is an efficient, puzzle-specific way to
    /// sequence a permutation multiple times.
    fn ntimes<T: PrimInt>(&self, n: T) -> Self {
        let mut n = n;
        if n > T::zero() {
            let mut res = self.clone();
            while n > T::one() {
                res = res.sequence(self);
                n = n - T::one();
            }

            res
        } else if n < T::zero() {
            let inv = self.invert();
            let mut res = inv.clone();
            while n < T::zero() - T::one() {
                res = res.sequence(&inv);
                n = n + T::one();
            }

            res
        } else {
            Self::identity().clone()
        }
    }
}

/// Representation of a set of generators to use in a `MoveTable`
///
/// TODO Make the type parameter an associated type instead? It would is theoretically possible that
/// multiple puzzles have the same move sets (e.g. 3x3 and 2x2), but it might not be that
/// beneficial to unify the move types.
pub trait PuzzleMove: EnumIndex {
    type Puzzle: PuzzlePerm;

    /// Permutation associated with this move
    fn permutation(&self) -> &Self::Puzzle;

    /// Returns whether two moves can combine with each other to produde another move.
    ///
    /// This is used to trim down search trees.
    fn combines_with(&self, other: &Self) -> bool;
}

/// Implement `PuzzlePerm` for a type by making lazily-evaluated tables for combining and inverting
/// permutations.
///
/// Takes a closure which returns the identity and a function which returns some other type which
/// implements `PuzzlePerm` associawith the implementing type.
///
/// Note: this should only be used when there are not very many values for the type, as the sequence
/// tables containts `COUNT * COUNT` elements.
#[macro_export]
macro_rules! impl_puzzle_perm_with_tables {
    ($impl_ty:ty, $identity_closure:expr, $prem_closure:expr) => {
        impl $crate::puzzle::PuzzlePerm for $impl_ty {
            fn identity() -> &'static Self {
                $identity_closure()
            }

            fn invert(&self) -> Self {
                use $crate::util::{EnumCount, EnumIndex, IntoEnumIterator};
                lazy_static! {
                    static ref TABLE: [$impl_ty; <$impl_ty>::COUNT] = {
                        let mut table: [$impl_ty; <$impl_ty>::COUNT] = Default::default();

                        for sym in <$impl_ty>::iter() {
                            let perm = ($prem_closure)(&sym);
                            for other in <$impl_ty>::iter() {
                                if perm.sequence(($prem_closure)(&other)).is_identity() {
                                    table[sym.index()] = other;
                                }
                            }
                        }

                        table
                    };
                }

                TABLE[self.index()]
            }

            fn sequence(&self, other: &Self) -> Self {
                use $crate::util::{EnumCount, EnumIndex, IntoEnumIterator};
                lazy_static! {
                    static ref TABLE: [[$impl_ty; <$impl_ty>::COUNT]; <$impl_ty>::COUNT] = {
                        let mut table: [[$impl_ty; <$impl_ty>::COUNT]; <$impl_ty>::COUNT] =
                            Default::default();

                        for sym in <$impl_ty>::iter() {
                            let perm = ($prem_closure)(&sym);
                            for other in <$impl_ty>::iter() {
                                let result = perm.sequence(($prem_closure)(&other));

                                for result_sym in <$impl_ty>::iter() {
                                    if *($prem_closure)(&result_sym) == result {
                                        table[sym.index()][other.index()] = result_sym;
                                    }
                                }
                            }
                        }

                        table
                    };
                }

                TABLE[self.index()][other.index()]
            }
        }
    };
}
