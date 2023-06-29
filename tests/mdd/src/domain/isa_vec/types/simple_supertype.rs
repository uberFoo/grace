// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_supertype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::baz::Baz;
use crate::domain::isa_vec::types::simple_subtype_a::SimpleSubtypeA;
use crate::domain::isa_vec::types::simple_subtype_b::SIMPLE_SUBTYPE_B;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
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
    pub id: usize,
    pub state: bool,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSupertypeEnum {
    SimpleSubtypeA(usize),
    SimpleSubtypeB(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-implementation"}}}
impl SimpleSupertype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_a"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_a(
        state: bool,
        subtype: &Rc<RefCell<SimpleSubtypeA>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<SimpleSupertype>> {
        store.inter_simple_supertype(|id| {
            Rc::new(RefCell::new(SimpleSupertype {
                state: state,
                subtype: SimpleSupertypeEnum::SimpleSubtypeA(subtype.borrow().id()),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-new_simple_subtype_b"}}}
    /// Inter a new SimpleSupertype in the store, and return it's `id`.
    pub fn new_simple_subtype_b(
        state: bool,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<SimpleSupertype>> {
        store.inter_simple_supertype(|id| {
            Rc::new(RefCell::new(SimpleSupertype {
                state: state,
                subtype: SimpleSupertypeEnum::SimpleSubtypeB(SIMPLE_SUBTYPE_B),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-impl-nav-backward-one-to-baz"}}}
    /// Navigate to [`Baz`] across R4(1-1)
    pub fn r4_baz<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<Baz>>> {
        span!("r4_baz");
        vec![store
            .iter_baz()
            .find(|baz| baz.borrow().fugue == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
