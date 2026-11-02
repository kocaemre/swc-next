#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use swc_next_ecma_ast::{Lang, SourceType};
use swc_next_type_strip::{Options, strip_types};

fn bench_files(c: &mut Criterion) {
    let bench_cases = [
        (
            "astro-config",
            include_str!("../files/astro-config.ts"),
            Lang::Ts,
        ),
        (
            "effect-schema-ast",
            include_str!("../files/effect-schema-ast.ts"),
            Lang::Ts,
        ),
        (
            "hono-types",
            include_str!("../files/hono-types.ts"),
            Lang::Ts,
        ),
        ("checker", include_str!("../files/checker.ts"), Lang::Ts),
        (
            "cal.com.tsx",
            include_str!("../files/cal.com.tsx"),
            Lang::Tsx,
        ),
    ];

    for (name, source, lang) in bench_cases {
        let options = Options {
            source_type: SourceType::Module,
            lang,
        };
        c.bench_function(&format!("{name}/type_strip"), |b| {
            b.iter(|| black_box(strip_types(black_box(source), black_box(options))));
        });
    }
}

criterion_group!(benches, bench_files);
criterion_main!(benches);
