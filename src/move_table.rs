//! Move tables are large tables storing the coordinate transformations for a
//! set of generating moves.
//!
//! This module allows constructing them automatically based on the `Coord` and
//! `PuzzleMove` implementations.

use std::marker::PhantomData;

use crate::coord::{CompositeCoord, Coord};
use crate::puzzle::{PuzzleMove, PuzzlePerm};
use crate::symmetry::{SymCoord, Symmetry};
use crate::util::{EnumCount, IntoEnumIterator};

pub trait MoveTable {
    type Puzzle: PuzzlePerm;
    type Coord: Coord<Self::Puzzle>;
    type Move: PuzzleMove<Puzzle = Self::Puzzle>;

    fn get_move(&self, coord: Self::Coord, mov: Self::Move) -> Self::Coord;
}

pub struct BasicMoveTable<C, M> {
    table: Box<[C]>,
    _moves: PhantomData<M>,
}

impl<C, M> BasicMoveTable<C, M>
where
    C: Coord<M::Puzzle>,
    M: PuzzleMove,
{
    /// Create and fill a move table
    pub fn create() -> Self {
        let mut table = Vec::with_capacity(C::COUNT * M::COUNT);

        for coord in C::iter() {
            let perm = coord.into_perm();

            // Apply each of the moves and add resulting coordinate to the table
            for mov in M::iter() {
                let new_perm = perm.sequence(mov.permutation());
                table.push(C::from_perm(&new_perm));
            }
        }

        Self {
            // TODO is there a way to create a Box<[T]> directly?
            // Is that worth doing anyway, since we would have to deal with
            // initializing all of the elements?
            table: table.into_boxed_slice(),
            _moves: PhantomData::default(),
        }
    }
}

impl<C, M> MoveTable for BasicMoveTable<C, M>
where
    C: Coord<M::Puzzle>,
    M: PuzzleMove,
{
    type Puzzle = M::Puzzle;
    type Coord = C;
    type Move = M;

    fn get_move(&self, coord: C, mov: M) -> C {
        self.table[M::COUNT * coord.index() + mov.index()]
    }
}

/// Move table for composite coordinates which uses a separate table for each sub-coordinate.
///
/// This can be used as-is, or it can be used as a more efficient way of constructing a
/// `BasicMoveTable` for composite coordinates. In the latter case, the "master" table can be
/// constructed from the sub-tables, resulting much fewer permutation and coordinate calculations.
pub struct CompositeMoveTable<'a, C, AT, BT> {
    table_a: &'a AT,
    table_b: &'a BT,
    _coord: PhantomData<C>,
}

impl<'a, C, AT, BT> CompositeMoveTable<'a, C, AT, BT> {
    pub fn new(table_a: &'a AT, table_b: &'a BT) -> Self {
        CompositeMoveTable {
            table_a,
            table_b,
            _coord: PhantomData::default(),
        }
    }
}

impl<'a, P, A, B, C, M, AT, BT> CompositeMoveTable<'a, C, AT, BT>
where
    P: PuzzlePerm,
    M: PuzzleMove<Puzzle = P>,
    AT: MoveTable<Puzzle = P, Coord = A, Move = M>,
    BT: MoveTable<Puzzle = P, Coord = B, Move = M>,
    A: Coord<P>,
    B: Coord<P>,
    C: Coord<P> + CompositeCoord<P, CoordA = A, CoordB = B>,
{
    /// Build a full / flattened move table from a composite move table.
    pub fn to_basic(&self) -> BasicMoveTable<C, M> {
        let mut table = Vec::with_capacity(C::COUNT * M::COUNT);

        for coord in C::iter() {
            // Apply each of the moves and add resulting coordinate to the table
            for mov in M::iter() {
                table.push(self.get_move(coord, mov));
            }
        }

        BasicMoveTable {
            table: table.into_boxed_slice(),
            _moves: PhantomData::default(),
        }
    }
}

