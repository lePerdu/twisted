use std::fmt::{self, Write};

use super::{NotationMove, NotationPrim, NotationStr};

impl<M: NotationPrim> fmt::Display for NotationMove<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.prim.to_string())?;

        // Only show number if not 1
        let abs = self.count.abs();
        if abs != 1 {
            write!(f, "{}", abs)?;
        }

        // Show ' for inverses
        if self.count.is_negative() {
            f.write_char('\'')?;
        }

        Ok(())
    }
}

impl<M: NotationPrim> fmt::Display for NotationStr<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            // Print first separately so spaces can be put in between
            write!(f, "{}", self.moves[0])?;

            for m in &self.moves[1..] {
                write!(f, " {}", m)?;
            }
        }

        Ok(())
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    use super::super::test::TestPrim;

    #[test]
    fn prints_basic_move() {
        assert_eq!(format!("{}", NotationMove::basic(TestPrim::A)), "A");
    }

    #[test]
    fn prints_inverse_move() {
        assert_eq!(format!("{}", NotationMove::inverse(TestPrim::B)), "B'");
    }

    #[test]
    fn prints_ntimes_move() {
        assert_eq!(format!("{}", NotationMove::ntimes(TestPrim::C, 3)), "C3");
    }

    #[test]
    fn prints_ntimes_inverse_move() {
        assert_eq!(format!("{}", NotationMove::ntimes(TestPrim::D, -3)), "D3'");
    }

    #[test]
    fn prints_notation_str() {
        use TestPrim::*;
        assert_eq!(
            format!(
                "{}",
                NotationStr::from(vec![
                    NotationMove::basic(A),
                    NotationMove::inverse(B),
                    NotationMove::ntimes(C, 3),
                    NotationMove::ntimes(D, -2)
                ])
            ),
            "A B' C3 D2'"
        );
    }
}
*/
