//! Initialize Woog
//!
//! This involves creating instances in Woog that the compiler stages depend
//! upon.
use std::path::{Path, PathBuf};

use sarzak::{
    mc::{FileSnafu, Result},
    sarzak::types::{Conditionality, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{
            Access, Block, Field, Function, GraceType, Item, Local, ObjectMethod, Ownership,
            Parameter, Reference, Statement, Structure, StructureField, SymbolTable, Value,
            Variable, Visibility, WoogOption, PUBLIC,
        },
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        find_store, get_referrers_sorted, inner_object_is_struct, is_object_stale,
        render::{RenderIdent, RenderType},
    },
    options::{ExternalEntity, GraceConfig, Target},
    BUILD_DIR, TARGET_DIR,
};

pub(crate) fn persist_woog(woog: &WoogStore, src_path: &Path, domain: &Domain) -> Result<()> {
    let mut path = PathBuf::from(src_path);
    path.pop();
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(domain.name());

    woog.persist(&path).context(FileSnafu { path })
}

/// Woog post-load domain processing
///
/// Below we add an ObjectMethod instance for each object in the domain.
///
/// We also inter types in woog that exist in sarzak, so that we can access them
/// during code generation.
pub(crate) fn init_woog(
    src_path: &Path,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
) -> WoogStore {
    // Look for a persisted store.
    let mut path = PathBuf::from(src_path);
    path.pop();
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(domain.name());

    // 🚧 put this back once timestamps are working, which I think depends on EEs working.
    // let mut woog = if path.exists() {
    //     log::debug!("Loading Woog store from: {}", path.display());
    //     WoogStore::load(&path).unwrap_or_else(|e| {
    //         log::warn!("Failed to load Woog store: {}", e);
    //         WoogStore::new()
    //     })
    // } else {
    //     WoogStore::new()
    // };
    let mut woog = WoogStore::new();

    let borrowed = Ownership::new_borrowed();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, &mut woog);

    let mutable = Ownership::new_mutable();
    let mut_access = Access::new(&mutable, &public, &mut woog);

    let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
    objects.sort_by(|a, b| a.name.cmp(&b.name));

    // Iterate over the objects and create ObjectMethods for each.
    for obj in objects {
        if !is_object_stale(obj, &woog, domain) {
            continue;
        }

        if inner_object_is_struct(obj, config, domain) {
            inter_method_new(obj, &access, &mut_access, module, config, domain, &mut woog);
        } else if config.is_external(&obj.id) {
            let ext = config.get_external(&obj.id).unwrap();
            inter_external(
                obj,
                &ext,
                &access,
                &mut_access,
                module,
                config,
                domain,
                &mut woog,
            );
        }
    }

    // Inter types
    for ty in domain.sarzak().iter_ty() {
        let _ = GraceType::new_ty(&ty, &mut woog);
    }

    woog
}

