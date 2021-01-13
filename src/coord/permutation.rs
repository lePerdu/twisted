//! Helpers for defining coordinates defined by a permutation of items.

use num_traits::PrimInt;

use crate::util::{rotate_left, rotate_right};

pub fn calculate_coord<C, T, I1, I2>(items_in_order: I1, items: I2) -> C
where
    C: PrimInt,
    T: Copy + Eq,
    I1: Iterator<Item = T> + DoubleEndedIterator + ExactSizeIterator,
    I2: Iterator<Item = T>,
{
    let mut c = C::zero();
    let mut items: Vec<T> = items.collect();

    // Go in reverse order because it's easier to compute with factorial
    // base as (l_1 + (l_2 + (... ) * 2) * 1
    for (index, item) in items_in_order.enumerate().skip(1).rev() {
        // Rotate left until the correct corner
        let mut rot_count = C::zero();
        while items[index] != item {
            rotate_left(&mut items[..=index]);
            rot_count = rot_count + C::one();

            assert!(
                rot_count.to_usize().unwrap() < items.len(),
                "Item not found in permutation"
            );
        }

        c = (c + rot_count) * C::from(index).unwrap()
    }

    c
}

pub fn apply_coord<C, T, P>(coord: C, items_in_order: impl Iterator<Item = T>, items: &mut [P])
where
    C: PrimInt,
    T: Copy + Eq,
    P: Copy,
{
    let mut coord = coord;

    for (index, _) in items_in_order.enumerate().skip(1) {
        let base = C::from(index).unwrap() + C::one();
        let rotations = (coord % base).to_usize().unwrap();
        coord = coord / base;

        // Rotate right the extracted number of times
        for _ in 0..rotations {
            rotate_right(&mut items[..=index]);
        }
    }
}
