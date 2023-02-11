//! One to One Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("one_to_one", "models/one_to_one.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! ![One to One Test Model][one_to_one]
use uuid::{uuid, Uuid};

pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;
pub use macros::*;

// one_to_one
pub const UUID_NS: Uuid = uuid!("2b22547d-74d3-5dd4-94a9-122a2ac9341f");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new(&mut store, "fred".to_owned());
        let tgt_1 = Referent::new(&mut store, "gene".to_owned());
        let a = A::new(&mut store, &tgt_0, 42);

        let select_tgt = one_to_one_get_one_tgt_across_r1!(a, store);
        assert_eq!(&tgt_0, select_tgt);

        let select_a = one_to_one_maybe_get_one_a_across_r1!(tgt_0, store);
        assert!(select_a.is_some());
        assert_eq!(Some(&a), select_a);

        let select_a = one_to_one_maybe_get_one_a_across_r1!(tgt_1, store);
        assert!(select_a.is_none());
    }

    #[test]
    fn test_r2() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new(&mut store, "hugh".to_owned());
        let tgt_1 = Referent::new(&mut store, "jack".to_owned());
        let b_0 = B::new(&mut store, &tgt_0, true);
        let b_1 = B::new(&mut store, &tgt_1, false);

        let select_tgt_0 = one_to_one_get_one_tgt_across_r2!(b_0, store);
        assert_eq!(&tgt_0, select_tgt_0);

        let select_tgt_1 = one_to_one_get_one_tgt_across_r2!(b_1, store);
        assert_eq!(&tgt_1, select_tgt_1);

        let select_b_0 = one_to_one_get_one_b_across_r2!(tgt_0, store);
        assert_eq!(&b_0, select_b_0);

        let select_b_1 = one_to_one_get_one_b_across_r2!(tgt_1, store);
        assert_eq!(&b_1, select_b_1);
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let tgt_0 = Referent::new(&mut store, "jerry".to_owned());
        let tgt_1 = Referent::new(&mut store, "bob".to_owned());
        let c_0 = C::new(&mut store, Some(&tgt_1), 42.0);
        let c_1 = C::new(&mut store, None, 1.162);

        let tgt = one_to_one_maybe_get_one_tgt_across_r3!(c_0, store);
        assert!(tgt.is_some());
        assert_eq!(Some(&tgt_1), tgt);

        let tgt = one_to_one_maybe_get_one_tgt_across_r3!(c_1, store);
        assert!(tgt.is_none());

        let c = one_to_one_maybe_get_one_c_across_r3!(tgt_0, store);
        assert!(c.is_none());

        let c = one_to_one_maybe_get_one_c_across_r3!(tgt_1, store);
        assert!(c.is_some());
        assert_eq!(Some(&c_0), c);
    }
}
