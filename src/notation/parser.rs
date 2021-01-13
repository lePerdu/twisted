//! Generic puzzle notation parser.

use std::str::FromStr;

// TODO Use parsing framework? (even though this is very light-weight parsing, it is a bit verbose)

use super::{NotationMove, NotationPrim, NotationStr};

type ParseState<'a> = &'a str;

// TODO Implement actual errors
type ParseErr = ();

type Result<T> = std::result::Result<T, ParseErr>;

fn parse_prim<M: NotationPrim>(s: ParseState) -> Result<(M, ParseState)> {
    // Read in leading alpha-numeric characters from input and compare against that
    // Find first non-alpha character and split there
    let alpha_len = s
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(s.len());
    let (ident, s) = s.split_at(alpha_len);

    M::from_str(ident).map(|prim| (prim, s)).map_err(|_| ())
}

fn parse_num(s: ParseState) -> Result<(Option<u8>, ParseState)> {
    let (n, len) = s
        .chars()
        .take_while(|c| c.is_digit(10))
        .fold((0, 0), |(n, count), digit| {
            // Return the number and how many digits were consumed
            // Digit will always exist because of the take_while
            let digit_val = digit.to_digit(10).unwrap();
            (n * 10 + digit_val, count + 1)
        });

    if len == 0 {
        Ok((None, s))
    } else if n > 0 {
        // TODO Check overflow
        Ok((Some(n as u8), &s[len..]))
    } else {
        Err(())
    }
}

fn parse_prime(s: ParseState) -> Result<(bool, ParseState)> {
    if s.chars().next() == Some('\'') {
        Ok((true, &s[1..]))
    } else {
        Ok((false, s))
    }
}

fn parse_count(s: ParseState) -> Result<(i8, ParseState)> {
    let (opt_n, s) = parse_num(s)?;
    // TODO Check overflow
    let n = opt_n.unwrap_or(1) as i8;

    let (inverse, s) = parse_prime(s)?;

    let n = if inverse { -n } else { n };

    Ok((n, s))
}

fn parse_move<M: NotationPrim>(s: ParseState) -> Result<(NotationMove<M>, ParseState)> {
    let (prim, s) = parse_prim(s)?;
    let (count, s) = parse_count(s)?;

    Ok((NotationMove::ntimes(prim, count), s))
}

fn parse_move_full<M: NotationPrim>(s: ParseState) -> Result<NotationMove<M>> {
    let (m, s) = parse_move(s)?;
    if s.is_empty() {
        Ok(m)
    } else {
        Err(())
    }
}

/// Parses a string notation into move notation.
///
/// The notation string can be padded with whitespace.
fn parse_notation<M: NotationPrim>(s: ParseState) -> Result<NotationStr<M>> {
    s.split_whitespace()
        .map(|move_str| parse_move_full(move_str))
        .try_fold(Vec::new(), |moves, m| {
            let mut moves = moves;
            m.map(move |m| {
                moves.push(m);
                moves
            })
        })
        .map(NotationStr::from)
}

impl<M: NotationPrim> FromStr for NotationMove<M> {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self> {
        parse_move_full(s)
    }
}

impl<M: NotationPrim> FromStr for NotationStr<M> {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self> {
        parse_notation(s)
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    use super::super::test::TestPrim;

    #[test]
    fn parses_basic_move() {
        assert_eq!(
            NotationMove::from_str("A"),
            Ok(NotationMove::basic(TestPrim::A))
        );
    }

    #[test]
    fn parses_inverse_move() {
        assert_eq!(
            NotationMove::from_str("B'"),
            Ok(NotationMove::inverse(TestPrim::B))
        );
    }

    #[test]
    fn parses_ntimes_move() {
        assert_eq!(
            NotationMove::from_str("C3"),
            Ok(NotationMove::ntimes(TestPrim::C, 3))
        );
    }

    #[test]
    fn parses_ntimes_inverse_move() {
        assert_eq!(
            NotationMove::from_str("D3'"),
            Ok(NotationMove::ntimes(TestPrim::D, -3))
        );
    }

    #[test]
    fn parses_notation_str() {
        use TestPrim::*;
        assert_eq!(
            NotationStr::from_str(" A\tB' C3   D2'  "),
            Ok(NotationStr::from(vec![
                NotationMove::basic(A),
                NotationMove::inverse(B),
                NotationMove::ntimes(C, 3),
                NotationMove::ntimes(D, -2)
            ]))
        );
    }

    // Test errors

    #[test]
    fn fails_on_invalid_move() {
        assert_eq!(NotationMove::<TestPrim>::from_str("Q"), Err(()));
    }

    #[test]
    fn fails_on_zero_count() {
        assert_eq!(NotationMove::<TestPrim>::from_str("A0"), Err(()));
    }

    #[test]
    fn fails_on_text_after_move() {
        assert_eq!(NotationMove::<TestPrim>::from_str("A2A"), Err(()));
    }

    #[test]
    fn fails_on_notation_str_with_invalid_move() {
        assert_eq!(NotationStr::<TestPrim>::from_str("A B' Q3 D2'"), Err(()));
    }
}
*/
