use crate::puzzle::PuzzlePerm;

use super::super::corner::{Corner, CornerOrient, CornerPerm, CornerPos};

use CornerOrient::*;
use CornerPos::*;

// Primitive moves

const U: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(UFL, Oriented),
        Corner::new(ULB, Oriented),
        Corner::new(UBR, Oriented),
        Corner::new(URF, Oriented),
        Corner::new(DLF, Oriented),
        Corner::new(DFR, Oriented),
        Corner::new(DRB, Oriented),
        Corner::new(DBL, Oriented),
    ],
};

const R: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(ULB, Oriented),
        Corner::new(URF, Clockwise),
        Corner::new(DFR, AntiClockwise),
        Corner::new(UFL, Oriented),
        Corner::new(DLF, Oriented),
        Corner::new(DRB, Clockwise),
        Corner::new(UBR, AntiClockwise),
        Corner::new(DBL, Oriented),
    ],
};

const F: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(ULB, Oriented),
        Corner::new(UBR, Oriented),
        Corner::new(UFL, Clockwise),
        Corner::new(DLF, AntiClockwise),
        Corner::new(DFR, Clockwise),
        Corner::new(URF, AntiClockwise),
        Corner::new(DRB, Oriented),
        Corner::new(DBL, Oriented),
    ],
};

const D: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(ULB, Oriented),
        Corner::new(UBR, Oriented),
        Corner::new(URF, Oriented),
        Corner::new(UFL, Oriented),
        Corner::new(DBL, Oriented),
        Corner::new(DLF, Oriented),
        Corner::new(DFR, Oriented),
        Corner::new(DRB, Oriented),
    ],
};

const L: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(DBL, AntiClockwise),
        Corner::new(UBR, Oriented),
        Corner::new(URF, Oriented),
        Corner::new(ULB, Clockwise),
        Corner::new(UFL, AntiClockwise),
        Corner::new(DFR, Oriented),
        Corner::new(DRB, Oriented),
        Corner::new(DLF, Clockwise),
    ],
};

const B: CornerPerm = CornerPerm {
    cubies: [
        Corner::new(UBR, Clockwise),
        Corner::new(DRB, AntiClockwise),
        Corner::new(URF, Oriented),
        Corner::new(UFL, Oriented),
        Corner::new(DLF, Oriented),
        Corner::new(DFR, Oriented),
        Corner::new(DBL, Clockwise),
        Corner::new(ULB, AntiClockwise),
    ],
};

// Accessor functions for the primitives and their derivatives

pub fn u() -> &'static CornerPerm {
    &U
}

pub fn u_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref U_PRIME: CornerPerm = u().invert();
    }

    &U_PRIME
}

pub fn u2() -> &'static CornerPerm {
    lazy_static! {
        static ref U2: CornerPerm = u().ntimes(2);
    }

    &U2
}

pub fn f() -> &'static CornerPerm {
    &F
}

pub fn f_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref F_PRIME: CornerPerm = f().invert();
    }

    &F_PRIME
}

pub fn f2() -> &'static CornerPerm {
    lazy_static! {
        static ref F2: CornerPerm = f().ntimes(2);
    }

    &F2
}

pub fn r() -> &'static CornerPerm {
    &R
}

pub fn r_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref R_PRIME: CornerPerm = r().invert();
    }

    &R_PRIME
}

pub fn r2() -> &'static CornerPerm {
    lazy_static! {
        static ref R2: CornerPerm = r().ntimes(2);
    }

    &R2
}

pub fn d() -> &'static CornerPerm {
    &D
}

pub fn d_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref D_PRIME: CornerPerm = d().invert();
    }

    &D_PRIME
}

pub fn d2() -> &'static CornerPerm {
    lazy_static! {
        static ref D2: CornerPerm = d().ntimes(2);
    }

    &D2
}

pub fn l() -> &'static CornerPerm {
    &L
}

pub fn l_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref L_PRIME: CornerPerm = l().invert();
    }

    &L_PRIME
}

pub fn l2() -> &'static CornerPerm {
    lazy_static! {
        static ref L2: CornerPerm = l().ntimes(2);
    }

    &L2
}

pub fn b() -> &'static CornerPerm {
    &B
}

pub fn b_prime() -> &'static CornerPerm {
    lazy_static! {
        static ref B_PRIME: CornerPerm = b().invert();
    }

    &B_PRIME
}

pub fn b2() -> &'static CornerPerm {
    lazy_static! {
        static ref B2: CornerPerm = b().ntimes(2);
    }

    &B2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_own_inverse() {
        let def = CornerPerm::identity().invert();
        assert_eq!(&def, CornerPerm::identity());
    }

    /// Checks that the order of a permutation is exactly as specified
    fn has_order(perm: &CornerPerm, order: u32) {
        let mut p = perm.clone();
        for o in 1..order {
            assert_ne!(
                &p,
                CornerPerm::identity(),
                "Order ({}) is less than expected ({})",
                o,
                order
            );
            p = p.sequence(perm);
        }

        assert_eq!(
            &p,
            CornerPerm::identity(),
            "Order is greater than expected ({})",
            order
        );
    }

    mod quarter_turns_have_order_4 {
        use super::*;

        #[test]
        fn u_has_order_4() {
            has_order(u(), 4);
        }

        #[test]
        fn r_has_order_4() {
            has_order(r(), 4);
        }

        #[test]
        fn f_has_order_4() {
            has_order(f(), 4);
        }

        #[test]
        fn d_has_order_4() {
            has_order(d(), 4);
        }

        #[test]
        fn l_has_order_4() {
            has_order(l(), 4);
        }

        #[test]
        fn b_has_order_4() {
            has_order(b(), 4);
        }
    }

    mod quarter_turn_pairs_have_order_6 {
        use super::*;

        /// Creates a new permutation from A and B: A B A^-1 B^-1
        fn ab_ab_inv(a: &CornerPerm, b: &CornerPerm) -> CornerPerm {
            a.sequence(b).sequence(&a.invert()).sequence(&b.invert())
        }

        #[test]
        fn regular_sexy_move() {
            has_order(&ab_ab_inv(r(), u()), 6);
        }

        #[test]
        fn front_sledgehammer() {
            has_order(&ab_ab_inv(r(), f()), 6);
        }

        #[test]
        fn back_sexy_move() {
            has_order(&ab_ab_inv(l(), d()), 6);
        }

        #[test]
        fn back_sledgehammer() {
            has_order(&ab_ab_inv(l(), f()), 6);
        }
    }

    mod opposite_turns_commute {
        use super::*;

        /// Tests whether two permutations commute
        fn commutes(a: &CornerPerm, b: &CornerPerm) {
            assert_eq!(a.sequence(b), b.sequence(a));
        }

        #[test]
        fn ud() {
            commutes(u(), d());
        }

        #[test]
        fn rl() {
            commutes(r(), l());
        }

        #[test]
        fn fb() {
            commutes(f(), b());
        }
    }
}
