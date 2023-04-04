// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use uuid::Uuid;

use crate::domain::one_to_many_ts::types::a::A;
use crate::domain::one_to_many_ts::types::b::B;
use crate::domain::one_to_many_ts::types::c::C;
use crate::domain::one_to_many_ts::types::d::D;
use crate::domain::one_to_many_ts::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_ts::store::ObjectStore as OneToManyTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The object of so many relationships
///
/// I’m related to stuff.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToManyTsStore) -> Referent {
        let id = Uuid::new_v4();
        let new = Referent { id: id, name: name };
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-a"}}}
    /// Navigate to [`A`] across R1(1-M)
    pub fn r1_a<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&A> {
        store
            .iter_a()
            .filter_map(|a| if a.ptr == self.id { Some(a) } else { None })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-b"}}}
    /// Navigate to [`B`] across R2(1-Mc)
    pub fn r2_b<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&B> {
        store
            .iter_b()
            .filter_map(|b| {
                if b.ptr == Some(self.id) {
                    Some(b)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-c"}}}
    /// Navigate to [`C`] across R3(1-M)
    pub fn r3_c<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&C> {
        store
            .iter_c()
            .filter_map(|c| if c.ptr == self.id { Some(c) } else { None })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-d"}}}
    /// Navigate to [`D`] across R4(1-Mc)
    pub fn r4_d<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&D> {
        store
            .iter_d()
            .filter_map(|d| {
                if d.ptr == Some(self.id) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
