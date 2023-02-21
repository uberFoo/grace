//! Generate From trait implementations for use in sarzak Domain
//!
use std::{collections::HashMap, fmt::Write};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_many_r_subs_across_r27,
            sarzak_get_one_ass_to_across_r22, sarzak_get_one_ass_to_across_r23,
            sarzak_get_one_obj_across_r15, sarzak_get_one_obj_across_r16,
            sarzak_get_one_obj_across_r25, sarzak_get_one_r_assoc_across_r21,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_isa_across_r13,
            sarzak_get_one_r_to_across_r5, sarzak_get_one_t_across_r2,
            sarzak_maybe_get_many_ass_froms_across_r26, sarzak_maybe_get_many_r_froms_across_r17,
            sarzak_maybe_get_many_r_sups_across_r14,
        },
        types::{AssociativeReferrer, Attribute, Object, Referrer, Subtype, Supertype, Type},
    },
    v1::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_referrers_sorted, object_is_singleton, object_is_supertype,
        render::{RenderIdent, RenderType},
    },
    options::{FromDomain, GraceConfig},
    types::ObjectStoreDefinition,
};

pub(crate) struct DomainFromBuilder {
    definition: Option<Box<dyn ObjectStoreDefinition>>,
    domain: Option<FromDomain>,
}

impl DomainFromBuilder {
    pub(crate) fn new() -> Self {
        Self {
            definition: None,
            domain: None,
        }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ObjectStoreDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn domain(mut self, domain: FromDomain) -> Self {
        self.domain = Some(domain);

        self
    }

    pub(crate) fn build(&mut self) -> Result<Box<DomainFromGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "DomainFromBuilder::build called before definition".to_owned()
            }
        );

        ensure!(
            self.domain.is_some(),
            CompilerSnafu {
                description: "DomainFromBuilder::build called before domain".to_owned()
            }
        );

        Ok(Box::new(DomainFromGenerator {
            definition: self.definition.take().unwrap(),
            from_domain: self.domain.take().unwrap(),
        }))
    }
}

pub(crate) struct DomainFromGenerator {
    definition: Box<dyn ObjectStoreDefinition>,
    from_domain: FromDomain,
}

impl FileGenerator for DomainFromGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        // Output the domain/module documentation/description
        emit!(buffer, "//! {} Object From Trait Implementations", module);
        emit!(buffer, "//!");
        emit!(
            buffer,
            "//! These are [`From`] trait implementations for the domain: _{}_. They are",
            domain.name()
        );
        emit!(
            buffer,
            "//! generated to be used during the extrusion process. This is the process",
        );
        emit!(
            buffer,
            "//! by which instances of one domain are transformed into instances of another."
        );
        emit!(
            buffer,
            "//! In this case the source domain is `{}`.",
            self.from_domain.module
        );
        emit!(buffer, "//!");
        emit!(
            buffer,
            "//! It is hoped that the model has not changed enough to render"
        );
        emit!(
            buffer,
            "//! these implementations useless. In any case it's expected that"
        );
        emit!(
            buffer,
            "//! the generated code will need to be manually edited."
        );

        // It's expected that this code will be edited, block accordingly.
        buffer.block(
            DirectiveKind::IgnoreGenerated,
            format!("{}-from-impl-file", module),
            |buffer| {
                self.definition
                    .write_code(config, domain, woog, imports, module, obj_id, buffer)?;

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
    }
}

pub(crate) struct DomainFromImpl;

impl DomainFromImpl {
    pub(crate) fn new() -> Box<dyn ObjectStoreDefinition> {
        Box::new(Self)
    }
}

impl ObjectStoreDefinition for DomainFromImpl {}

