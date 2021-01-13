pub use strum::{EnumCount, IntoEnumIterator};

// TODO Make derive impl for this
// TODO Make it Into<usize> instead?

/// Trait for types which act as bounds-restricted indices into tables.
///
/// Provides an iterator over all values, conversion into index (usize), and an element count.
pub trait EnumIndex: 'static + Copy + Eq + EnumCount + IntoEnumIterator {
    /// Returns the index of this element.
    ///
    /// The index MUST be the same as the order of the item in its `IntoEnumIterator` implementation
    /// (and hence must start from 0 and end at `IntoEnumIterator::VALUE_COUNT`).
    fn index(&self) -> usize;
}

pub fn rotate_left<T: Copy>(slice: &mut [T]) {
    let len = slice.len();
    let first = slice[0];
    for i in 1..len {
        slice[i - 1] = slice[i];
    }

    slice[len - 1] = first;
}

pub fn rotate_right<T: Copy>(slice: &mut [T]) {
    let len = slice.len();
    let last = slice[len - 1];
    for i in (1..len).rev() {
        slice[i] = slice[i - 1];
    }

    slice[0] = last;
}

// TODO Make this a derive macro instead?
#[macro_export]
macro_rules! make_newtype_enum_index {
    (
        $( #[ $attrs:meta ] )*
        $v:vis struct $newtype:ident ( $inner:ty ) : $count:expr ;
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
        $( #[$attrs] )*
        $v struct $newtype($inner);

        impl $crate::util::EnumCount for $newtype {
            const COUNT: usize = $count;
        }

        impl $crate::util::IntoEnumIterator for $newtype {
            type Iterator = ::std::iter::Map<::std::ops::Range<usize>, fn(usize) -> $newtype>;

            fn iter() -> Self::Iterator {
                use $crate::util::EnumCount;
                (0..Self::COUNT).map(|n| Self(n as $inner))
            }
        }

        impl $crate::util::EnumIndex for $newtype {
            fn index(&self) -> usize {
                self.0 as usize
            }
        }
    };
}
