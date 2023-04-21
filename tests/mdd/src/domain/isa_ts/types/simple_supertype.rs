// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_supertype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_ts::types::baz::Baz;
use crate::domain::isa_ts::types::simple_subtype_a::SimpleSubtypeA;
use crate::domain::isa_ts::types::simple_subtype_b::SIMPLE_SUBTYPE_B;
use serde::{Deserialize, Serialize};

use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-hybrid-documentation"}}}
/// This [`Supertype`] is Simple
///
/// By that I mean that it's [`Subtypes`] consist only of singletons.
///
/// Not any more they don't. I sort of wonder if hijacking this test was a bad idea, because
///  now we don't have the singleton test. I'll put it back.
///
/// Anyway, there's a bug, and I thought adding something to [`OhBoy`] would surface the bug
/// , but it didn't. See it's description for more info.
///
/// So now, I think the bug is happening when this is a hybrid, so it's getting some attributes
/// . This is going to raise all sorts of hell, because I think I only made hybrid work with
///  referentials. Maybe not. We'll see. Fun! 💥💥💥💥
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SimpleSupertype {
    pub subtype: SimpleSupertypeEnum,
    pub id: Uuid,
    pub state: bool,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSupertypeEnum {
    SimpleSubtypeA(Uuid),
    SimpleSubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-implementation"}}}
impl SimpleSupertype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_a"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_a(
        state: bool,
        subtype: &SimpleSubtypeA,
        store: &mut IsaTsStore,
    ) -> SimpleSupertype {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id();
        let new = SimpleSupertype {
            state: state,
            subtype: SimpleSupertypeEnum::SimpleSubtypeA(subtype.id()),
            id,
        };
        store.inter_simple_supertype(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_a_"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_a_(state: bool, subtype: &SimpleSubtypeA) -> SimpleSupertype {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id();
        let new = SimpleSupertype {
            state: state,
            subtype: SimpleSupertypeEnum::SimpleSubtypeA(subtype.id()),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_b"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_b(state: bool, store: &mut IsaTsStore) -> SimpleSupertype {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = SIMPLE_SUBTYPE_B;
        let new = SimpleSupertype {
            state: state,
            subtype: SimpleSupertypeEnum::SimpleSubtypeB(SIMPLE_SUBTYPE_B),
            id,
        };
        store.inter_simple_supertype(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_b_"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_b_(state: bool) -> SimpleSupertype {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = SIMPLE_SUBTYPE_B;
        let new = SimpleSupertype {
            state: state,
            subtype: SimpleSupertypeEnum::SimpleSubtypeB(SIMPLE_SUBTYPE_B),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-nav-backward-one-to-baz"}}}
    /// Navigate to [`Baz`] across R4(1-1)
    pub fn r4_baz<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Baz> {
        vec![store.iter_baz().find(|baz| baz.fugue == self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