fn inter_method_new(
    obj: &Object,
    access: &Access,
    mut_access: &Access,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    let block = Block::new(Uuid::new_v4(), woog);

    let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);
    let item = Item::new_structure(&structure, woog);
    let _ = Statement::new_item(&block, &item, woog);

    let method = ObjectMethod::new(&block, obj, woog);
    let function = Function::new_object_method(
        format!(
            "Inter a new '{}' in the store, and return it's `id`.",
            obj.name
        ),
        "new".to_owned(),
        &method,
        woog,
    );

    let table = SymbolTable::new(&block, woog);

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_referrers_sorted!(&obj, domain.sarzak());

    let mut params = Vec::new();
    let mut fields = Vec::new();
    // Collect the attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let ty = GraceType::new_ty(&ty, woog);

        let field = Field::new(attr.as_ident(), &ty, woog);
        let field = StructureField::new(None, &field, &structure, woog);
        fields.push(field);

        // We are going to generate the id, so don't include it in the
        // list of parameters.
        if attr.name != "id" {
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
            let var = Variable::new_parameter(attr.as_ident(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty.into(), &var, woog);
            params.push(param);
        }
    }

    // And the referential attributes
    for referrer in &referrers {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        // This determines how a reference is stored in the struct. In this
        // case a UUID.
        match cond {
            // If it's conditional build a parameter that's an optional reference
            // to the referent.
            Conditionality::Conditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );
                let reference = Reference::new(&r_obj, woog);
                let reference = GraceType::new_reference(&reference, woog);
                let option = WoogOption::new(&reference, woog);
                let ty = GraceType::new_woog_option(&option, woog);
                let _ = Value::new_variable(&access, &ty, &var, woog);
                params.push(param);

                let uuid = GraceType::new_ty(&Ty::new_uuid(), woog);
                let option = WoogOption::new(&uuid, woog);
                let ty = GraceType::new_woog_option(&option, woog);
                let field = Field::new(referrer.referential_attribute.as_ident(), &ty, woog);
                let field = StructureField::new(None, &field, &structure, woog);
                fields.push(field);
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );
                let reference = Reference::new(&r_obj, woog);
                let ty = GraceType::new_reference(&reference, woog);
                let _ = Value::new_variable(&access, &ty, &var, woog);
                params.push(param);

                let field = Field::new(
                    referrer.referential_attribute.as_ident(),
                    &GraceType::new_ty(&Ty::new_uuid(), woog),
                    woog,
                );
                let field = StructureField::new(None, &field, &structure, woog);
                fields.push(field);
            }
        }
    }

    // And the associative attributes
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];

        let one = assoc.r23_associative_referent(domain.sarzak())[0];
        let one_obj = one.r25_object(domain.sarzak())[0];

        let other = assoc.r22_associative_referent(domain.sarzak())[0];
        let other_obj = other.r25_object(domain.sarzak())[0];

        // One side
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter(
            assoc_referrer.one_referential_attribute.as_ident(),
            &table,
            &param,
            woog,
        );
        let reference = Reference::new(&one_obj, woog);
        let ty = GraceType::new_reference(&reference, woog);
        let _ = Value::new_variable(&access, &ty, &var, woog);
        params.push(param);

        let field = Field::new(
            assoc_referrer.one_referential_attribute.as_ident(),
            &GraceType::new_ty(&Ty::new_uuid(), woog),
            woog,
        );
        let field = StructureField::new(None, &field, &structure, woog);
        fields.push(field);

        // Other side
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter(
            assoc_referrer.other_referential_attribute.as_ident(),
            &table,
            &param,
            woog,
        );
        let reference = Reference::new(&other_obj, woog);
        let ty = GraceType::new_reference(&reference, woog);
        let _ = Value::new_variable(&access, &ty, &var, woog);
        params.push(param);

        let field = Field::new(
            assoc_referrer.other_referential_attribute.as_ident(),
            &GraceType::new_ty(&Ty::new_uuid(), woog),
            woog,
        );
        let field = StructureField::new(None, &field, &structure, woog);
        fields.push(field);
    }

    if let Target::Domain(_) = config.get_target() {
        // Add the store to the end of the  input parameters
        let store = find_store(module, &woog, domain);
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter("store".to_owned(), &table, &param, woog);
        let external = Ty::External(store.id);
        let ty = GraceType::new_ty(&external, woog);
        let _ = Value::new_variable(&mut_access, &ty, &var, woog);
        params.push(param);
    }

    // Link the params
    // I need to maintain the order I've adopted because I'don't need things
    // changing. That said, I need to iterate over the local parameters,
    // and not what's interred in the store. So, I do the weird thing, and
    // iterate over the locals, and push the change to the store.
    params.iter_mut().rev().fold(None, |next, param| {
        param.next = next;
        woog.inter_parameter(param.clone());
        Some(param.id)
    });
    // Same-same for the fields
    fields.iter_mut().rev().fold(None, |next, field| {
        field.next = next;
        woog.inter_structure_field(field.clone());
        Some(field.id)
    });

    // Locals we'll need
    let id = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("id".to_owned(), &table, &id, woog);
    let _value = Value::new_variable(
        &access,
        &GraceType::new_ty(&Ty::new_uuid(), woog),
        &var,
        woog,
    );

    let new = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("new".to_owned(), &table, &new, woog);
    let _value = Value::new_variable(
        &access,
        &GraceType::new_reference(&Reference::new(&obj, woog), woog),
        &var,
        woog,
    );
}

