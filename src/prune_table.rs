use std::marker::PhantomData;

use crate::coord::{CompositeCoord, Coord};
use crate::move_table::MoveTable;
use crate::puzzle::{PuzzleMove, PuzzlePerm};

pub trait PruneTable {
    /// Puzzle this pruning table is for.
    type Puzzle: PuzzlePerm;

    /// Coordinate or composite coordinate this pruning table is indexed by.
    type Coord: Coord<Self::Puzzle>;

    /// Move set used to count minimum bounds.
    type Move: PuzzleMove<Puzzle = Self::Puzzle>;

    // TODO Make u8? Will it every be larger than that?
    /// Returns a lower bound for the number of moves it will take to get to a particular state.
    fn get_min_moves(&self, coord: Self::Coord) -> u32;
}

/// Pruning table which always returns 0
#[derive(Clone, Debug)]
pub struct ZeroPruneTable<C, M>(PhantomData<(C, M)>);

impl<C, M> Default for ZeroPruneTable<C, M> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<P, C, M> PruneTable for ZeroPruneTable<C, M>
where
    P: PuzzlePerm,
    C: Coord<P>,
    M: PuzzleMove<Puzzle = P>,
{
    type Puzzle = P;
    type Coord = C;
    type Move = M;

    fn get_min_moves(&self, _coord: Self::Coord) -> u32 {
        0
    }
}

/// An exact-valued pruning table calculated from a coordinate move table.
///
/// This can be used with any coordinate, including composite coordinates, that has a move table.
/// Note that for a composite coordinate with sub-coordinate sizes `A` and `B`, this pruning table
/// will have size `A*B`. If that is not desired, pruning tables should be made individually for
/// each of the sub-coordinates and combined with `CompositePruneTable`.
pub struct FullPruneTable<C, M> {
    table: Box<[u8]>,
    _phantoms: PhantomData<(C, M)>,
}

impl<P, C, M> PruneTable for FullPruneTable<C, M>
where
    P: PuzzlePerm,
    C: Coord<P>,
    M: PuzzleMove<Puzzle = P>,
{
    type Puzzle = P;
    type Coord = C;
    type Move = M;

    fn get_min_moves(&self, coord: C) -> u32 {
        self.table[coord.index()] as u32
    }
}

impl<P, C, M> FullPruneTable<C, M>
where
    P: PuzzlePerm,
    C: Coord<P>,
    M: PuzzleMove<Puzzle = P>,
{
    /// Create a pruning table from a move table.
    ///
    /// The resulting pruning table will have the same number of values as the coordinate `C`.
    ///
    /// TODO Allow multiple targets
    pub fn create<MT>(move_table: &MT, target: C) -> Self
    where
        MT: MoveTable<Puzzle = P, Coord = C, Move = M>,
    {
        info!("Building pruning table");

        // Initialize with max to indicate unfilled
        let mut table = vec![std::u8::MAX; C::COUNT].into_boxed_slice();

        // Fill in the target
        table[target.index()] = 0;

        let mut remaining = C::COUNT - 1;
        let mut n = 0;

        // The table is filled with 2 methods:
        // - Look for elements with a distance of n, apply all moves to those, and fill in the
        //   resulting coordinates with n + 1 if they have not already been filled. ("forward
        //   search")
        // - Look for unfilled elements, apply all moves to them, and fill in the unfilled element
        //   with n + 1 if one of the moves goes to an element with distance n. ("reverse search")
        //
        // The critical path in both algorithms is applying each move to a coordinate. In the
        // forward search, this is done for entries with a value of n, while in the reverse search
        // it is done for unfilled entries. For this reason, the reverse search is faster when the
        // number of remaining entries is less than the number that were filled in the last
        // iteration.

        // Forward search
        loop {
            info!("Forward search at distance {}", n);

            // Keep track of the number filled to know when to start reverse search
            let mut filled = 0;

            // Look for indices with distance n and apply all moves to them.
            for coord in C::iter() {
                let move_dist = table[coord.index()];
                if move_dist == n {
                    // Apply each move
                    for m in M::iter() {
                        let new_coord = move_table.get_move(coord, m);
                        let new_index = new_coord.index();

                        // Fill in the table if not already filled
                        if table[new_index] == std::u8::MAX {
                            table[new_index] = n + 1;
                            filled += 1;
                        }
                    }
                }
            }

            n += 1;
            info!("{} filled at distance {}", filled, n);
            remaining -= filled;

            // If there are fewer left than were filled in this iteration, the "reverse search" will
            // be more efficient
            // (also exits if remaining == 0)
            if remaining <= filled {
                break;
            }
        }

        // Reverse search
        while remaining > 0 {
            info!("Reverse search at distance {}", n);

            // Keep track of this for logging purposes
            let mut filled = 0;

            for coord in C::iter() {
                let index = coord.index();
                if table[index] == std::u8::MAX {
                    // Apply each move
                    for m in M::iter() {
                        let new_coord = move_table.get_move(coord, m);
                        let new_index = new_coord.index();

                        // Fill in the table if the new coordinate is a distance n
                        if table[new_index] == n {
                            table[index] = n + 1;
                            filled += 1;

                            // Don't need to check any more moves
                            break;
                        }
                    }
                }
            }

            n += 1;
            info!("{} filled at distance {}", filled, n);
            remaining -= filled;
        }

        Self {
            table,
            _phantoms: PhantomData::default(),
        }
    }
}

