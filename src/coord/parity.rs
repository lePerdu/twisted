//! Helper functions for defining coordinates defined by a set of identical, independent values with
//! a whole-puzzle parity.

use num_traits::PrimInt;

use crate::util::EnumIndex;

/// Calculates a coordinate from a set of independent values, all in the range from `[0, base)`.
///
/// It is assumed that the whole puzzle has a fixed parity, so the last item is excluded from the
/// coordinate calculation.
pub fn calculate_coord<C, T, I>(items: I) -> C
where
    C: PrimInt,
    T: EnumIndex,
    I: Iterator<Item = T> + DoubleEndedIterator,
{
    let mut c = 0;
    for item in items.rev().skip(1) {
        c = c * T::COUNT + item.index();
    }

    C::from(c).unwrap()
}

/// Extracts values from a coordinate built with `coord()`.
///
/// Values are returned in order, with the last one set so that the parity of all of them is equal
/// to `parity`.
pub fn extract_from_coord<C, T, Idx, I, M>(
    coord: C,
    parity: C,
    all_items: I,
    mapper: M,
) -> impl Iterator<Item = (Idx, T)>
where
    C: PrimInt,
    T: EnumIndex,
    Idx: EnumIndex,
    I: Iterator<Item = Idx> + ExactSizeIterator,
    M: Fn(C) -> T,
{
    let base = C::from(T::COUNT).unwrap();
    let mut coord = coord;
    let mut sum = C::zero();
    let mut all_items = all_items;

    std::iter::from_fn(move || {
        all_items.next().map(|item| {
            let val = {
                // Calculate the last one from the parity
                if all_items.len() == 0 {
                    (parity + base - sum) % base
                } else {
                    // Divide by the base repeatedly to extract the orientations
                    let val = coord % base;
                    coord = coord / base;
                    sum = (sum + val) % base;

                    val
                }
            };

            (item, mapper(val))
        })
    })
}
