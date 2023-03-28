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
            Access, Block, Borrowed, Call, Enumeration, EnumerationEnum, EnumerationField,
            Expression, Field, Function, GraceType, Hack, Item, Literal, Local, ObjectMethod,
            Ownership, Parameter, Reference, Referent, Statement, StructExpression,
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
        local_object_is_singleton, local_object_is_struct, local_object_is_supertype,
        object_is_singleton, object_is_supertype,
        render::{RenderIdent, RenderType},
    },
    options::{ExternalEntity, GraceConfig, Target},
    BUILD_DIR, TARGET_DIR,
};

pub(crate) fn init_woog<P: AsRef<Path>>(
    src_path: P,
    config: &GraceConfig,
    domain: &Domain,
) -> WoogStore {
    // Look for a persisted store.
    let mut path = PathBuf::from(src_path.as_ref());
    path.pop();
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(domain.name());

    if path.exists() && !config.get_always_process() {
        panic!("We don't want to load the store yet.");
        log::debug!("Loading Woog store from: {}", path.display());
        WoogStore::load(&path).unwrap_or_else(|e| {
            log::warn!("Failed to load Woog store: {}", e);
            WoogStore::new()
        })
    } else {
        WoogStore::new()
    }
}

pub(crate) fn persist_woog<P: AsRef<Path>>(
    woog: &WoogStore,
    src_path: P,
    domain: &Domain,
) -> Result<()> {
    let mut path = PathBuf::from(src_path.as_ref());
    path.pop();
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(domain.name());

    woog.persist(&path).context(FileSnafu {
        path,
        description: "persisting Woog store".to_owned(),
    })
}

/// Woog post-load domain processing
///
/// Below we add an ObjectMethod instance for each object in the domain.
///
/// We also inter types in woog that exist in sarzak, so that we can access them
/// during code generation.
pub(crate) fn populate_woog(
    module: &str,
    config: &GraceConfig,
    imports: &HashMap<String, Domain>,
    mut woog: &mut WoogStore,
    domain: &Domain,
) -> Result<()> {
    let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
    objects.sort_by(|a, b| a.name.cmp(&b.name));

    // Iterate over the objects and create ObjectMethods for each.
    for obj in objects {
        if !is_object_stale(obj, &woog, domain) {
            log::debug!("Skipping woog for: {}", obj.name);
            continue;
        }

        log::debug!("Populating woog for: {}", obj.name);

        // 🚧 Looking at the structure of this code here, I can see that I should
        // process structs in a func, and hybrids in a func, etc. Do that sooner
        // than later dude.

        // dbg!(local_object_is_struct(obj, config, domain));
        // dbg!(local_object_is_supertype(obj, config, domain));
        // dbg!(local_object_is_hybrid(obj, config, domain));
        // dbg!(local_object_is_singleton(obj, config, domain));

        // This generates a struct for each object.
        if local_object_is_struct(obj, config, domain) {
            make_structure(obj, domain, &mut woog);
        } else if local_object_is_supertype(obj, config, domain) {
            if local_object_is_hybrid(obj, config, domain) {
                // 🚧 Do something clever with the make_data_structure call to add
                // the `subtype` field. Or whatever it is.
                make_hybrid_enumeration(obj, &mut woog, domain);
            } else {
                // Create an enum for the supertype.
                // make_enumeration(obj, &mut woog, domain);
            }
        }

        if config.is_external(&obj.id) {
            log::debug!("Populating woog for external: {}", obj.name);
            let ext = config.get_external(&obj.id).unwrap();
            inter_external_method_new(obj, &ext, module, config, domain, &mut woog)?;
        } else if local_object_is_struct(obj, config, domain) {
            log::debug!("Populating woog for struct: {}", obj.name);
            inter_struct_method_new(obj, module, config, domain, &mut woog)?;
        } else if local_object_is_hybrid(obj, config, domain) {
            log::debug!("Populating woog for hybrid: {}", obj.name);
            inter_hybrid_method_new(obj, module, config, imports, domain, &mut woog)?;
        }
    }

    // Inter types
    for ty in domain.sarzak().iter_ty() {
        let _ = GraceType::new_ty(&ty, &mut woog);
    }

    Ok(())
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
) -> Result<()> {
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

    let method = ObjectMethod::new(Uuid::new_v4(), obj, woog);
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
    let referent = Referent::new_object(&obj, woog);
    let _ = Value::new_variable(
        &access,
        &GraceType::new_reference(&Reference::new(&referent, woog), woog),
        &var,
        woog,
    );

    // This is the struct.
    let struct_expr = StructExpression::new(
        obj.as_type(&Ownership::new_owned(), woog, domain),
        Uuid::new_v4(),
        woog,
    );

    let store = if let Target::Domain(_) = config.get_target() {
        // Add the store to the end of the  input parameters
        let store = find_store(module, &woog, domain);
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);

        let var = Variable::new_parameter("store".to_owned(), &table, &param, woog);
        let external = Ty::External(store.id);
        let ty = GraceType::new_ty(&external, woog);
        let _ = Value::new_variable(&mut_access, &ty, &var, woog);

        Some(param)
    } else {
        None
    };

    // collect_attributes iterates over all the attributes and relationship-related bits
    // and creates Parameters and StructureFields from them.
    collect_attributes(
        obj,
        &struct_expr,
        &function,
        store,
        None,
        &table,
        module,
        config,
        domain,
        woog,
    )?;

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

    Ok(())
}

