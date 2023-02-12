//! One to One Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// one_to_one
pub const UUID_NS: Uuid = uuid!("2b22547d-74d3-5dd4-94a9-122a2ac9341f");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("fred".to_owned(), &mut store);
        let tgt_1 = Referent::new("gene".to_owned(), &mut store);
        let a = A::new(42, &tgt_0, &mut store);

        // let select_tgt = one_to_one_get_one_tgt_across_r1!(a, store);
        // assert_eq!(&tgt_0, select_tgt);

        // let select_a = one_to_one_maybe_get_one_a_across_r1!(tgt_0, store);
        // assert!(select_a.is_some());
        // assert_eq!(Some(&a), select_a);

        // let select_a = one_to_one_maybe_get_one_a_across_r1!(tgt_1, store);
        // assert!(select_a.is_none());
    }

    #[test]
    fn test_r2() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("hugh".to_owned(), &mut store);
        let tgt_1 = Referent::new("jack".to_owned(), &mut store);
        let b_0 = B::new(true, &tgt_0, &mut store);
        let b_1 = B::new(false, &tgt_1, &mut store);

        // let select_tgt_0 = one_to_one_get_one_tgt_across_r2!(b_0, store);
        // assert_eq!(&tgt_0, select_tgt_0);

        // let select_tgt_1 = one_to_one_get_one_tgt_across_r2!(b_1, store);
        // assert_eq!(&tgt_1, select_tgt_1);

        // let select_b_0 = one_to_one_get_one_b_across_r2!(tgt_0, store);
        // assert_eq!(&b_0, select_b_0);

        // let select_b_1 = one_to_one_get_one_b_across_r2!(tgt_1, store);
        // assert_eq!(&b_1, select_b_1);
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("jerry".to_owned(), &mut store);
        let tgt_1 = Referent::new("bob".to_owned(), &mut store);
        let c_0 = C::new(42.0, Some(&tgt_1), &mut store);
        let c_1 = C::new(1.162, None, &mut store);

        // let tgt = one_to_one_maybe_get_one_tgt_across_r3!(c_0, store);
        // assert!(tgt.is_some());
        // assert_eq!(Some(&tgt_1), tgt);

        // let tgt = one_to_one_maybe_get_one_tgt_across_r3!(c_1, store);
        // assert!(tgt.is_none());

        // let c = one_to_one_maybe_get_one_c_across_r3!(tgt_0, store);
        // assert!(c.is_none());

        // let c = one_to_one_maybe_get_one_c_across_r3!(tgt_1, store);
        // assert!(c.is_some());
        // assert_eq!(Some(&c_0), c);
    }
}
