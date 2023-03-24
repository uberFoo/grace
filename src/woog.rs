//! Initialize Woog
//!
//! This involves creating instances in Woog that the compiler stages depend
//! upon.
use std::path::{Path, PathBuf};

use fnv::FnvHashMap as HashMap;
use sarzak::{
    mc::{CompilerSnafu, FileSnafu, Result},
    sarzak::types::{Conditionality, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{
            Access, Block, Borrowed, Call, Expression, Field, Function, GraceType, Hack, Literal,
            Local, ObjectMethod, Ownership, Parameter, Reference, Statement, StructExpression,
            StructExpressionField, Structure, StructureField, SymbolTable, Value, Variable,
            Visibility, WoogOption, XLet, PUBLIC, SHARED,
        },
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        find_store, get_assoc_referent_from_referrer_sorted, get_binary_referrers_sorted,
        get_subtypes_sorted, is_object_stale, local_object_is_enum, local_object_is_hybrid,
        local_object_is_struct, object_is_singleton, object_is_supertype,
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
) -> Result<WoogStore> {
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

        log::debug!("Populating woog for: {}", obj.name);
        make_data_structure(obj, domain, &mut woog);

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

    Ok(woog)
}

/// I'm trying to organize this function to be as similar to how the code is generated.
/// What I mean is that I'd like to introduce the woog objects in the same order as
/// they would be introduced, were they output by a parser.
///
/// Wow. Read those two sentences. They first is about code generation, and the second is
/// about code compilation. They are a reversal of the other, but also sort of the same
/// thing.
///
/// Anyway, I sort of want both things, and it's because I expect that the action
/// language, which I'd like to parse with a macro, and the generated code will
/// look pretty similar.
///
/// What I'd like to see this become is some quick setup in order to output the function
/// header, and then a sarzak! {} macro that contains everything else, written out as
/// some rust-like language. The macro will parse this and generate the code that is
/// currently in this function. That's why I stated the goal above.
fn inter_struct_method_new(
    obj: &Object,
    module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> () {
    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mutable = Ownership::new_borrowed(&Borrowed::new_mutable(), woog);
    let mut_access = Access::new(&mutable, &public, woog);

    let block = Block::new(Uuid::new_v4(), woog);
    let table = SymbolTable::new(&block, woog);

    let method = ObjectMethod::new(obj, woog);
    let function = Function::new_object_method(
        format!(
            "Inter a new '{}' in the store, and return it's `id`.",
            obj.name
        ),
        "new".to_owned(),
        &block,
        &method,
        woog,
    );

    // This needs to be moved to someplace generic. It's only here as I develop
    // the code.
    //
    // Create the structure definition
    //
    let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);

    //
    // Create statements in the body
    //

    //
    // `let id = Uuid::new_v5(...)`
    let id = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("id".to_owned(), &table, &id, woog);
    let _value = Value::new_variable(
        &access,
        &GraceType::new_ty(&Ty::new_uuid(), woog),
        &var,
        woog,
    );

    //
    // `let new = Struct {...}`
    //
    // This is the variable.
    let new = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("new".to_owned(), &table, &new, woog);
    let _ = Value::new_variable(
        &access,
        &GraceType::new_reference(&Reference::new(&obj, woog), woog),
        &var,
        woog,
    );

    // This is the struct.
    let struct_expr =
        StructExpression::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);
    // collect_attributes iterates over all the attributes and relationship-related bits
    // and creates Parameters and StructureFields from them.
    let (mut params, mut fields) = collect_attributes(
        obj,
        &structure,
        &struct_expr,
        &function,
        &table,
        module,
        config,
        domain,
        woog,
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

    // Same-same for the fields. Something that calls us is going to have to do the
    // same thing with all of these statements. I need to keep that in the back of
    // my head.
    // fields.iter_mut().rev().fold(None, |next, field| {
    //     field.next = next;
    //     woog.inter_structure_field(field.clone());
    //     Some(field.id)
    // });

    let expr = Expression::new_struct_expression(&struct_expr, woog);
    // The type of the StructExpression is the object itself.
    let obj_type = domain
        .sarzak()
        .iter_ty()
        .find(|t| t.id() == obj.id)
        .unwrap();
    let ty = GraceType::new_ty(&obj_type, woog);
    let _ = Value::new_expression(&access, &ty, &expr, woog);

    // This is the statement.
    let xlet = XLet::new(&expr, &var, woog);
    let _ = Statement::new_x_let(&block, None, &xlet, woog);
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

    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mutable = Ownership::new_borrowed(&Borrowed::new_mutable(), woog);
    let mut_access = Access::new(&mutable, &public, woog);

    let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

    for subtype in subtypes {
        let s_obj = subtype.r15_object(domain.sarzak())[0];

        let block = Block::new(Uuid::new_v4(), woog);
        let table = SymbolTable::new(&block, woog);

        let method = ObjectMethod::new(s_obj, woog);
        let function = Function::new_object_method(
            format!(
                "Inter a new '{}' in the store, and return it's `id`.",
                s_obj.name
            ),
            "new".to_owned(),
            &block,
            &method,
            woog,
        );

        // This needs to be moved to someplace generic. It's only here as I develop
        // the code.
        //
        // Create the structure definition
        //
        let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);

        //
        // Create statements in the body
        //

        //
        // `let id = Uuid::new_v5(...)`
        let id = Local::new(Uuid::new_v4(), woog);
        let var = Variable::new_local("id".to_owned(), &table, &id, woog);
        let _value = Value::new_variable(
            &access,
            &GraceType::new_ty(&Ty::new_uuid(), woog),
            &var,
            woog,
        );

        //
        // `let new = Struct {...}`
        //
        // This is the variable.
        let new = Local::new(Uuid::new_v4(), woog);
        let var = Variable::new_local("new".to_owned(), &table, &new, woog);
        let _ = Value::new_variable(
            &access,
            &GraceType::new_reference(&Reference::new(&obj, woog), woog),
            &var,
            woog,
        );

        // This is the struct.
        let struct_expr =
            StructExpression::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);
        let (mut params, mut fields) = collect_attributes(
            s_obj,
            &structure,
            &struct_expr,
            &function,
            &table,
            module,
            config,
            domain,
            woog,
        );

        // These are for the "subtype" attribute, which points at the subtype.
        let reference = Reference::new(&s_obj, woog);
        let ty = GraceType::new_reference(&reference, woog);

        // let field = Field::new(SUBTYPE_ATTR.to_owned(), &ty, woog);
        // let field = StructureField::new(None, &field, &structure, woog);
        // fields.insert(0, field);

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
        // fields.iter_mut().rev().fold(None, |next, field| {
        //     field.next = next;
        //     woog.inter_structure_field(field.clone());
        //     Some(field.id)
        // });

        let expr = Expression::new_struct_expression(&struct_expr, woog);
        let obj_type = domain
            .sarzak()
            .iter_ty()
            .find(|t| t.id() == obj.id)
            .unwrap();
        let ty = GraceType::new_ty(&obj_type, woog);
        let _ = Value::new_expression(&access, &ty, &expr, woog);

        // This is the statement.
        let xlet = XLet::new(&expr, &var, woog);
        let _ = Statement::new_x_let(&block, None, &xlet, woog);
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
    const VALUE_FIELD: &str = "ext_value";

    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let owned = Ownership::new_owned();
    let owned_access = Access::new(&owned, &public, woog);

    let mutable = Ownership::new_borrowed(&Borrowed::new_mutable(), woog);
    let mut_access = Access::new(&mutable, &public, woog);

    let block = Block::new(Uuid::new_v4(), woog);
    let table = SymbolTable::new(&block, woog);

    let method = ObjectMethod::new(obj, woog);
    let function = Function::new_object_method(
        format!(
            "Create a new instance of the external entity,  '{}', wrapped in an {}.",
            external.name, obj.name
        ),
        external.ctor.clone(),
        &block,
        &method,
        woog,
    );

    // This needs to be moved to someplace generic. It's only here as I develop
    // the code.
    //
    // Create the structure definition
    //
    let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);

    //
    // Create statements in the body
    //

    //
    // `let id = Uuid::new_v5(...)`
    let id = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("id".to_owned(), &table, &id, woog);
    let _value = Value::new_variable(
        &access,
        &GraceType::new_ty(&Ty::new_uuid(), woog),
        &var,
        woog,
    );

    //
    // `let new = Struct {...}`
    //
    // This is the variable.
    let new = Local::new(Uuid::new_v4(), woog);
    let var = Variable::new_local("new".to_owned(), &table, &new, woog);
    let _ = Value::new_variable(
        &access,
        &GraceType::new_reference(&Reference::new(&obj, woog), woog),
        &var,
        woog,
    );

    // This is the struct.
    let struct_expr =
        StructExpression::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);
    let (mut params, mut fields) = collect_attributes(
        obj,
        &structure,
        &struct_expr,
        &function,
        &table,
        module,
        config,
        domain,
        woog,
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

    // let field = Field::new(VALUE_FIELD.to_owned(), &ty, woog);
    // let field = StructExpressionField::new(VALUE_FIELD.to_owned(), &expr, &structure, None, woog);
    // fields.insert(0, field);

    let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
    let var = Variable::new_parameter(VALUE_FIELD.to_owned(), &table, &param, woog);
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
    // fields.iter_mut().rev().fold(None, |next, field| {
    //     field.next = next;
    //     woog.inter_structure_field(field.clone());
    //     Some(field.id)
    // });

    let expr = Expression::new_struct_expression(&struct_expr, woog);
    let obj_type = domain
        .sarzak()
        .iter_ty()
        .find(|t| t.id() == obj.id)
        .unwrap();
    let ty = GraceType::new_ty(&obj_type, woog);
    let _ = Value::new_expression(&access, &ty, &expr, woog);

    // This is the statement.
    let xlet = XLet::new(&expr, &var, woog);
    let _ = Statement::new_x_let(&block, None, &xlet, woog);
}

