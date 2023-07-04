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
    use tracy_client::Client;

    #[test]
    fn test_r1() {
        Client::start();

        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("Woogie".to_owned(), &mut store);
        let a_0 = A::new("Iniya".to_owned(), &tgt_0, &mut store);

        let tgt_1 = Referent::new("blubber".to_owned(), &mut store);
        let a_1 = A::new("foo".to_owned(), &tgt_1, &mut store);
        let a_2 = A::new("bar".to_owned(), &tgt_1, &mut store);

        // Test in the one direction.
        let tgt = a_0.read().unwrap().r1_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_0.read().unwrap(), &*tgt[0].read().unwrap());

        let tgt = a_1.read().unwrap().r1_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        let tgt = a_2.read().unwrap().r1_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        // Test in the many direction
        let a = tgt_0.read().unwrap().r1_a(&store);
        assert_eq!(1, a.len());
        assert_eq!(&*a_0.read().unwrap(), &*a[0].read().unwrap());

        let a = tgt_1.read().unwrap().r1_a(&store);
        assert_eq!(2, a.len());

        // Result contains a_1 and a_2, but in any order.
        let an_a = a
            .iter()
            .find(|a| a.read().unwrap().id == a_1.read().unwrap().id);
        assert!(an_a.is_some());
        assert_eq!(&*a_1.read().unwrap(), &*an_a.unwrap().read().unwrap());

        let an_a = a
            .iter()
            .find(|a| a.read().unwrap().id == a_2.read().unwrap().id);
        assert!(an_a.is_some());
        assert_eq!(&*a_2.read().unwrap(), &*an_a.unwrap().read().unwrap());
    }

    #[test]
    fn test_r2() {
        Client::start();

        let mut store = ObjectStore::new();

        let b_0 = B::new("oh no".to_owned(), None, &mut store);

        let tgt_1 = Referent::new("not".to_owned(), &mut store);
        let b_1 = B::new("more".to_owned(), Some(&tgt_1), &mut store);
        let b_2 = B::new("strings".to_owned(), Some(&tgt_1), &mut store);

        // Test in the one direction.
        let tgt = b_0.read().unwrap().r2_referent(&store);
        assert!(tgt.len() == 0);

        let tgt = b_1.read().unwrap().r2_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        let tgt = b_2.read().unwrap().r2_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        // Test in the many direction
        let b = tgt_1.read().unwrap().r2_b(&store);
        assert!(b.len() == 2);

        // Result contains b_1 and b_2, but in any order.
        let a_b = b
            .iter()
            .find(|b| b.read().unwrap().id == b_1.read().unwrap().id);
        assert!(a_b.is_some());
        assert_eq!(&*b_1.read().unwrap(), &*a_b.unwrap().read().unwrap());

        let a_b = b
            .iter()
            .find(|b| b.read().unwrap().id == b_2.read().unwrap().id);
        assert!(a_b.is_some());
        assert_eq!(&*b_2.read().unwrap(), &*a_b.unwrap().read().unwrap());
    }

    #[test]
    fn test_r3() {
        Client::start();

        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("last".to_owned(), &mut store);

        let tgt_1 = Referent::new("few".to_owned(), &mut store);
        let c_1 = C::new(3.141, &tgt_1, &mut store);
        let c_2 = C::new(1.618, &tgt_1, &mut store);

        // Test in the one direction.
        let tgt = c_1.read().unwrap().r3_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        let tgt = c_2.read().unwrap().r3_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        // Test in the many direction
        let c = tgt_0.read().unwrap().r3_c(&store);
        assert!(c.len() == 0);

        let c = tgt_1.read().unwrap().r3_c(&store);
        assert!(c.len() == 2);

        // Result contains c_1 and c_2, but in any order.
        let a_c = c
            .iter()
            .find(|c| c.read().unwrap().id == c_1.read().unwrap().id);
        assert!(a_c.is_some());
        assert_eq!(&*c_1.read().unwrap(), &*a_c.unwrap().read().unwrap());

        let a_c = c
            .iter()
            .find(|c| c.read().unwrap().id == c_2.read().unwrap().id);
        assert!(a_c.is_some());
        assert_eq!(&*c_2.read().unwrap(), &*a_c.unwrap().read().unwrap());
    }

    #[test]
    fn test_r4() {
        Client::start();

        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new("lonely".to_owned(), &mut store);
        let d_0 = D::new("larry".to_owned(), None, &mut store);

        let tgt_1 = Referent::new("string".to_owned(), &mut store);
        let d_1 = D::new("moe".to_owned(), Some(&tgt_1), &mut store);
        let d_2 = D::new("curly".to_owned(), Some(&tgt_1), &mut store);

        // Test in the one direction.
        let tgt = d_0.read().unwrap().r4_referent(&store);
        assert!(tgt.len() == 0);

        let tgt = d_1.read().unwrap().r4_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        let tgt = d_2.read().unwrap().r4_referent(&store);
        assert!(tgt.len() == 1);
        assert_eq!(&*tgt_1.read().unwrap(), &*tgt[0].read().unwrap());

        // Test in the many direction
        let d = tgt_0.read().unwrap().r4_d(&store);
        assert!(d.len() == 0);

        let d = tgt_1.read().unwrap().r4_d(&store);
        assert!(d.len() == 2);

        // Result contains d_1 and d_2, but in any order.
        let a_d = d
            .iter()
            .find(|d| d.read().unwrap().id == d_1.read().unwrap().id);
        assert!(a_d.is_some());
        assert_eq!(&*d_1.read().unwrap(), &*a_d.unwrap().read().unwrap());

        let a_d = d
            .iter()
            .find(|d| d.read().unwrap().id == d_2.read().unwrap().id);
        assert!(a_d.is_some());
        assert_eq!(&*d_2.read().unwrap(), &*a_d.unwrap().read().unwrap());
    }
}
