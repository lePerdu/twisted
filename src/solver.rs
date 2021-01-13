//! IDA* solving algorithm using move tables

use crate::coord::Coord;
use crate::move_table::MoveTable;
use crate::prune_table::PruneTable;
use crate::puzzle::PuzzleMove;
use crate::util::IntoEnumIterator;

#[derive(Clone, Debug)]
struct StackState<C, M: IntoEnumIterator> {
    coord: C,
    mov: M,
    move_iter: M::Iterator,
}

impl<C, M: IntoEnumIterator> StackState<C, M> {
    fn new(coord: C, mov: M) -> Self {
        Self {
            coord,
            mov,
            move_iter: M::iter(),
        }
    }
}

/// Iterator producing puzzle solutions using IDA*.
///
/// Solutions are returned in order by length, shortest first (however solutions of equal length
/// have no specified order).
pub struct SolutionIter<'a, MT, PT>
where
    MT: MoveTable,
    PT: PruneTable<Puzzle = MT::Puzzle, Coord = MT::Coord, Move = MT::Move>,
{
    move_table: &'a MT,
    prune_table: &'a PT,
    target: MT::Coord,
    init_coord: MT::Coord,
    max_depth: u32,
    next_max_depth: u32,
    first_move_iter: <MT::Move as IntoEnumIterator>::Iterator,
    stack: Vec<StackState<MT::Coord, MT::Move>>,
}

fn consume_iter<I: Iterator>(mut iter: I) -> I {
    iter.find(|_| false);
    iter
}

impl<'a, MT, PT> SolutionIter<'a, MT, PT>
where
    MT: MoveTable,
    PT: PruneTable<Puzzle = MT::Puzzle, Coord = MT::Coord, Move = MT::Move>,
{
    pub fn new(
        move_table: &'a MT,
        prune_table: &'a PT,
        target: MT::Coord,
        perm: &MT::Puzzle,
    ) -> Self {
        Self {
            move_table,
            prune_table,
            target,
            init_coord: MT::Coord::from_perm(perm),
            max_depth: 0,
            next_max_depth: 1,
            // Consume this at first so that the 0-depth solution (i.e. initial permutation is
            // already at the target) will be chedked.
            first_move_iter: consume_iter(MT::Move::iter()),
            stack: Vec::new(),
        }
    }
}