fn inter_hybrid_method_new(
    obj: &Object,
    module: &str,
    config: &GraceConfig,
    _zimports: &HashMap<String, Domain>,
    domain: &Domain,
    woog: &mut WoogStore,
) -> Result<()> {
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

        let method = ObjectMethod::new(Uuid::new_v4(), obj, woog);
        let function = Function::new_object_method(
            format!(
                "Inter a new `{}' in the store, and return it's `id`.",
                s_obj.name
            ),
            format!("new_{}", s_obj.name),
            &block,
            &method,
            woog,
        );

        //
        // Create statements in the body
        //

        //
        // `let new = Struct {...}`
        //
        // This is the variable.
        let new = Local::new(Uuid::new_v4(), woog);
        let new_var = Variable::new_local("new".to_owned(), &table, &new, woog);
        let referent = Referent::new_object(&obj, woog);
        let _ = Value::new_variable(
            &access,
            &GraceType::new_reference(&Reference::new(&referent, woog), woog),
            &new_var,
            woog,
        );

        // This is the struct.
        let struct_expr = StructExpression::new(
            obj.as_type(&Ownership::new_owned(), woog, domain),
            Uuid::new_v4(),
            woog,
        );

        let store = if let Target::Domain(_) = config.get_target() {
            // Add the store to the end of the  input parameters
            let store = find_store(module, &woog, domain);
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);

            let var = Variable::new_parameter("store".to_owned(), &table, &param, woog);
            let external = Ty::External(store.id);
            let ty = GraceType::new_ty(&external, woog);
            let _ = Value::new_variable(&mut_access, &ty, &var, woog);

            Some(param)
        } else {
            None
        };

        dbg!(&store);

        // let (param, field) = if object_is_singleton(s_obj, config, &Some(imports), domain)?
        // && !object_is_supertype(s_obj, config, &Some(imports), domain)?
        // {
        // 🚧 Add a field

        // (store, None)
        // } else {
        let (param, field) = {
            let referent = Referent::new_object(&s_obj, woog);
            let reference = Reference::new(&referent, woog);
            let ty = GraceType::new_reference(&reference, woog);

            // This is the subtype param to the functions
            // Put this before the store: that's the store.as_ref() below.
            let param = Parameter::new(Uuid::new_v4(), Some(&function), store.as_ref(), woog);

            let var = Variable::new_parameter(SUBTYPE_ATTR.to_owned(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty, &var, woog);

            // This is the subtype field in the struct
            let aux_enum_field = woog
                .iter_hybrid_enum()
                .find(|ae| ae.object == obj.id)
                .unwrap()
                .r46_enumeration(woog)[0]
                .r36_enumeration_field(woog)
                .iter()
                .cloned()
                .find(|ef| {
                    ef.r36_field(woog)
                        .iter()
                        .cloned()
                        .find(|f| match f.r37_grace_type(woog)[0] {
                            GraceType::Reference(id) => {
                                let reference = woog.exhume_reference(&id).unwrap();
                                let referent = reference.r13_referent(woog)[0];
                                match referent {
                                    Referent::Object(id) => {
                                        let obj = domain.sarzak().exhume_object(&id).unwrap();
                                        dbg!(&obj.name, &s_obj.name);
                                        obj.name == s_obj.name
                                    }
                                    _ => false,
                                }
                            }
                            GraceType::Ty(id) => {
                                let ty = domain.sarzak().exhume_ty(&id).unwrap();
                                panic!("{:?}", ty);
                            }
                            道 => unimplemented!("Apparently you need to deal with {:?}", 道),
                        })
                        .is_some()
                })
                .unwrap()
                .clone();

            let referent = Referent::new_enumeration_field(&aux_enum_field, woog);
            let reference = Reference::new(&referent, woog);
            let ty = GraceType::new_reference(&reference, woog);
            let value = typecheck_and_coerce(&ty, &var, config, woog, domain)?;
            // dbg!(&value);
            // let hack = Hack::new(
            //     format!(
            //         "{}Enum::{}(subtype.id)",
            //         obj.as_type(&Ownership::new_owned(), woog, domain),
            //         s_obj.as_type(&Ownership::new_owned(), woog, domain)
            //     ),
            //     woog,
            // );
            let hack = Hack::new(value, woog);
            // dbg!(&hack);
            let literal = Literal::new_hack(&hack, woog);
            let expr = Expression::new_literal(&literal, woog);
            let field = StructExpressionField::new(
                SUBTYPE_ATTR.to_owned(),
                &expr,
                &struct_expr,
                None,
                woog,
            );
            dbg!(&field);

            (Some(param), Some(field))
        };

        // collect_attributes iterates over all the attributes and relationship-related bits
        // and creates Parameters and StructureFields from them.
        collect_attributes(
            obj,
            &struct_expr,
            &function,
            param,
            field,
            &table,
            module,
            config,
            domain,
            woog,
        )?;

        let expr = Expression::new_struct_expression(&struct_expr, woog);
        let obj_type = domain
            .sarzak()
            .iter_ty()
            .find(|t| t.id() == obj.id)
            .unwrap();
        let ty = GraceType::new_ty(&obj_type, woog);
        let _ = Value::new_expression(&access, &ty, &expr, woog);

        // This is the statement.
        let xlet = XLet::new(&expr, &new_var, woog);
        let _ = Statement::new_x_let(&block, None, &xlet, woog);
    }

    Ok(())
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
) -> Result<()> {
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

    let method = ObjectMethod::new(Uuid::new_v4(), obj, woog);
    let function = Function::new_object_method(
        format!(
            "Create a new instance of the external entity, '{}', wrapped in an {}.",
            external.name, obj.name
        ),
        external.ctor.clone(),
        &block,
        &method,
        woog,
    );

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
    let referent = Referent::new_object(&obj, woog);
    let _ = Value::new_variable(
        &access,
        &GraceType::new_reference(&Reference::new(&referent, woog), woog),
        &var,
        woog,
    );

    // This is the struct.
    let struct_expr = StructExpression::new(
        obj.as_type(&Ownership::new_owned(), woog, domain),
        Uuid::new_v4(),
        woog,
    );

    let store = if let Target::Domain(_) = config.get_target() {
        // Add the store to the end of the  input parameters
        let store = find_store(module, &woog, domain);
        let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);

        let var = Variable::new_parameter("store".to_owned(), &table, &param, woog);
        let ty = Ty::External(store.id);
        let ty = GraceType::new_ty(&ty, woog);
        let _ = Value::new_variable(&mut_access, &ty, &var, woog);

        Some(param)
    } else {
        None
    };

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

    let param = Parameter::new(Uuid::new_v4(), Some(&function), store.as_ref(), woog);
    let var = Variable::new_parameter(VALUE_FIELD.to_owned(), &table, &param, woog);

    let _ = Value::new_variable(&owned_access, &ty.into(), &var, woog);

    // This is the value field in the struct
    let hack = Hack::new(VALUE_FIELD.to_owned(), woog);
    let literal = Literal::new_hack(&hack, woog);
    let expr = Expression::new_literal(&literal, woog);
    let field = StructExpressionField::new(VALUE_FIELD.to_owned(), &expr, &struct_expr, None, woog);

    collect_attributes(
        obj,
        &struct_expr,
        &function,
        Some(param),
        Some(field),
        &table,
        module,
        config,
        domain,
        woog,
    )?;

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

    Ok(())
}