impl CodeWriter for DomainFromImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            imports.is_some(),
            CompilerSnafu {
                description: "DomainFromImpl::write_code called without imports".to_owned()
            }
        );
        let imports = match imports {
            Some(imports) => imports,
            None => unreachable!(),
        };

        // Get a list of objects that aren't imported, aren't enums, and aren't singletons.
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let (from_name, from_module, objects) = if let Some(from_domain) = config.get_from_domain()
        {
            let name = from_domain
                .module
                .split("::")
                .last()
                .context(CompilerSnafu {
                    description: "failed to split path",
                })?
                .to_owned();

            if let Some(store) = imports.get(&name) {
                (
                    name,
                    from_domain.module,
                    objects
                        .iter()
                        .filter(|(id, obj)| {
                            object_is_supertype(obj, domain.sarzak())
                                || !object_is_singleton(obj, domain.sarzak())
                                    && !config.is_imported(*id)
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                ensure!(
                    false,
                    CompilerSnafu {
                        description: format!("failed to find import for {}", from_domain.module)
                    }
                );
                (name, from_domain.module, Vec::new()) // Keeps the compiler happy
            }
        } else {
            ensure!(
                false,
                CompilerSnafu {
                    description:
                        "DomainFromImpl::write_code called without From Domain configuration"
                            .to_owned()
                }
            );
            ("".to_owned(), "".to_owned(), Vec::new()) // Keeps the compiler happy
        };

        buffer.block(
            DirectiveKind::IgnoreGenerated,
            format!("{}-from-impl-definition", module),
            |buffer| {
                // Generate the use statements
                emit!(buffer, "use crate::{}::ObjectStore;", module,);
                emit!(buffer, "use crate::{}::types::{{", module);
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{},",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                emit!(buffer, "}};");
                emit!(buffer, "");

                emit!(
                    buffer,
                    "use crate::{}::ObjectStore as {}Store;",
                    from_module,
                    from_name.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                emit!(buffer, "use crate::{}::types::{{", from_module);
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{} as From{},",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                emit!(buffer, "}};");
                emit!(buffer, "");

                // Generate the ObjectStore From implementation
                emit!(
                    buffer,
                    "impl From<&{}Store> for ObjectStore {{",
                    from_name.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                emit!(
                    buffer,
                    "fn from(from: &{}Store) -> Self {{",
                    from_name.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                emit!(buffer, "let mut to = ObjectStore::new();");
                for (_, obj) in &objects {
                    emit!(buffer, "");
                    // if object_is_supertype(obj, domain.sarzak()) {
                    //     emit!(
                    //         buffer,
                    //         "// These are just UUID's that are preserved across domains."
                    //     );
                    //     emit!(buffer, "for (id, _) in from.iter_{}() {{", obj.as_ident());
                    //     emit!(
                    //         buffer,
                    //         "let instance = to.exhume_{}(&id).unwrap();",
                    //         obj.as_ident()
                    //     );
                    //     emit!(buffer, "to.inter_{}(instance.clone());", obj.as_ident());
                    //     emit!(buffer, "}}");
                    // } else {
                    emit!(
                        buffer,
                        "for (_, instance) in from.iter_{}() {{",
                        obj.as_ident()
                    );
                    emit!(
                        buffer,
                        "let instance = {}::from(instance);",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "to.inter_{}(instance);", obj.as_ident());
                    emit!(buffer, "}}");
                    // }
                }
                emit!(buffer, "");
                emit!(buffer, "to");
                emit!(buffer, "}}");
                emit!(buffer, "}}");
                emit!(buffer, "");

                // Generate the individual From implementations
                for (_, obj) in &objects {
                    if object_is_supertype(obj, domain.sarzak()) {
                        emit!(buffer, "");
                        emit!(
                            buffer,
                            "impl From<&From{}> for {} {{",
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                        );
                        emit!(
                            buffer,
                            "fn from(src: &From{}) -> Self {{",
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                        );
                        emit!(buffer, "match src {{");
                        // Darnedest thing. Uncommenting the line below causes the compiler to
                        // freak out.
                        // let subtypes = get_subtypes_sorted!(obj, domain.sarzak());
                        // I'm convinced that R14 and R15 are broken.
                        let sup = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                        let isa = sarzak_get_one_r_isa_across_r13!(sup[0], domain.sarzak());
                        let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, domain.sarzak());
                        subtypes.sort_by(|a, b| {
                            let a = sarzak_get_one_obj_across_r15!(a, domain.sarzak());
                            let b = sarzak_get_one_obj_across_r15!(b, domain.sarzak());
                            a.name.cmp(&b.name)
                        });

                        for subtype in subtypes {
                            let s_obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                            emit!(
                                buffer,
                                "From{}::{}(src) => {}::{}(src.clone()),",
                                obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                                s_obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                                obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                                s_obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                            );
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "}}");
                        emit!(buffer, "}}");
                    } else {
                        emit!(
                            buffer,
                            "impl From<&From{}> for {} {{",
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                        );
                        emit!(
                            buffer,
                            "fn from(src: &From{}) -> Self {{",
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                        );
                        emit!(buffer, "Self {{");

                        // Attributes
                        let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                        attrs.sort_by(|a, b| a.name.cmp(&b.name));
                        for attr in attrs {
                            let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());

                            // 🚧: This is sort of a hack. What we really want to do is notice
                            // that the lhs is OWNED, and that the rhs is BORROWED. Then we'd
                            // clone. This is exactly the sort of thing that would belong with
                            // other conversions in codegen.rs. Hiding in the make new function.
                            // Once I get back to generating tests, I'll unify them. Doing so
                            // now is tricky because over there we're using GType's. That should
                            // maybe get unified somehow by then?
                            match ty {
                                Type::String(_) => {
                                    emit!(
                                        buffer,
                                        "{}: src.{}.clone(),",
                                        attr.as_ident(),
                                        attr.as_ident()
                                    )
                                }
                                _ => {
                                    emit!(buffer, "{}: src.{},", attr.as_ident(), attr.as_ident())
                                }
                            }
                        }

                        // Referential Attributes
                        for referrer in get_referrers_sorted!(obj, domain.sarzak()) {
                            let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
                            let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
                            let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());

                            emit!(
                                buffer,
                                "{}: src.{},",
                                referrer.referential_attribute.as_ident(),
                                referrer.referential_attribute.as_ident(),
                            );
                        }
                        for assoc_referrer in
                            sarzak_maybe_get_many_ass_froms_across_r26!(obj, domain.sarzak())
                        {
                            let assoc =
                                sarzak_get_one_r_assoc_across_r21!(assoc_referrer, domain.sarzak());

                            let one = sarzak_get_one_ass_to_across_r23!(assoc, domain.sarzak());
                            let one_obj = sarzak_get_one_obj_across_r25!(one, domain.sarzak());

                            let other = sarzak_get_one_ass_to_across_r22!(assoc, domain.sarzak());
                            let other_obj = sarzak_get_one_obj_across_r25!(other, domain.sarzak());

                            emit!(
                                buffer,
                                "{}: src.{},",
                                assoc_referrer.one_referential_attribute.as_ident(),
                                assoc_referrer.one_referential_attribute.as_ident()
                            );
                            emit!(
                                buffer,
                                "{}: src.{},",
                                assoc_referrer.other_referential_attribute.as_ident(),
                                assoc_referrer.other_referential_attribute.as_ident()
                            );
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "}}");
                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    }
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}
