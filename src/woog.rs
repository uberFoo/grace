//! Initialize Woog
//!
//! This involves creating instances in Woog that the compiler stages depend
//! upon.
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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
        find_store, get_referrers_sorted, get_subtypes_sorted, is_object_stale,
        local_object_is_hybrid, local_object_is_struct, object_is_singleton, object_is_supertype,
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
pub(crate) fn populate_woog(
    src_path: &Path,
    module: &str,
    config: &GraceConfig,
    imports: &HashMap<String, Domain>,
    domain: &Domain,
) -> WoogStore {
    // Look for a persisted store.
    let mut path = PathBuf::from(src_path);
    path.pop();
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(domain.name());

    let mut woog = if path.exists() && !config.get_always_process() {
        log::debug!("Loading Woog store from: {}", path.display());
        WoogStore::load(&path).unwrap_or_else(|e| {
            log::warn!("Failed to load Woog store: {}", e);
            WoogStore::new()
        })
    } else {
        WoogStore::new()
    };

    let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
    objects.sort_by(|a, b| a.name.cmp(&b.name));

    // Iterate over the objects and create ObjectMethods for each.
    for obj in objects {
        if !is_object_stale(obj, &woog, domain) {
            log::debug!("Skipping woog for: {}", obj.name);
            continue;
        }

        if config.is_external(&obj.id) {
            log::debug!("Populating woog for external: {}", obj.name);
            let ext = config.get_external(&obj.id).unwrap();
            inter_external_method_new(obj, &ext, module, config, domain, &mut woog);
        } else if local_object_is_struct(obj, config, domain) {
            log::debug!("Populating woog for struct: {}", obj.name);
            inter_struct_method_new(obj, module, config, domain, &mut woog);
        } else if local_object_is_hybrid(obj, config, domain) {
            // log::debug!("Populating woog for hybrid: {}", obj.name);
            // inter_hybrid_method_new(obj, module, config, imports, domain, &mut woog);
        }
    }

    // Inter types
    for ty in domain.sarzak().iter_ty() {
        let _ = GraceType::new_ty(&ty, &mut woog);
    }

    woog
}

fn inter_struct_method_new(
    obj: &Object,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    let borrowed = Ownership::new_borrowed();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mutable = Ownership::new_mutable();
    let mut_access = Access::new(&mutable, &public, woog);

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

    let (mut params, mut fields) = collect_attributes(
        obj, &structure, &function, &table, module, config, domain, woog,
    );

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

fn inter_hybrid_method_new(
    obj: &Object,
    module: &str,
    config: &GraceConfig,
    imports: &HashMap<String, Domain>,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    const SUBTYPE_ATTR: &str = "subtype";

    let borrowed = Ownership::new_borrowed();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mutable = Ownership::new_mutable();
    let mut_access = Access::new(&mutable, &public, woog);

    let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

    for subtype in subtypes {
        let s_obj = subtype.r15_object(domain.sarzak())[0];

        let block = Block::new(Uuid::new_v4(), woog);

        let structure = Structure::new(s_obj.as_type(&Ownership::new_owned(), woog, domain), woog);
        let item = Item::new_structure(&structure, woog);
        let _ = Statement::new_item(&block, &item, woog);

        let method = ObjectMethod::new(&block, s_obj, woog);
        let function = Function::new_object_method(
            format!(
                "Inter a new '{}' in the store, and return it's `id`.",
                s_obj.name
            ),
            "new".to_owned(),
            &method,
            woog,
        );

        let table = SymbolTable::new(&block, woog);

        let (mut params, mut fields) = collect_attributes(
            s_obj, &structure, &function, &table, module, config, domain, woog,
        );

        // These are for the "subtype" attribute, which points at the subtype.
        let reference = Reference::new(&s_obj, woog);
        let ty = GraceType::new_reference(&reference, woog);

        let field = Field::new(SUBTYPE_ATTR.to_owned(), &ty, woog);
        let field = StructureField::new(None, &field, &structure, woog);
        fields.insert(0, field);

        // Fix these unwraps later.
        if object_is_singleton(s_obj, config, &Some(imports), domain).unwrap()
            && !object_is_supertype(s_obj, config, &Some(imports), domain).unwrap()
        {
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
            let var = Variable::new_parameter(SUBTYPE_ATTR.to_owned(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty.into(), &var, woog);
            params.insert(0, param);
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
}

/// Create a method to create a new instance of the external entity.
///
fn inter_external_method_new(
    obj: &Object,
    external: &ExternalEntity,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    const VALUE_ATTR: &str = "ext_value";

    let borrowed = Ownership::new_borrowed();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let owned = Ownership::new_owned();
    let owned_access = Access::new(&owned, &public, woog);

    let mutable = Ownership::new_mutable();
    let mut_access = Access::new(&mutable, &public, woog);

    let block = Block::new(Uuid::new_v4(), woog);

    let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);
    let item = Item::new_structure(&structure, woog);
    let _ = Statement::new_item(&block, &item, woog);

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

    let (mut params, mut fields) = collect_attributes(
        obj, &structure, &function, &table, module, config, domain, woog,
    );

    // Maybe this is a hack, maybe it's cool. In any case, I'm inserting an
    // attribute on external entities to store the internal value of the thing.
    // I dub thee: `道`! Maybe I should just go with `ext_value`? Didn't work. 😢
    // This is unfortunate. I'm not really sure how else to do it though.
    let ee = domain
        .sarzak()
        .iter_external()
        .find(|e| e.name == external.name)
        .unwrap();
    let ty = ee.r3_ty(domain.sarzak())[0];
    let ty = GraceType::new_ty(&ty, woog);

    let field = Field::new(VALUE_ATTR.to_owned(), &ty, woog);
    let field = StructureField::new(None, &field, &structure, woog);
    fields.insert(0, field);

    let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
    let var = Variable::new_parameter(VALUE_ATTR.to_owned(), &table, &param, woog);
    let _ = Value::new_variable(&owned_access, &ty.into(), &var, woog);
    params.insert(0, param);

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

fn collect_attributes(
    obj: &Object,
    structure: &Structure,
    function: &Function,
    table: &SymbolTable,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> (Vec<Parameter>, Vec<StructureField>) {
    let borrowed = Ownership::new_borrowed();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

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

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_referrers_sorted!(&obj, domain.sarzak());
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

    (params, fields)
}
