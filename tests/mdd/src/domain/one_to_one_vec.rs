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

        let select_tgt = a.borrow().r1_referent(&store);
        assert_eq!(tgt_0, select_tgt[0]);

        let select_a = tgt_0.borrow().r1c_a(&store);
        assert!(select_a.len() == 1);
        assert_eq!(a, select_a[0]);

        let select_a = tgt_1.borrow().r1c_a(&store);
        assert!(select_a.len() == 0);
    }

    #[test]
    fn test_r2() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("hugh".to_owned(), &mut store);
        let tgt_1 = Referent::new("jack".to_owned(), &mut store);
        let b_0 = B::new(true, &tgt_0, &mut store);
        let b_1 = B::new(false, &tgt_1, &mut store);

        let tgt = b_0.borrow().r2_referent(&store);
        assert_eq!(tgt_0, tgt[0]);

        let tgt = b_1.borrow().r2_referent(&store);
        assert_eq!(tgt_1, tgt[0]);

        let b = tgt_0.borrow().r2_b(&store);
        assert_eq!(b_0, b[0]);

        let b = tgt_1.borrow().r2_b(&store);
        assert_eq!(b_1, b[0]);
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("jerry".to_owned(), &mut store);
        let tgt_1 = Referent::new("bob".to_owned(), &mut store);
        let c_0 = C::new(42.0, Some(&tgt_1), &mut store);
        let c_1 = C::new(1.162, None, &mut store);

        let tgt = c_0.borrow().r3_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(tgt_1, tgt[0]);

        let tgt = c_1.borrow().r3_referent(&store);
        assert!(tgt.len() == 0);

        let c = tgt_0.borrow().r3c_c(&store);
        assert!(c.len() == 0);

        let c = tgt_1.borrow().r3c_c(&store);
        assert!(c.len() == 1);
        assert_eq!(c_0, c[0]);
    }

    #[test]
    fn test_r8() {
        // This one is the reflexive relationship
        let mut store = ObjectStore::new();

        let p_2 = Parameter::new("p_2".to_owned(), None, &mut store);
        let p_1 = Parameter::new("p_1".to_owned(), Some(&p_2), &mut store);
        let p_0 = Parameter::new("p_0".to_owned(), Some(&p_1), &mut store);

        assert!(p_0.borrow().r8c_parameter(&store).len() == 0);

        let p = p_0.borrow().r8_parameter(&store);
        assert_eq!(p_1, p[0]);

        assert!(p_2.borrow().r8_parameter(&store).len() == 0);
    }
}