impl<'a, P, A, B, C, M, AT, BT> MoveTable for CompositeMoveTable<'a, C, AT, BT>
where
    P: PuzzlePerm,
    M: PuzzleMove<Puzzle = P>,
    AT: MoveTable<Puzzle = P, Coord = A, Move = M>,
    BT: MoveTable<Puzzle = P, Coord = B, Move = M>,
    A: Coord<P>,
    B: Coord<P>,
    C: Coord<P> + CompositeCoord<P, CoordA = A, CoordB = B>,
{
    type Puzzle = P;
    type Coord = C;
    type Move = M;

    fn get_move(&self, coord: C, mov: M) -> C {
        let (a, b) = coord.into_coords();
        C::from_coords(self.table_a.get_move(a, mov), self.table_b.get_move(b, mov))
    }
}

pub struct SymMoveTable<C, M> {
    /// Mapping from (EquivClass, M) -> C (equiv class and symmetry)
    coord_table: Box<[C]>,
    /// Mapping from (M, Symmetry) -> M
    /// TODO Should this be in a separate table?
    move_table: Box<[M]>,
}

impl<C, M> SymMoveTable<C, M>
where
    C: SymCoord<M::Puzzle> + Coord<M::Puzzle>,
    M: PuzzleMove,
{
    pub fn create() -> Self {
        let representants = C::representants();
        let mut coord_table = Vec::with_capacity(representants.len() * M::COUNT);

        for representant in representants.iter() {
            let perm = representant.into_perm();

            // Apply each of the moves and add resulting coordinate to the table
            for mov in M::iter() {
                let new_perm = perm.sequence(mov.permutation());
                coord_table.push(C::from_perm(&new_perm));
            }
        }

        let mut move_table = Vec::with_capacity(M::COUNT * C::Symmetry::COUNT);
        for mov in M::iter() {
            let perm = mov.permutation();

            for sym in C::Symmetry::iter() {
                let transformed = perm.sequence(sym.permutation());
                // Find the move with the right permutation
                if let Some(transformed_move) = M::iter().find(|m| *m.permutation() == transformed)
                {
                    move_table.push(transformed_move);
                } else {
                    panic!("Transformed move coult not be found.");
                }
            }
        }

        Self {
            // TODO is there a way to create a Box<[T]> directly?
            // Is that worth doing anyway, since we would have to deal with
            // initializing all of the elements?
            coord_table: coord_table.into_boxed_slice(),
            move_table: move_table.into_boxed_slice(),
        }
    }
}

impl<C, M> MoveTable for SymMoveTable<C, M>
where
    C: SymCoord<M::Puzzle> + Coord<M::Puzzle>,
    M: PuzzleMove,
{
    type Puzzle = M::Puzzle;
    type Coord = C;
    type Move = M;

    fn get_move(&self, coord: C, mov: M) -> C {

    }
}

/* TODO Move into cube-specific mod
#[cfg(test)]
pub(crate) mod test {
    use super::*;

    use std::fmt::Debug;

    use crate::coord::{CornerOrient7Coord, CornerPos7Coord};
    use crate::cube::moves::UrfTurn;

    fn coordinates_correct_after_move<C: Coord + Debug, M: PuzzleMove>(
        table: &impl MoveTable<C, M>,
    ) {
        let mut perm = PuzzlePerm::default();

        // Run through a series of moves and make sure the coordinates match up
        for turn in M::iter() {
            let orig_coord = C::from(&perm);
            perm += turn.permutation();
            let table_coord = table.get_move(orig_coord, turn);
            let perm_coord = C::from(&perm);

            assert_eq!(table_coord, perm_coord);
        }
    }

    // Do the tests for each coordinate

    macro_rules! make_tests {
        ($name:ident, $coord:ty) => {
            pub mod $name {
                use super::*;
                lazy_static! {
                    pub static ref TABLE: BasicMoveTable<$coord, UrfTurn> =
                        BasicMoveTable::create();
                }

                #[test]
                fn coordinates_correct_after_move() {
                    super::coordinates_correct_after_move(&*TABLE);
                }
            }
        };
    }

    make_tests!(corner_orient, CornerOrient7Coord);
    make_tests!(corner_pos, CornerPos7Coord);

    lazy_static! {
        pub static ref CORNER_MOVE_TABLE: CompositeMoveTable<
            'static,
            CornerOrient7Coord,
            CornerPos7Coord,
            UrfTurn,
            BasicMoveTable<CornerOrient7Coord, UrfTurn>,
            BasicMoveTable<CornerPos7Coord, UrfTurn>,
        > = CompositeMoveTable::new(&*corner_orient::TABLE, &*corner_pos::TABLE);
    }
}
*/