/// Create a method to create a new instance of the external entity.
///
/// This shares so much code with that above, it's rather silly. I have not given
/// it any real thought, but there must be a way to refactor this.
fn inter_external(
    obj: &Object,
    external: &ExternalEntity,
    access: &Access,
    mut_access: &Access,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    let block = Block::new(Uuid::new_v4(), woog);
    let method = ObjectMethod::new(&block, obj, woog);
    let function = Function::new_object_method(
        format!(
            "Create a new instance of the external entity,  '{}', wrapped in an {}.",
            external.name, obj.name
        ),
        external.ctor.clone(),
        &method,
        woog,
    );

    let table = SymbolTable::new(&block, woog);

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_referrers_sorted!(&obj, domain.sarzak());

    // Collect the attributes
    let mut params: Vec<Parameter> = Vec::new();
    // Maybe this is a hack, maybe it's cool. In any case, I'm inserting an
    // attribute on external entities to store the internal value of the thing.
    // I dub thee: `道`! Maybe I should just go with `_ext_value`?
    // This is unfortunate. I'm not really sure how else to do it though.
    let ee = domain
        .sarzak()
        .iter_external()
        .find(|e| e.name == external.name)
        .unwrap();
    let ty = ee.r3_ty(domain.sarzak())[0];
    let ty = GraceType::new_ty(&ty, woog);
    let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
    let var = Variable::new_parameter("道".to_owned(), &table, &param, woog);
    let _ = Value::new_variable(&access, &ty.into(), &var, woog);
    params.push(param);

    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        // We are going to generate the id, so don't include it in the
        // list of parameters.
        if attr.name != "id" {
            let ty = attr.r2_ty(domain.sarzak())[0];
            let ty = GraceType::new_ty(&ty, woog);
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
            let var = Variable::new_parameter(attr.as_ident(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty.into(), &var, woog);

            params.push(param);
        }
    }

    // And the referential attributes
    for referrer in &referrers {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        // This determines how a reference is stored in the struct. In this
        // case a UUID.
        match cond {
            // If it's conditional build a parameter that's an optional reference
            // to the referent.
            Conditionality::Conditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );
                let reference = Reference::new(&r_obj, woog);
                let reference = GraceType::new_reference(&reference, woog);
                let option = WoogOption::new(&reference, woog);
                let option = GraceType::new_woog_option(&option, woog);
                let _ = Value::new_variable(&access, &option, &var, woog);

                params.push(param);
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );
                let reference = Reference::new(&r_obj, woog);
                let reference = GraceType::new_reference(&reference, woog);
                let _ = Value::new_variable(&access, &reference, &var, woog);

                params.push(param);
            }
        }
    }

    // And the associative attributes
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];

        let one = assoc.r23_associative_referent(domain.sarzak())[0];
        let one_obj = one.r25_object(domain.sarzak())[0];

        let other = assoc.r22_associative_referent(domain.sarzak())[0];
        let other_obj = other.r25_object(domain.sarzak())[0];

        // One side
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter(
            assoc_referrer.one_referential_attribute.as_ident(),
            &table,
            &param,
            woog,
        );
        let reference = Reference::new(&one_obj, woog);
        let reference = GraceType::new_reference(&reference, woog);
        let _ = Value::new_variable(&access, &reference, &var, woog);

        params.push(param);

        // Other side
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter(
            assoc_referrer.other_referential_attribute.as_ident(),
            &table,
            &param,
            woog,
        );
        let reference = Reference::new(&other_obj, woog);
        let reference = GraceType::new_reference(&reference, woog);
        let _ = Value::new_variable(&access, &reference, &var, woog);

        params.push(param);
    }

    if let Target::Domain(_) = config.get_target() {
        // Add the store to the end of the  input parameters
        let store = find_store(module, &woog, domain);
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
        let var = Variable::new_parameter("store".to_owned(), &table, &param, woog);
        let ty = Ty::External(store.id);
        let ty = GraceType::new_ty(&ty, woog);
        let _ = Value::new_variable(&mut_access, &ty, &var, woog);

        params.push(param);
    }

    // Link the params
    // I need to maintain the order I've adopted because I'don't need things
    // changing. That said, I need to iterate over the local parameters,
    // and not what's interred in teh store. So, I do the weird thing, and
    // iterate over the locals, and push the change to the store.
    params.iter_mut().rev().fold(None, |next, param| {
        param.next = next;
        woog.inter_parameter(param.clone());
        Some(param.id)
    });
}
