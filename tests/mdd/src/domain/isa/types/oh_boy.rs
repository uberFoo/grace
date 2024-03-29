// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"oh_boy-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::simple_subtype_a::SimpleSubtypeA;
use serde::{Deserialize, Serialize};

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-struct-documentation"}}}
/// This should break all sorts of shit.
///
/// The only purpose of this is to make [`SimpleSubtypeA`] a supertype. Then I got crafty with
///  the name. One of these days I'm going to throw an emoji in there...😝
///
/// So, more testing. Grace as of this moment (1677427948) doesn't properly recognize when
/// a subtype, that's also a supertype, has a subtype that isn't const... when rendering the
///  subtype code in the supertype. Whew, parsing that someday will be a nightmare.
///
/// Basically, [`SimpleSupertype`] thinks this object is a const. And it was (no attributes
///  besides id) until this moment.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OhBoy {
    pub attribution: String,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-implementation"}}}
impl OhBoy {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-struct-impl-new"}}}
    /// Inter a new 'Oh Boy!' in the store, and return it's `id`.
    pub fn new(attribution: String, store: &mut IsaStore) -> OhBoy {
        let id = Uuid::new_v4();
        let new = OhBoy { attribution, id };
        store.inter_oh_boy(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-impl-nav-subtype-to-supertype-simple_subtype_a"}}}
    // Navigate to [`SimpleSubtypeA`] across R8(isa)
    pub fn r8_simple_subtype_a<'a>(&'a self, store: &'a IsaStore) -> Vec<&SimpleSubtypeA> {
        vec![store.exhume_simple_subtype_a(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