/// Create the Structure definition
///
/// This creates a struct, or and enum, (or perhaps both?), including it's fields.
///
/// We lose information doing this, so we can't actually use this struct as a basis
/// for creating a function that takes values by reference. It's got me thinking
/// though. Wy do we take things by reference in new? Why not take the UUID that
/// we are actually interested in? Oh, type-checking!
///
/// I noticed that we are missing visibility for both the data structure as well
/// as it's fields.
fn make_structure(obj: &Object, domain: &Domain, woog: &mut WoogStore) {
    let mut structure = Structure::new(
        obj.as_type(&Ownership::new_owned(), woog, domain),
        None,
        &obj,
        woog,
    );

    Item::new_structure(&structure, woog);

    // Build a UUID type
    let uuid = GraceType::new_ty(&Ty::new_uuid(), woog);

    let mut last_field: Option<Uuid> = None;
    let mut field_zero: Option<Uuid> = None;

    // Collect the attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let ty = GraceType::new_ty(&ty, woog);

        let field = Field::new(attr.as_ident(), None, &ty, woog);

        // This assumes the existence of an attribute.
        if field_zero.is_none() {
            field_zero = Some(field.id);
        }

        if let Some(last) = last_field {
            let mut last_field = woog.exhume_field(&last).unwrap().clone();
            last_field.next = Some(field.id);
            woog.inter_field(last_field);
        }
        last_field = Some(field.id);

        let _field = StructureField::new(&field, &structure, woog);
    }

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_binary_referrers_sorted!(obj, domain.sarzak());
    // And the referential attributes
    for referrer in &referrers {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        let referent = Referent::new_object(&r_obj, woog);
        let reference = Reference::new(&referent, woog);
        let ty = GraceType::new_reference(&reference, woog);

        // This determines how a reference is stored in the struct. In this
        // case a UUID.
        match cond {
            // If it's conditional build a parameter that's an optional reference
            // to the referent.
            Conditionality::Conditional(_) => {
                let option = WoogOption::new(&ty, woog);
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

            let field = Field::new(an_ass.referential_attribute.as_ident(), None, &uuid, woog);

            if let Some(last) = last_field {
                let mut last_field = woog.exhume_field(&last).unwrap().clone();
                last_field.next = Some(field.id);
                woog.inter_field(last_field);
            }
            last_field = Some(field.id);

            let _field = StructureField::new(&field, &structure, woog);
        }
    }

    // Add the zeroth field
    debug_assert!(field_zero.is_some());
    structure.field_zero = field_zero;
    woog.inter_structure(structure);
}

