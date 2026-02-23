#[macro_use]
extern crate criterion;

use aas::part1::v3_1::environment::Environment;
use criterion::Criterion;
use criterion::criterion_group;

use pprof::criterion::{Output, PProfProfiler};

// Thanks to the example provided by @jebbow in his article
// https://www.jibbow.com/posts/criterion-flamegraphs/

fn parse_dev_aas_json(str: &str) -> Environment {
    serde_json::from_str(str).unwrap()
}

fn bench(c: &mut Criterion) {
    let str = include_str!("./files/json/mvp-dpp-1.0.0.json").to_string();

    c.bench_function("JSON Environment 'Dev' serde-json", |b| {
        b.iter(|| std::hint::black_box(parse_dev_aas_json(&str)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);
