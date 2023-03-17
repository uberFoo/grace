use criterion::{black_box, criterion_group, criterion_main, Criterion};

use grace::{DomainConfig, GraceCompilerOptions, ModelCompiler, SarzakModelCompiler, Target};
use sarzak::domain::DomainBuilder;

fn generate_everything() {
    let mut options = GraceCompilerOptions::default();
    options.target = Target::Domain(DomainConfig {
        from_module: None,
        from_path: None,
        persist: true,
        persist_timestamps: false,
    });
    if let Some(ref mut derive) = options.derive {
        derive.push("Clone".to_string());
        derive.push("Deserialize".to_string());
        derive.push("Serialize".to_string());
    }
    options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
    options.always_process = Some(true);

    let grace = ModelCompiler::default();

    // Build the domains
    let domain = DomainBuilder::new()
        .cuckoo_model("benches/everything.json")
        .unwrap()
        .build_v2()
        .unwrap();

    grace
        .compile(
            domain,
            "mdd",
            "domain/everything",
            "/tmp/bench/everything",
            Box::new(&options),
            false,
        )
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate_everything", |b| b.iter(|| generate_everything()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