/// Create the structure definition
fn make_data_structure(obj: &Object, domain: &Domain, woog: &mut WoogStore) {
    //
    // We should probably be creating an Item, and a Statement to be complete.
    //
    let structure = Structure::new(obj.as_type(&Ownership::new_owned(), woog, domain), woog);

    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mut last_field: Option<Uuid> = None;

    // Collect the attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let ty = GraceType::new_ty(&ty, woog);

        let field = Field::new(attr.as_ident(), None, &ty, woog);
        if let Some(last) = last_field {
            let mut last_field = woog.exhume_field(&last).unwrap().clone();
            last_field.next = Some(field.id);
            woog.inter_field(last_field);
        }
        last_field = Some(field.id);
        let _field = StructureField::new(&field, &structure, woog);
    }

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_binary_referrers_sorted!(&obj, domain.sarzak());
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
                // Build an `Option<UUID>` type
                let uuid = GraceType::new_ty(&Ty::new_uuid(), woog);
                let option = WoogOption::new(&uuid, woog);
                let ty = GraceType::new_woog_option(&option, woog);

                let field = Field::new(referrer.referential_attribute.as_ident(), None, &ty, woog);
                if let Some(last) = last_field {
                    let mut last_field = woog.exhume_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_field(last_field);
                }
                last_field = Some(field.id);
                let _field = StructureField::new(&field, &structure, woog);
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                // Build a UUID type
                let ty = GraceType::new_ty(&Ty::new_uuid(), woog);

                let field = Field::new(referrer.referential_attribute.as_ident(), None, &ty, woog);
                if let Some(last) = last_field {
                    let mut last_field = woog.exhume_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_field(last_field);
                }
                last_field = Some(field.id);
                let _field = StructureField::new(&field, &structure, woog);
            }
        }
    }

    // And the associative attributes
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let referents = get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());

        for referent in referents {
            let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
            let assoc_obj = referent.r25_object(domain.sarzak())[0];

            // Build a UUID type
            let ty = GraceType::new_ty(&Ty::new_uuid(), woog);

            let field = Field::new(an_ass.referential_attribute.as_ident(), None, &ty, woog);
            if let Some(last) = last_field {
                let mut last_field = woog.exhume_field(&last).unwrap().clone();
                last_field.next = Some(field.id);
                woog.inter_field(last_field);
            }
            last_field = Some(field.id);
            let _field = StructureField::new(&field, &structure, woog);
        }
    }

    let sfs = structure
        .r35_structure_field(woog)
        .iter()
        .map(|sf| sf.r35_field(woog))
        .collect::<Vec<_>>();
    dbg!(&sfs);
}