/// Create the Enumeration definition
///
/// This is awkward. We need the first field to create the enumeration, and we
/// can't create any EnumerationField-s until we've built the enumeration. So
/// we need to keep a local copy of the fields to build the EnumerationField-s
/// after we've built the enumeration.
fn make_hybrid_enumeration(obj: &Object, woog: &mut WoogStore, domain: &Domain) {
    let mut fields = Vec::new();

    for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
        let s_obj = subtype.r15_object(domain.sarzak())[0];

        let referent = Referent::new_object(&s_obj, woog);
        let reference = Reference::new(&referent, woog);
        let ty = GraceType::new_reference(&reference, woog);

        let field = Field::new(
            s_obj.as_type(&Ownership::new_owned(), woog, domain),
            None,
            &ty,
            woog,
        );

        fields.push(field);
    }

    let hybrid = woog
        .iter_hybrid_enum()
        .find(|e| e.object == obj.id)
        .unwrap()
        .clone();
    let enumeration = Enumeration::new_hybrid_enum(
        obj.as_type(&Ownership::new_owned(), woog, domain),
        &fields[0],
        &obj,
        &hybrid,
        woog,
    );

    for field in fields {
        let _field = EnumerationField::new(&enumeration, &field, woog);
    }
}

/// Gather together all of the attributes for a type
///
/// 🚧 I'm now wondering if we could be iterating over the [`Structure`] instead of
/// going through all the rigamarole of getting the attributes again.
fn collect_attributes(
    obj: &Object,
    struct_expr: &StructExpression,
    function: &Function,
    tail_params: Option<Parameter>,
    tail_fields: Option<StructExpressionField>,
    table: &SymbolTable,
    _module: &str,
    config: &GraceConfig,
    domain: &Domain,
    woog: &mut WoogStore,
) -> Result<()> {
    let borrowed = woog
        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
        .unwrap()
        .clone();
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, woog);

    // Build a UUID type
    let uuid = GraceType::new_ty(&Ty::new_uuid(), woog);

    let mut last_param_uuid: Option<Uuid> = None;
    let mut last_field_uuid: Option<Uuid> = None;

    // Collect the attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let ty = GraceType::new_ty(&ty, woog);

        // We are going to generate the id, so don't include it in the
        // list of parameters.
        let var = if attr.name != "id" {
            let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);

            if let Some(last) = last_param_uuid {
                let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                last_param.next = Some(param.id);
                woog.inter_parameter(last_param);
            }
            last_param_uuid = Some(param.id);

            let var = Variable::new_parameter(attr.as_ident(), &table, &param, woog);
            let _ = Value::new_variable(&access, &ty.clone().into(), &var, woog);

            var
        } else {
            let local = Local::new(Uuid::new_v4(), woog);
            let var = Variable::new_local(attr.as_ident(), &table, &local, woog);
            let _ = Value::new_variable(&access, &ty.clone().into(), &var, woog);

            var
        };

        let value = typecheck_and_coerce(&ty, &var, config, woog, domain)?;
        let hack = Hack::new(value, woog);
        let literal = Literal::new_hack(&hack, woog);
        let expr = Expression::new_literal(&literal, woog);
        let field = StructExpressionField::new(attr.as_ident(), &expr, struct_expr, None, woog);

        if let Some(last) = last_field_uuid {
            let mut last_field = woog.exhume_struct_expression_field(&last).unwrap().clone();
            last_field.next = Some(field.id);
            woog.inter_struct_expression_field(last_field);
        }
        last_field_uuid = Some(field.id);
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

                if let Some(last) = last_param_uuid {
                    let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                    last_param.next = Some(param.id);
                    woog.inter_parameter(last_param);
                }
                last_param_uuid = Some(param.id);

                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );

                let referent = Referent::new_object(&r_obj, woog);
                let reference = Reference::new(&referent, woog);
                let reference = GraceType::new_reference(&reference, woog);
                let option = WoogOption::new(&reference, woog);
                let ty = GraceType::new_woog_option(&option, woog);
                let _ = Value::new_variable(&access, &ty, &var, woog);

                let value = typecheck_and_coerce(&ty, &var, config, woog, domain)?;
                let hack = Hack::new(value, woog);
                let literal = Literal::new_hack(&hack, woog);
                let expr = Expression::new_literal(&literal, woog);
                let field = StructExpressionField::new(
                    referrer.referential_attribute.as_ident(),
                    &expr,
                    struct_expr,
                    None,
                    woog,
                );

                if let Some(last) = last_field_uuid {
                    let mut last_field =
                        woog.exhume_struct_expression_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_struct_expression_field(last_field);
                }
                last_field_uuid = Some(field.id);
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                let param = Parameter::new(Uuid::new_v4(), Some(&function), None, woog);

                if let Some(last) = last_param_uuid {
                    let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                    last_param.next = Some(param.id);
                    woog.inter_parameter(last_param);
                }
                last_param_uuid = Some(param.id);

                let var = Variable::new_parameter(
                    referrer.referential_attribute.as_ident(),
                    &table,
                    &param,
                    woog,
                );

                let referent = Referent::new_object(&r_obj, woog);
                let reference = Reference::new(&referent, woog);
                let ty = GraceType::new_reference(&reference, woog);
                let _ = Value::new_variable(&access, &ty, &var, woog);

                let value = typecheck_and_coerce(&uuid, &var, config, woog, domain)?;
                let hack = Hack::new(value, woog);
                let literal = Literal::new_hack(&hack, woog);
                let expr = Expression::new_literal(&literal, woog);
                let field = StructExpressionField::new(
                    referrer.referential_attribute.as_ident(),
                    &expr,
                    struct_expr,
                    None,
                    woog,
                );

                if let Some(last) = last_field_uuid {
                    let mut last_field =
                        woog.exhume_struct_expression_field(&last).unwrap().clone();
                    last_field.next = Some(field.id);
                    woog.inter_struct_expression_field(last_field);
                }
                last_field_uuid = Some(field.id);
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

            if let Some(last) = last_param_uuid {
                let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
                last_param.next = Some(param.id);
                woog.inter_parameter(last_param);
            }
            last_param_uuid = Some(param.id);

            let var = Variable::new_parameter(
                an_ass.referential_attribute.as_ident(),
                &table,
                &param,
                woog,
            );

            let referent = Referent::new_object(&assoc_obj, woog);
            let reference = Reference::new(&referent, woog);
            let ty = GraceType::new_reference(&reference, woog);
            let _ = Value::new_variable(&access, &ty, &var, woog);

            let value = typecheck_and_coerce(&uuid, &var, config, woog, domain)?;
            let hack = Hack::new(value, woog);
            let literal = Literal::new_hack(&hack, woog);
            let expr = Expression::new_literal(&literal, woog);
            let field = StructExpressionField::new(
                an_ass.referential_attribute.as_ident(),
                &expr,
                struct_expr,
                None,
                woog,
            );

            if let Some(last) = last_field_uuid {
                let mut last_field = woog.exhume_struct_expression_field(&last).unwrap().clone();
                last_field.next = Some(field.id);
                woog.inter_struct_expression_field(last_field);
            }
            last_field_uuid = Some(field.id);
        }
    }

    // Add on the things sent to us.
    if let Some(tail) = tail_params {
        if let Some(last) = last_param_uuid {
            let mut last_param = woog.exhume_parameter(&last).unwrap().clone();
            last_param.next = Some(tail.id);
            woog.inter_parameter(last_param);
        }
    }

    if let Some(tail) = tail_fields {
        if let Some(last) = last_field_uuid {
            let mut last_field = woog.exhume_struct_expression_field(&last).unwrap().clone();
            last_field.next = Some(tail.id);
            woog.inter_struct_expression_field(last_field);
        }
    }

    Ok(())
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
                            let reference = woog.exhume_reference(&id).unwrap();
                            let referent = reference.r13_referent(woog)[0];
                            let object = match referent {
                                Referent::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                                Referent::EnumerationField(id) => {
                                    let id = woog
                                        .exhume_enumeration_field(&id)
                                        .unwrap()
                                        .r36_enumeration(woog)[0]
                                        .r40_object(domain.sarzak())[0]
                                        .id;
                                    domain.sarzak().exhume_object(&id).unwrap()
                                }
                                _ => unimplemented!(),
                            };
                            dbg!(&referent, &object);

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
                                    let _𝛂_val =
                                        Value::new_variable(&access, &rhs_ty, &𝛂_var, woog);

                                    // We need to create the `id` method in order to call it.
                                    // We create it on the object from the rhs.
                                    let id = ObjectMethod::new(Uuid::new_v4(), object, woog);
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
                                    let _call = Call::new(&id_func, woog);

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
                                        "unable to coerce type from `{}: {}` to `{}`",
                                        rhs.as_ident(),
                                        display_type(&rhs_ty, woog, domain),
                                        display_type(&lhs_ty, woog, domain)
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
                                "unable to coerce type from `{}: {}` to `{}`",
                                rhs.as_ident(),
                                display_type(&rhs_ty, woog, domain),
                                display_type(&lhs_ty, woog, domain)
                            )
                        }
                    );
                    rhs.as_ident()
                }
            }
        }
        GraceType::Reference(id) => {
            let reference = woog.exhume_reference(&id).unwrap();
            let referent = reference.r13_referent(woog)[0];
            match referent {
                // Referent::Object(id) => domain.sarzak().exhume_object(&id).unwrap().as_ident(),
                Referent::Object(id) => "woof".to_owned(),
                Referent::EnumerationField(id) => {
                    let field = woog.exhume_enumeration_field(&id).unwrap();
                    let object = match field.r36_enumeration(woog)[0].subtype {
                        EnumerationEnum::HybridEnum(h) => woog.exhume_hybrid_enum(&h).unwrap(),
                    };
                    // let subtype = domain.sarzak().exhume_object(&hybrid.object).unwrap();
                    let subtype = match field.r36_field(woog)[0].r37_grace_type(woog)[0] {
                        GraceType::Reference(id) => {
                            let reference = woog.exhume_reference(&id).unwrap();
                            match reference.r13_referent(woog)[0] {
                                Referent::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                                _ => unimplemented!(),
                            }
                        }
                        _ => unimplemented!(),
                    };
                    format!(
                        "{}::{}({}.id)",
                        object.as_type(&Ownership::new_owned(), woog, domain),
                        subtype.as_type(&Ownership::new_owned(), woog, domain),
                        rhs.as_ident()
                    )
                }
                _ => unimplemented!(),
            }

            // dbg!(&referent, &object);
            // dbg!(woog.exhume_hybrid_enum_field(&referent.id()));
        }
        GraceType::Ty(id) => {
            let ty = domain.sarzak().exhume_ty(&id).unwrap();
            match ty {
                Ty::Uuid(_) => {
                    // If the lhs is a uuid, and the rhs is a reference, we need to
                    // pull it's id.
                    match &rhs_ty {
                        GraceType::Reference(id) => {
                            let reference = woog.exhume_reference(&id).unwrap();
                            let referent = reference.r13_referent(woog)[0];
                            let obj = match referent {
                                Referent::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                                Referent::EnumerationField(id) => woog
                                    .exhume_enumeration_field(&id)
                                    .unwrap()
                                    .r36_enumeration(woog)[0]
                                    .r40_object(domain.sarzak())[0],
                                _ => unimplemented!(),
                            };
                            dbg!(&referent, &obj);

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
                                        "unable to coerce type from `{}: {}` to `{}`",
                                        rhs.as_ident(),
                                        display_type(&rhs_ty, woog, domain),
                                        display_type(&lhs_ty, woog, domain)
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
                                "unable to coerce type from `{}: {}` to `{}`",
                                rhs.as_ident(),
                                display_type(&rhs_ty, woog, domain),
                                display_type(&lhs_ty, woog, domain)
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
                        "unable to coerce type from `{}: {}` to `{}`",
                        rhs.as_ident(),
                        display_type(&rhs_ty, woog, domain),
                        display_type(&lhs_ty, woog, domain)
                    ),
                }
            );
            rhs.as_ident()
        }
    })
}

fn display_type(ty: &GraceType, woog: &WoogStore, domain: &Domain) -> String {
    match ty {
        GraceType::Reference(id) => {
            let reference = woog.exhume_reference(&id).unwrap();
            let referent = reference.r13_referent(woog)[0];
            let object = match referent {
                Referent::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                Referent::EnumerationField(id) => woog
                    .exhume_enumeration_field(&id)
                    .unwrap()
                    .r36_enumeration(woog)[0]
                    .r40_object(domain.sarzak())[0],
                _ => unimplemented!(),
            };
            dbg!(&referent, &object);
            format!("&{}", object.as_type(&Ownership::new_owned(), woog, domain))
        }
        foo => unimplemented!("display_type needs to handle type: {:?}", foo),
    }
}