impl<'a, MT, PT> Iterator for SolutionIter<'a, MT, PT>
where
    MT: MoveTable,
    PT: PruneTable<Puzzle = MT::Puzzle, Coord = MT::Coord, Move = MT::Move>,
{
    type Item = Vec<MT::Move>;

    fn next(&mut self) -> Option<Self::Item> {
        // Keep trying until a solution is found
        loop {
            let next_state = self.stack.last_mut().map(|state| {
                let next_move = state.move_iter.next();
                next_move.map(|m| (state.coord, state.mov, m))
            });

            match next_state {
                Some(Some((coord, this_move, next_move))) => {
                    // If the new move combines with the old one, there is another (single) move which
                    // is their combination
                    if !this_move.combines_with(&next_move) {
                        let new_coord = self.move_table.get_move(coord, next_move);

                        // Current depth is stack.len(), but we want the depth after adding another move
                        let depth = self.stack.len() as u32 + 1;
                        let heuristic_depth = self.prune_table.get_min_moves(new_coord) + depth;
                        if heuristic_depth <= self.max_depth {
                            // Add a new element for this move and keep going
                            self.stack.push(StackState::new(new_coord, next_move));

                            // Only return a solution if it is for the current depth (otherwise it has
                            // been returned before)
                            // TODO This will fail to return solutions if the heuristic is not a strict
                            // lower bound
                            if depth == self.max_depth && new_coord == self.target {
                                return Some(self.stack.iter().map(|state| state.mov).collect());
                            }
                        } else {
                            // Next max depth is the smallest estimated depth after the current max
                            // depth that will have a solution.
                            self.next_max_depth =
                                std::cmp::min(self.next_max_depth, heuristic_depth);
                        }
                    }
                }
                Some(None) => {
                    // No more moves in this state, so pop it off the stack
                    self.stack.pop();
                }
                None => {
                    // Stack is empty
                    match self.first_move_iter.next() {
                        Some(first_move) => {
                            let new_coord = self.move_table.get_move(self.init_coord, first_move);
                            self.stack.push(StackState::new(new_coord, first_move));

                            // Only return a solution if it is for the current depth (otherwise it has
                            // been returned before)
                            // TODO Change this if the maximum depth does not always increase by 1
                            if self.max_depth == 1 && new_coord == self.target {
                                return Some(self.stack.iter().map(|state| state.mov).collect());
                            }
                        }
                        None => {
                            // No more moves to try, so increase the depth
                            self.max_depth = self.next_max_depth;
                            self.next_max_depth += 1;
                            self.first_move_iter = MT::Move::iter();

                            if self.max_depth == 1 {
                                // If this is the first iteration, check if the initial state is already
                                // at the target
                                if self.init_coord == self.target {
                                    return Some(Vec::new());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn solve_cube<MT, PT>(
    move_table: &MT,
    prune_table: &PT,
    perm: &MT::Puzzle,
    target: MT::Coord,
) -> Option<Vec<MT::Move>>
where
    MT: MoveTable,
    PT: PruneTable<Puzzle = MT::Puzzle, Coord = MT::Coord, Move = MT::Move>,
{
    fn depth_search<MT, PT>(
        move_table: &MT,
        prune_table: &PT,
        depth: u32,
        path: Vec<MT::Move>,
        coord: MT::Coord,
        target: MT::Coord,
    ) -> (bool, Vec<MT::Move>)
    where
        MT: MoveTable,
        PT: PruneTable<Puzzle = MT::Puzzle, Coord = MT::Coord, Move = MT::Move>,
    {
        if coord == target {
            return (true, path);
        }

        if depth == 0 {
            return (false, path);
        }

        let mut path = path;

        let last_move = path.last().copied();
        let new_moves = MT::Move::iter().filter(|m| match last_move {
            Some(last) => !last.combines_with(m),
            None => true,
        });

        for m in new_moves {
            let new_coord = move_table.get_move(coord, m);

            // Skip if the expected distance is too big
            if prune_table.get_min_moves(new_coord) as u32 > depth {
                continue;
            }

            path.push(m);

            let (found, new_path) =
                depth_search(move_table, prune_table, depth - 1, path, new_coord, target);
            if found {
                return (found, new_path);
            }

            path = new_path;
            path.pop();
        }

        (false, path)
    }

    let root_coord = MT::Coord::from_perm(perm);
    for depth in 1.. {
        let (found, path) = depth_search(
            move_table,
            prune_table,
            depth,
            Vec::new(),
            root_coord,
            target,
        );

        if found {
            return Some(path);
        }
    }

    None
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::cube::coord::Corner7Coord;
    use crate::cube::moves::UrfTurn;
    use crate::cube::primitives;
    use crate::move_table::test::CORNER_MOVE_TABLE;
    use crate::prune_table::test::CORNER_PRUNE_TABLE;

    fn solve(perm: &PuzzlePerm) -> Option<Vec<UrfTurn>> {
        solve_cube(
            &*CORNER_MOVE_TABLE,
            &*CORNER_PRUNE_TABLE,
            perm,
            Corner7Coord::default(),
        )
    }

    #[test]
    fn solves_solved_cube() {
        assert_eq!(solve(primitives::identity()), Some(Vec::new()));
    }

    // TODO Make these more robust?

    #[test]
    fn solves_with_one_move() {
        assert_eq!(solve(primitives::u()), Some(vec![UrfTurn::UP]));
    }

    #[test]
    fn solves_with_two_moves() {
        let perm = primitives::u() + primitives::r_prime();
        assert_eq!(solve(&perm), Some(vec![UrfTurn::R, UrfTurn::UP]));
    }
}
*/