fn collect_attributes(
    obj: &Object,
    structure: &Structure,
    _struct_expr: &StructExpression,
    function: &Function,
    table: &SymbolTable,
    _module: &str,
    _config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> (Vec<Parameter>, Vec<Field>) {
    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    let mut last_field: Option<Uuid> = None;
    let mut last_param: Option<Uuid> = None;

    let mut params = Vec::new();
    let mut fields = Vec::new();

    // Collect the attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let ty = GraceType::new_ty(&ty, woog);

        let field = Field::new(attr.as_ident(), None, &ty, woog);
        if let Some(last) = last_field {
            let mut last_field = woog.exhume_field(&last).unwrap().clone();
            last_field.next = Some(field.id);
            woog.inter_field(last_field);
        }
        last_field = Some(field.id);
        let _field = StructureField::new(&field, &structure, woog);
        fields.push(field);
        // fields.push(attr.as_ident());

        // let value = typecheck_and_coerce()
        // let hack = Hack::new()
        // let field = StructureField::new(attr.as_ident(), &expr, &structure, None, woog);
        // fields.push(field);

        // We are going to generate the id, so don't include it in the
        // list of parameters.
        if attr.name != "id" {
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
            if let Some(last) = last_param {
                let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                last_param.next = Some(param.id);
                woog.inter_parameter(last_param);
            }
            last_param = Some(param.id);
            let var = Variable::new_parameter(attr.as_ident(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty.into(), &var, woog);
            params.push(param);
        }
    }

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_binary_referrers_sorted!(&obj, domain.sarzak());
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
                if let Some(last) = last_param {
                    let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                    last_param.next = Some(param.id);
                    woog.inter_parameter(last_param);
                }
                last_param = Some(param.id);
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

                let field = Field::new(referrer.referential_attribute.as_ident(), None, &ty, woog);
                if let Some(last) = last_field {
                    let mut last_field = woog.exhume_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_field(last_field);
                }
                last_field = Some(field.id);
                let _field = StructureField::new(&field, &structure, woog);
                fields.push(field);

                // let field = StructureField::new(
                // referrer.referential_attribute.as_ident(),
                // &expr,
                // &structure,
                // None,
                // woog,
                // );
                // fields.push(field);
                // fields.push(referrer.referential_attribute.as_ident());
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
                if let Some(last) = last_param {
                    let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                    last_param.next = Some(param.id);
                    woog.inter_parameter(last_param);
                }
                last_param = Some(param.id);
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

                let field = Field::new(referrer.referential_attribute.as_ident(), None, &ty, woog);
                if let Some(last) = last_field {
                    let mut last_field = woog.exhume_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_field(last_field);
                }
                last_field = Some(field.id);
                let _field = StructureField::new(&field, &structure, woog);
                fields.push(field);

                // fields.push(referrer.referential_attribute.as_ident());
            }
        }
    }

    // And the associative attributes
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let referents = get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());

        for referent in referents {
            let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
            let assoc_obj = referent.r25_object(domain.sarzak())[0];

            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);
            if let Some(last) = last_param {
                let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                last_param.next = Some(param.id);
                woog.inter_parameter(last_param);
            }
            last_param = Some(param.id);
            let var = Variable::new_parameter(
                an_ass.referential_attribute.as_ident(),
                &table,
                &param,
                woog,
            );
            let reference = Reference::new(&obj, woog);
            let ty = GraceType::new_reference(&reference, woog);
            let _ = Value::new_variable(&access, &ty, &var, woog);
            params.push(param);

            let field = Field::new(an_ass.referential_attribute.as_ident(), None, &ty, woog);
            if let Some(last) = last_field {
                let mut last_field = woog.exhume_field(&last).unwrap().clone();
                last_field.next = Some(field.id);
                woog.inter_field(last_field);
            }
            last_field = Some(field.id);
            let _field = StructureField::new(&field, &structure, woog);

            // fields.push(field);
        }
    }

    let sfs = structure
        .r35_structure_field(woog)
        .iter()
        .map(|sf| sf.r35_field(woog))
        .collect::<Vec<_>>();
    dbg!(&sfs);

    let ps = function.r5c_parameter(woog);
    dbg!(&ps);

    (params, fields)
}

