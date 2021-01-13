//! 2x2x2 cube puzzle notation

use std::str::FromStr;

use num_traits::PrimInt;

use crate::puzzle::PuzzlePerm;

// These just implement Display and FromStr, they don't have exports
mod parser;
mod printer;

/// Trait for primitive moves in a puzzle notation.
pub trait NotationPrim: Copy + Eq + FromStr + ToString {
    type Puzzle: PuzzlePerm;

    fn permutation(&self) -> &Self::Puzzle;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotationMove<M: NotationPrim> {
    prim: M,
    count: i8,
}

impl<M: NotationPrim> NotationMove<M> {
    pub fn ntimes<T: PrimInt>(prim: M, n: T) -> Self {
        NotationMove {
            prim,
            count: n.to_i8().unwrap(),
        }
    }

    pub fn basic(prim: M) -> Self {
        Self::ntimes(prim, 1)
    }

    pub fn inverse(prim: M) -> Self {
        Self::ntimes(prim, -1)
    }

    pub fn permutation(&self) -> M::Puzzle {
        self.prim.permutation().ntimes(self.count)
    }

    /* TODO Add back simplification and cannonicalization?

    /// Tries to combine this move with another one.
    ///
    /// Returns a "nested" option as there are 3 possibilities:
    ///
    /// - The moves cannot combine (`None`)
    /// - The moves combine into a single move (`Some(Some(move))`)
    /// - The moves cancel (`Some(None)`)
    pub fn combine_with(&self, other: &Self) -> Option<Option<Self>> {
        if self.prim == other.prim {
            let new_count = self.count + other.count;
            if new_count == 0 {
                Some(None)
            } else {
                Some(Some(Self::ntimes(self.prim, new_count)))
            }
        } else {
            None
        }
    }
    */
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotationStr<M: NotationPrim> {
    moves: Vec<NotationMove<M>>,
}

impl<M: NotationPrim> Default for NotationStr<M> {
    fn default() -> Self {
        NotationStr { moves: vec![] }
    }
}

impl<M: NotationPrim> From<Vec<NotationMove<M>>> for NotationStr<M> {
    fn from(vec: Vec<NotationMove<M>>) -> Self {
        NotationStr { moves: vec }
    }
}

impl<M: NotationPrim> NotationStr<M> {
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn permutation(&self) -> M::Puzzle {
        self.moves
            .iter()
            .fold(M::Puzzle::identity().clone(), |perm, m| {
                perm.sequence(&m.permutation())
            })
    }

    /*
    pub fn normalize(&self) -> Self {
        // This will generally not be much smaller than the original (and can't
        // be larger)
        let mut moves = Vec::with_capacity(self.moves.len());
        // Buffer to combine and order commutative moves before pushing them
        // into the main move list. In most "real-world" cases, this will only
        // store a single move.
        let mut buffer = Vec::with_capacity(1);

        for m in &self.moves {
            // Look for a move it combines with and the index of that move
            let combined = buffer
                .iter()
                .enumerate()
                .find_map(|(index, bm)| m.combine_with(bm).map(|combined| (index, combined)));
            match combined {
                Some((index, Some(single_move))) => {
                    // Replace with the single move
                    buffer[index] = single_move;
                }
                Some((index, None)) => {
                    // Remove the original since it canceled with the new move
                    buffer.remove(index);
                }
                None => {
                    // If the new move doesn't commute with everything in the
                    // buffer, commit the moves in the buffer
                    if !buffer.iter().all(|bm| m.commutes_with(bm)) {
                        buffer.sort_unstable();
                        moves.append(&mut buffer); // Also clears the buffer
                    }

                    buffer.push(*m);
                }
            }
        }

        buffer.sort_unstable();
        moves.append(&mut buffer);
        moves.shrink_to_fit();
        NotationStr { moves: moves }
    }
    */
}

/* TODO Implement better mock datatype
#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TestPrim {
        A,
        B,
        C,
        D,
    }

    impl FromStr for TestPrim {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, ()> {
            use TestPrim::*;
            match s {
                "A" => Ok(A),
                "B" => Ok(B),
                "C" => Ok(C),
                "D" => Ok(D),
                _ => Err(()),
            }
        }
    }

    impl ToString for TestPrim {
        fn to_string(&self) -> String {
            use TestPrim::*;
            match self {
                A => "A",
                B => "B",
                C => "C",
                D => "D",
            }
            .to_string()
        }
    }

    impl NotationPrim for TestPrim {
        fn permutation(&self) -> &PuzzlePerm {

        }
    }

    /*

    /// Macro for building `NotationStr`s.
    ///
    /// Moves are separated by commas and the following prefixes can be used:
    /// - `-`: Inverts the primitive move
    /// - `2`: Doubles the primitive move
    #[macro_export]
    macro_rules! notation {
        (@accum () -> ($($acc:tt)*)) => {NotationStr::from(vec![ $($acc)* ])};

        // TODO Is there a way to support an optional trailing comma without
        // repeating the rules?

        (@accum (- $m:expr) -> ($($acc:tt)*)) => {
            notation!(@accum () -> ($($acc)* NotationMove::inverse($m),))
        };
        (@accum ($n:literal $m:expr) -> ($($acc:tt)*)) => {
            notation!(@accum () -> ($($acc)* NotationMove::ntimes($m, $n),))
        };
        (@accum ($m:expr) -> ($($acc:tt)*)) => {
            notation!(@accum () -> ($($acc)* NotationMove::basic($m),))
        };

        (@accum (- $m:expr, $($rest:tt)*) -> ($($acc:tt)*)) => {
            notation!(@accum ($($rest)*) -> ($($acc)* NotationMove::inverse($m),))
        };
        (@accum ($n:literal $m:expr, $($rest:tt)*) -> ($($acc:tt)*)) => {
            notation!(@accum ($($rest)*) -> ($($acc)* NotationMove::ntimes($m, $n),))
        };
        (@accum ($m:expr, $($rest:tt)*) -> ($($acc:tt)*)) => {
            notation!(@accum ($($rest)*) -> ($($acc)* NotationMove::basic($m),))
        };

        ($($code:tt)*) => {notation!(@accum ($($code)*) -> ())};
    }

    #[test]
    fn normalize_orders_commutative_moves() {
        assert_eq!(notation!(A, B).normalize(), notation!(A, B));
        assert_eq!(notation!(D, C).normalize(), notation!(C, D));
    }

    #[test]
    fn normalize_combines_single_moves() {
        assert_eq!(notation!(C, C).normalize(), notation!(2 C));
        assert_eq!(notation!(-B, -B).normalize(), notation!(2 B));
    }

    #[test]
    fn normalize_cancels_moves() {
        assert_eq!(notation!(A, -A).normalize(), notation!());
        assert_eq!(notation!(-D, D).normalize(), notation!());
        assert_eq!(notation!(2 B, 2 B).normalize(), notation!());
    }

    #[test]
    fn normalize_combines_with_commutor_in_between() {
        assert_eq!(notation!(B, A, B).normalize(), notation!(A, 2 B));
    }

    */

    /*
    // TODO This isn't currently done

    #[test]
    fn normalize_combines_with_cancel_in_between() {
        assert_eq!(notation!(-B, C, -C, 2 B).normalize(), notation!(B));
    }
    */
}

*/