/// Pruning table for a composite coordinate.
///
/// This hold a pruning table for each sub-coordinate and returns the greatest lower bound between
/// them. One of the sub-coordinate pruning tables can be `ZeroPruneTable` to ignore that coordinate
/// when calculating the lower bound for the move count.
///
/// The `C` type parameter is a coordinate that can be converted into a composite coordinate.
pub struct CompositePruneTable<'a, C, AT, BT> {
    table_a: &'a AT,
    table_b: &'a BT,
    _coord: PhantomData<C>,
}

impl<'a, P, M, A, B, C, AT, BT> CompositePruneTable<'a, C, AT, BT>
where
    P: PuzzlePerm,
    M: PuzzleMove<Puzzle = P>,
    AT: PruneTable<Puzzle = P, Coord = A, Move = M>,
    BT: PruneTable<Puzzle = P, Coord = B, Move = M>,
    A: Coord<P>,
    B: Coord<P>,
    C: Coord<P> + CompositeCoord<P, CoordA = A, CoordB = B>,
{
    pub fn new(table_a: &'a AT, table_b: &'a BT) -> Self {
        Self {
            table_a,
            table_b,
            _coord: PhantomData::default(),
        }
    }
}

impl<'a, P, M, A, B, C, AT, BT> PruneTable for CompositePruneTable<'a, C, AT, BT>
where
    P: PuzzlePerm,
    M: PuzzleMove<Puzzle = P>,
    AT: PruneTable<Puzzle = P, Coord = A, Move = M>,
    BT: PruneTable<Puzzle = P, Coord = B, Move = M>,
    A: Coord<P>,
    B: Coord<P>,
    C: Coord<P> + CompositeCoord<P, CoordA = A, CoordB = B>,
{
    type Puzzle = P;
    type Coord = C;
    type Move = M;

    fn get_min_moves(&self, coord: C) -> u32 {
        let (a, b) = coord.into_coords();
        std::cmp::max(self.table_a.get_min_moves(a), self.table_b.get_min_moves(b))
    }
}

/* TODO Move into cube-specific mod
#[cfg(test)]
pub(crate) mod test {
    use super::*;

    use crate::coord::Corner7Coord;
    use crate::cube::moves::UrfTurn;

    lazy_static! {
        pub static ref CORNER_PRUNE_TABLE: PruneTable<Corner7Coord, UrfTurn> = PruneTable::create(
            &*crate::move_table::test::CORNER_MOVE_TABLE,
            Corner7Coord::default(),
        );
    }

    #[test]
    fn table_is_created() {
        assert_eq!(CORNER_PRUNE_TABLE.table.len(), 3674160);
    }
}
*/
