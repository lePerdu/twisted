use crate::cube::cube2::primitives as corner_prim;
use crate::puzzle::PuzzlePerm;

use super::Cube3Perm;

mod edge_prim {
    use crate::cube::edge::{Edge, EdgeOrient::*, EdgePerm, EdgePos::*};

    pub const U: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(UR, Oriented),
            Edge::new(UF, Oriented),
            Edge::new(UL, Oriented),
            Edge::new(UB, Oriented),
            Edge::new(DF, Oriented),
            Edge::new(DR, Oriented),
            Edge::new(DB, Oriented),
            Edge::new(DL, Oriented),
            Edge::new(FR, Oriented),
            Edge::new(FL, Oriented),
            Edge::new(BL, Oriented),
            Edge::new(BR, Oriented),
        ],
    };

    pub const R: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(UF, Oriented),
            Edge::new(UL, Oriented),
            Edge::new(UB, Oriented),
            Edge::new(FR, Oriented),
            Edge::new(DF, Oriented),
            Edge::new(BR, Oriented),
            Edge::new(DB, Oriented),
            Edge::new(DL, Oriented),
            Edge::new(DR, Oriented),
            Edge::new(FL, Oriented),
            Edge::new(BL, Oriented),
            Edge::new(UR, Oriented),
        ],
    };

    pub const F: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(FL, NotOriented),
            Edge::new(UL, Oriented),
            Edge::new(UB, Oriented),
            Edge::new(UR, Oriented),
            Edge::new(FR, NotOriented),
            Edge::new(DR, Oriented),
            Edge::new(DB, Oriented),
            Edge::new(DL, Oriented),
            Edge::new(UF, NotOriented),
            Edge::new(DF, NotOriented),
            Edge::new(BL, Oriented),
            Edge::new(BR, Oriented),
        ],
    };

    pub const D: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(UF, Oriented),
            Edge::new(UL, Oriented),
            Edge::new(UB, Oriented),
            Edge::new(UR, Oriented),
            Edge::new(DL, Oriented),
            Edge::new(DF, Oriented),
            Edge::new(DR, Oriented),
            Edge::new(DB, Oriented),
            Edge::new(FR, Oriented),
            Edge::new(FL, Oriented),
            Edge::new(BL, Oriented),
            Edge::new(BR, Oriented),
        ],
    };

    pub const L: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(UF, Oriented),
            Edge::new(BL, Oriented),
            Edge::new(UB, Oriented),
            Edge::new(UR, Oriented),
            Edge::new(DF, Oriented),
            Edge::new(DR, Oriented),
            Edge::new(DB, Oriented),
            Edge::new(FL, Oriented),
            Edge::new(FR, Oriented),
            Edge::new(UL, Oriented),
            Edge::new(DL, Oriented),
            Edge::new(BR, Oriented),
        ],
    };

    pub const B: EdgePerm = EdgePerm {
        cubies: [
            Edge::new(UF, Oriented),
            Edge::new(UL, Oriented),
            Edge::new(BR, NotOriented),
            Edge::new(UR, Oriented),
            Edge::new(DF, Oriented),
            Edge::new(DR, Oriented),
            Edge::new(BL, NotOriented),
            Edge::new(DL, Oriented),
            Edge::new(FR, Oriented),
            Edge::new(FL, Oriented),
            Edge::new(UB, NotOriented),
            Edge::new(DB, NotOriented),
        ],
    };
}

pub fn u() -> &'static Cube3Perm {
    lazy_static! {
        static ref U: Cube3Perm = Cube3Perm {
            corners: corner_prim::u().clone(),
            edges: edge_prim::U.clone(),
        };
    }

    &U
}

pub fn u_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref U_PRIME: Cube3Perm = u().invert();
    }

    &U_PRIME
}

pub fn u2() -> &'static Cube3Perm {
    lazy_static! {
        static ref U2: Cube3Perm = u().ntimes(2);
    }

    &U2
}

pub fn f() -> &'static Cube3Perm {
    lazy_static! {
        static ref F: Cube3Perm = Cube3Perm {
            corners: corner_prim::f().clone(),
            edges: edge_prim::F.clone(),
        };
    }

    &F
}

pub fn f_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref F_PRIME: Cube3Perm = f().invert();
    }

    &F_PRIME
}

pub fn f2() -> &'static Cube3Perm {
    lazy_static! {
        static ref F2: Cube3Perm = f().ntimes(2);
    }

    &F2
}

pub fn r() -> &'static Cube3Perm {
    lazy_static! {
        static ref R: Cube3Perm = Cube3Perm {
            corners: corner_prim::r().clone(),
            edges: edge_prim::R.clone(),
        };
    }

    &R
}

pub fn r_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref R_PRIME: Cube3Perm = r().invert();
    }

    &R_PRIME
}

pub fn r2() -> &'static Cube3Perm {
    lazy_static! {
        static ref R2: Cube3Perm = r().ntimes(2);
    }

    &R2
}

pub fn d() -> &'static Cube3Perm {
    lazy_static! {
        static ref D: Cube3Perm = Cube3Perm {
            corners: corner_prim::d().clone(),
            edges: edge_prim::D.clone(),
        };
    }

    &D
}

pub fn d_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref D_PRIME: Cube3Perm = d().invert();
    }

    &D_PRIME
}

pub fn d2() -> &'static Cube3Perm {
    lazy_static! {
        static ref D2: Cube3Perm = d().ntimes(2);
    }

    &D2
}

pub fn l() -> &'static Cube3Perm {
    lazy_static! {
        static ref L: Cube3Perm = Cube3Perm {
            corners: corner_prim::l().clone(),
            edges: edge_prim::L.clone(),
        };
    }

    &L
}

pub fn l_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref L_PRIME: Cube3Perm = l().invert();
    }

    &L_PRIME
}

pub fn l2() -> &'static Cube3Perm {
    lazy_static! {
        static ref L2: Cube3Perm = l().ntimes(2);
    }

    &L2
}

pub fn b() -> &'static Cube3Perm {
    lazy_static! {
        static ref B: Cube3Perm = Cube3Perm {
            corners: corner_prim::b().clone(),
            edges: edge_prim::B.clone(),
        };
    }

    &B
}

pub fn b_prime() -> &'static Cube3Perm {
    lazy_static! {
        static ref B_PRIME: Cube3Perm = b().invert();
    }

    &B_PRIME
}

pub fn b2() -> &'static Cube3Perm {
    lazy_static! {
        static ref B2: Cube3Perm = b().ntimes(2);
    }

    &B2
}

// TODO These tests are identical to the 2x2x2 ones

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_own_inverse() {
        let def = Cube3Perm::identity().invert();
        assert_eq!(&def, Cube3Perm::identity());
    }

    /// Checks that the order of a permutation is exactly as specified
    fn has_order(perm: &Cube3Perm, order: u32) {
        let mut p = perm.clone();
        for o in 1..order {
            assert_ne!(
                &p,
                Cube3Perm::identity(),
                "Order ({}) is less than expected ({})",
                o,
                order
            );
            p = p.sequence(perm);
        }

        assert_eq!(
            &p,
            Cube3Perm::identity(),
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
        fn ab_ab_inv(a: &Cube3Perm, b: &Cube3Perm) -> Cube3Perm {
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
        fn commutes(a: &Cube3Perm, b: &Cube3Perm) {
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
