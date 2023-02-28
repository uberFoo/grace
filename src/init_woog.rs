//! Initialize Woog
//!
//! This involves creating instances in Woog that the compiler stages depend
//! upon.

use sarzak::{
    sarzak::types::{Conditionality, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{
            Access, GraceType, ObjectMethod, Ownership, Parameter, Value, Variable, Visibility,
            BORROWED, PUBLIC,
        },
    },
};

use crate::{
    codegen::{find_store, get_referrers_sorted, render::RenderIdent},
    options::{GraceCompilerOptions, Target},
};

/// Woog post-load domain processing
///
/// Below we add an ObjectMethod instance for each object in the domain.
pub(crate) fn init_woog(
    module: &str,
    options: &GraceCompilerOptions,
    domain: &Domain,
) -> WoogStore {
    let mut woog = WoogStore::new();

    let borrowed = Ownership::Borrowed(BORROWED);
    let public = Visibility::Public(PUBLIC);
    let access = Access::new(&borrowed, &public, &mut woog);

    let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
    objects.sort_by(|a, b| a.name.cmp(&b.name));

    // Iterate over the objects, generating an implementation for file each.
    for obj in objects {
        let method = ObjectMethod::new(
            "Create a new instance".to_owned(),
            "new".to_owned(),
            obj,
            &mut woog,
        );

        // These are more attributes on our object, and they should be sorted.
        let referrers = get_referrers_sorted!(&obj, domain.sarzak());

        // Collect the attributes
        let mut params: Vec<Parameter> = Vec::new();
        let mut attrs = obj.r1_attribute(domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = attr.r2_ty(domain.sarzak())[0];
                let param = Parameter::new(attr.as_ident(), &method, None, &mut woog);
                let var = Variable::new_parameter(&param, &mut woog);
                let value = Value::new_variable(&access, &ty.into(), &var, &mut woog);
                let foo = woog.exhume_variable(&param.id).unwrap();
                assert_eq!(foo.id(), param.id);
                let bar = woog.exhume_value(&param.id).unwrap();
                assert_eq!(bar.id, param.id);

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
                    let param = Parameter::new(
                        referrer.referential_attribute.as_ident(),
                        &method,
                        None,
                        &mut woog,
                    );
                    let var = Variable::new_parameter(&param, &mut woog);
                    let value = Value::new_variable(
                        &access,
                        &GraceType::WoogOption(GraceType::Reference(r_obj.id).id()),
                        &var,
                        &mut woog,
                    );

                    params.push(param);
                }
                // An unconditional reference translates into a reference to the referent.
                Conditionality::Unconditional(_) => {
                    let param = Parameter::new(
                        referrer.referential_attribute.as_ident(),
                        &method,
                        None,
                        &mut woog,
                    );
                    let var = Variable::new_parameter(&param, &mut woog);
                    let value = Value::new_variable(
                        &access,
                        &GraceType::Reference(r_obj.id),
                        &var,
                        &mut woog,
                    );

                    params.push(param);
                }
            }
        }

        if let Target::Domain(_) = options.target {
            // Add the store to the end of the  input parameters
            // let mut iter = sarzak.iter_ty();

            let store = find_store(module, domain);
            let param = Parameter::new("store".to_owned(), &method, None, &mut woog);
            let var = Variable::new_parameter(&param, &mut woog);
            let value = Value::new_variable(
                &access,
                &GraceType::Ty(Ty::External(store.id).id()),
                &var,
                &mut woog,
            );
        }

        // Link the params
        params.iter_mut().rev().fold(None, |next, param| {
            param.next = next;
            Some(param.id)
        });
    }

    woog
}