/// This function takes a type, presumably from the left-hand side of an assignment,
/// and a variable, presumably from the right-hand side of an assignment, and checks
/// that the types are compatible. The result, assuming compatibility, is a string
/// representation of what the right-hand side of the assignment should be in able
/// to match the types.
fn typecheck_and_coerce(
    lhs_ty: &GraceType,
    rhs: &Variable,
    config: &GraceConfig,
    woog: &mut WoogStore,
    domain: &Domain,
) -> Result<String> {
    let rhs_ty = rhs.r7_value(woog)[0].r3_grace_type(woog)[0].clone();

    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    Ok(match &lhs_ty {
        // GraceType::Reference(id) => {}
        GraceType::WoogOption(_) => {
            // ✨ Until this comment changes, i.e., until this is used by more than
            // rendering a new Self item, the type of the lhs option is uuid.
            match &rhs_ty {
                GraceType::WoogOption(id) => {
                    let opt = woog.exhume_woog_option(&id).unwrap();
                    let opt_ty = opt.r20_grace_type(woog)[0];
                    match &opt_ty {
                        GraceType::Reference(id) => {
                            let reference = woog.exhume_reference(&id).unwrap().clone();
                            let object = reference.r13_object(domain.sarzak())[0];

                            if local_object_is_enum(object, config, domain) {
                                // `var.map(|obj| obj.id())`
                                // This shit is going to be rough. I need to return an instance of
                                // Call, that points at a function with the name "map". Map needs
                                // one parameter, that happens to be a closure, which I don't have.
                                // Function should have a subtype of Object Method, so that we can
                                // refer to the variable upon which we are calling map.
                                // There's actually another whole instance of this happening with
                                // the "id" method on "obj". Then the whole thing get's wrapped up
                                // in an expression. Fuck me.
                                // This isn't so bad. I made function a subtype of type. Easy-peasy.
                                // Now to build the AST. This is probably going to be tedious and
                                // error prone.
                                // So Ijust realized that once I build this AST, code generation simply
                                // boils down to walking the tree and spitting out the correct text,
                                // based on the node. Just like a real compiler. 😃

                                // We are generating Foo.map(|foo| foo.id())
                                {
                                    // This creates a closure with a block and a symbol table and a
                                    // parameter called `𝛂`.
                                    let block = Block::new(Uuid::new_v4(), woog);
                                    let table = SymbolTable::new(&block, woog);
                                    let 𝛌 = Function::new_closure(
                                        "𝛌".to_owned(),
                                        "|𝛂| 𝛂.id()".to_owned(),
                                        &block,
                                        woog,
                                    );
                                    let 𝛂_param =
                                        Parameter::new(Uuid::new_v4(), Some(&𝛌), None, woog);
                                    let 𝛂_var = Variable::new_parameter(
                                        "𝛂".to_owned(),
                                        &table,
                                        &𝛂_param,
                                        woog,
                                    );
                                    let 𝛂_val = Value::new_variable(&access, &rhs_ty, &𝛂_var, woog);

                                    // We need to create the `id` method in order to call it.
                                    // We create it on the object from the rhs.
                                    let id = ObjectMethod::new(object, woog);
                                    let id_func = Function::new_object_method(
                                        "Get the id of the enum from it's current subtype"
                                            .to_owned(),
                                        "id".to_owned(),
                                        // This will need to be addressed. I'm guessing that once we are
                                        // generating these things, we'll need to look them up in the
                                        // store. Not the block, the function. That implies that there
                                        // is a dependency tree. That'll make things interesting. I either
                                        // figure out the order to call things, or I make sure that I
                                        // can do it recursively.
                                        &Block::new(Uuid::new_v4(), woog),
                                        &id,
                                        woog,
                                    );

                                    // Call the id method, I think/hope.
                                    // The more I think about it, the more I think that I'm going to
                                    // need a visualization to help debug. Yesterday I was designing
                                    // the Debug implementation generation in my head walking Woog.
                                    let call = Call::new(&id_func, woog);

                                    // Ok, I've managed to generate everything inside of the map method.
                                    // Now I need to invent a map method, I suppose...
                                }

                                format!(
                                    "{}.map(|{}| {}.id())",
                                    rhs.as_ident(),
                                    object.as_ident(),
                                    object.as_ident()
                                )
                            } else {
                                format!(
                                    "{}.map(|{}| {}.id)",
                                    rhs.as_ident(),
                                    object.as_ident(),
                                    object.as_ident()
                                )
                            }
                        }
                        _ => {
                            ensure!(
                                lhs_ty == &rhs_ty,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{:?}`, expected `{:?}`",
                                        rhs_ty, lhs_ty
                                    )
                                }
                            );
                            rhs.as_ident()
                        }
                    }
                }
                _ => {
                    ensure!(
                        lhs_ty == &rhs_ty,
                        CompilerSnafu {
                            description: format!(
                                "type mismatch: found `{:?}`, expected `{:?}`",
                                rhs_ty, lhs_ty
                            )
                        }
                    );
                    rhs.as_ident()
                }
            }
        }
        // GraceType::TimeStamp(id) => {}
        GraceType::Ty(id) => {
            let ty = domain.sarzak().exhume_ty(&id).unwrap();
            match ty {
                Ty::Uuid(_) => {
                    // If the lhs is a uuid, and the rhs is a reference, we need to
                    // pull it's id.
                    match &rhs_ty {
                        GraceType::Reference(id) => {
                            let obj = woog
                                .exhume_reference(&id)
                                .unwrap()
                                .r13_object(domain.sarzak())[0];

                            if local_object_is_enum(obj, config, domain) {
                                format!("{}.id()", rhs.as_ident())
                            } else {
                                format!("{}.id", rhs.as_ident())
                            }
                        }
                        _ => {
                            ensure!(
                                lhs_ty == &rhs_ty,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{:?}`, expected `{:?}`",
                                        rhs_ty, lhs_ty
                                    )
                                }
                            );
                            rhs.as_ident()
                        }
                    }
                }
                _ => {
                    ensure!(
                        lhs_ty == &rhs_ty,
                        CompilerSnafu {
                            description: format!(
                                "type mismatch: found `{:?}`, expected `{:?}`",
                                rhs_ty, lhs_ty
                            )
                        }
                    );
                    rhs.as_ident()
                }
            }
        }
        _ => {
            ensure!(
                lhs_ty == &rhs_ty,
                CompilerSnafu {
                    description: format!(
                        "type mismatch: found `{:?}`, expected `{:?}`",
                        rhs_ty, lhs_ty
                    )
                }
            );
            rhs.as_ident()
        }
    })
}
