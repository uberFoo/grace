//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// one_to_many
pub const UUID_NS: Uuid = uuid!("f2633df4-8cb2-5d43-b4aa-0ec7150bd928");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("Woogie".to_owned(), &mut store);
        let a_0 = A::new("Iniya".to_owned(), &tgt_0, &mut store);

        let tgt_1 = Referent::new("blubber".to_owned(), &mut store);
        let a_1 = A::new("foo".to_owned(), &tgt_1, &mut store);
        let a_2 = A::new("bar".to_owned(), &tgt_1, &mut store);

        // Test in the one direction.
        let tgt = a_0.referent_r1(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_0, tgt[0]);

        let tgt = a_1.referent_r1(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        let tgt = a_2.referent_r1(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        // Test in the many direction
        let a = tgt_0.a(&store);
        assert_eq!(1, a.len());
        assert_eq!(&a_0, a[0]);

        let a = tgt_1.a(&store);
        assert_eq!(2, a.len());

        // Result contains a_1 and a_2, but in any order.
        let an_a = a.iter().find(|a| a.id == a_1.id);
        assert!(an_a.is_some());
        assert_eq!(Some(&&a_1), an_a);

        let an_a = a.iter().find(|a| a.id == a_2.id);
        assert!(an_a.is_some());
        assert_eq!(Some(&&a_2), an_a);
    }

    #[test]
    fn test_r2() {
        let mut store = ObjectStore::new();

        let b_0 = B::new("oh no".to_owned(), None, &mut store);

        let tgt_1 = Referent::new("not".to_owned(), &mut store);
        let b_1 = B::new("more".to_owned(), Some(&tgt_1), &mut store);
        let b_2 = B::new("strings".to_owned(), Some(&tgt_1), &mut store);

        // Test in the one direction.
        let tgt = b_0.referent(&store);
        assert!(tgt.len() == 0);

        let tgt = b_1.referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        let tgt = b_2.referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        // Test in the many direction
        let b = tgt_1.b(&store);
        assert!(b.len() == 2);

        // Result contains b_1 and b_2, but in any order.
        let a_b = b.iter().find(|b| b.id == b_1.id);
        assert!(a_b.is_some());
        assert_eq!(Some(&&b_1), a_b);

        let a_b = b.iter().find(|b| b.id == b_2.id);
        assert!(a_b.is_some());
        assert_eq!(Some(&&b_2), a_b);
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("last".to_owned(), &mut store);

        let tgt_1 = Referent::new("few".to_owned(), &mut store);
        let c_1 = C::new(3.141, &tgt_1, &mut store);
        let c_2 = C::new(1.618, &tgt_1, &mut store);

        // Test in the one direction.
        let tgt = c_1.referent_r3(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        let tgt = c_2.referent_r3(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        // Test in the many direction
        let c = tgt_0.c(&store);
        assert!(c.len() == 0);

        let c = tgt_1.c(&store);
        assert!(c.len() == 2);

        // Result contains c_1 and c_2, but in any order.
        let a_c = c.iter().find(|c| c.id == c_1.id);
        assert!(a_c.is_some());
        assert_eq!(Some(&&c_1), a_c);

        let a_c = c.iter().find(|c| c.id == c_2.id);
        assert!(a_c.is_some());
        assert_eq!(Some(&&c_2), a_c);
    }

    #[test]
    fn test_r4() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("lonely".to_owned(), &mut store);
        let d_0 = D::new("larry".to_owned(), None, &mut store);

        let tgt_1 = Referent::new("string".to_owned(), &mut store);
        let d_1 = D::new("moe".to_owned(), Some(&tgt_1), &mut store);
        let d_2 = D::new("curly".to_owned(), Some(&tgt_1), &mut store);

        // Test in the one direction.
        let tgt = d_0.referent(&store);
        assert!(tgt.len() == 0);

        let tgt = d_1.referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        let tgt = d_2.referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&tgt_1, tgt[0]);

        // Test in the many direction
        let d = tgt_0.d(&store);
        assert!(d.len() == 0);

        let d = tgt_1.d(&store);
        assert!(d.len() == 2);

        // Result contains d_1 and d_2, but in any order.
        let a_d = d.iter().find(|d| d.id == d_1.id);
        assert!(a_d.is_some());
        assert_eq!(Some(&&d_1), a_d);

        let a_d = d.iter().find(|d| d.id == d_2.id);
        assert!(a_d.is_some());
        assert_eq!(Some(&&d_2), a_d);
    }
}
