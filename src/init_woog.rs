//! Initialize Woog
//!
//! This involves creating instances in Woog that the compiler stages depend
//! upon.

use sarzak::{
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_obj_across_r16,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_to_across_r5,
            sarzak_get_one_t_across_r2, sarzak_maybe_get_many_r_froms_across_r17,
            sarzak_maybe_get_one_t_ref_across_r27,
        },
        store::ObjectStore as SarzakStore,
        types::{Attribute, Object, Referrer, Type, UUID},
    },
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, ObjectMethod, Parameter, Visibility, BORROWED, MUTABLE, PUBLIC},
    },
};
use uuid::Uuid;

use crate::{
    codegen::render::{RenderIdent, RenderType},
    options::{GraceCompilerOptions, Target},
};

/// Woog post-load domain processing
///
/// Below we add an ObjectMethod instance for each object in the domain.
pub(crate) fn init_woog(
    module: &str,
    options: &GraceCompilerOptions,
    sarzak: &SarzakStore,
) -> WoogStore {
    let mut woog = WoogStore::new();
    sarzak::woog::init_instances(&mut woog);

    let mut objects: Vec<(&Uuid, &Object)> = sarzak.iter_object().collect();
    objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    // Iterate over the objects, generating an implementation for file each.
    for (id, obj) in objects {
        // These are more attributes on our object, and they should be sorted.
        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, sarzak);
        referrers.sort_by(|a, b| {
            let obj_a = sarzak.exhume_object(&a.obj_id).unwrap();
            let obj_b = sarzak.exhume_object(&b.obj_id).unwrap();
            obj_a.name.cmp(&obj_b.name)
        });

        // Collect the attributes
        let mut params: Vec<Parameter> = Vec::new();
        let mut attrs = sarzak_get_many_as_across_r1!(obj, sarzak);
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = sarzak_get_one_t_across_r2!(attr, sarzak);
                params.push(Parameter::new(
                    &mut woog,
                    &Mutability::Borrowed(BORROWED),
                    None,
                    &ty,
                    &Visibility::Public(PUBLIC),
                    attr.as_ident(),
                ));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, sarzak);
            let referent = sarzak_get_one_r_to_across_r5!(binary, sarzak);
            let r_obj = sarzak_get_one_obj_across_r16!(referent, sarzak);
            let reference = sarzak_maybe_get_one_t_ref_across_r27!(r_obj, sarzak).unwrap();

            // This determines how a reference is stored in the struct. In this
            // case a UUID.
            params.push(Parameter::new(
                &mut woog,
                &Mutability::Borrowed(BORROWED),
                None,
                &Type::Reference(reference.id),
                &Visibility::Public(PUBLIC),
                referrer.referential_attribute.as_ident(),
            ));
        }

        if let Target::Domain = options.target {
            // Add the store to the end of the  input parameters
            let mut iter = sarzak.iter_ty();
            let name = format!(
                "{}Store",
                module.as_type(&Mutability::Borrowed(BORROWED), sarzak)
            );
            let store_type = loop {
                let ty = iter.next();
                match ty {
                    Some((_, ty)) => match ty {
                        Type::External(e) => {
                            let ext = sarzak.exhume_external(&e).unwrap();
                            if ext.name == name {
                                break ty;
                            }
                        }
                        _ => continue,
                    },
                    None => panic!("Could not find store type for {}", module),
                }
            };
            params.push(Parameter::new(
                &mut woog,
                &Mutability::Mutable(MUTABLE),
                None,
                &store_type,
                &Visibility::Public(PUBLIC),
                "store".to_owned(),
            ));
        }

        // Link the params
        let mut iter = params.iter_mut().peekable();
        loop {
            if let Some(param) = iter.next() {
                if let Some(next) = iter.peek() {
                    param.next = Some(next.id);
                    woog.inter_parameter(param.clone());
                }
            } else {
                break;
            }
        }

        // Create an ObjectMethod
        // The uniqueness of this instance depends on the inputs to it's
        // new method. Param can be None, and two methods on the same
        // object will have the same obj. So it comes down to a unique
        // name for each object. So just "new" should suffice for name,
        // because it's scoped by obj already.
        let param = match params.len() {
            0 => None,
            _ => Some(&params[0]),
        };
        // We need to find the type that corresponds to this object
        let mut iter = sarzak.iter_ty();
        let ty = loop {
            if let Some((id, ty)) = iter.next() {
                if id == &obj.id {
                    break Some(ty);
                }
            } else {
                break None;
            }
        };
        let method = ObjectMethod::new(
            &mut woog,
            param,
            obj,
            ty.unwrap(),
            &Visibility::Public(PUBLIC),
            "new".to_owned(),
            "Create a new instance".to_owned(),
        );
    }

    woog
}
