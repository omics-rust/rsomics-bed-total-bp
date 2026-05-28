use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rsomics_bed_total_bp::total_bp;
use std::io::Cursor;

fn make_fixture(n: usize) -> String {
    let mut s = String::with_capacity(n * 35);
    for i in 0..n {
        let start = i as u64 * 1000;
        let end = start + 500;
        s.push_str(&format!("chr1\t{start}\t{end}\n"));
    }
    s
}

fn bench_total_bp(c: &mut Criterion) {
    let fixture = make_fixture(100_000);
    let mut group = c.benchmark_group("bed-total-bp");
    group.throughput(Throughput::Elements(100_000));
    group.bench_function("total_bp_100k", |b| {
        b.iter(|| total_bp(Cursor::new(fixture.as_str())).unwrap());
    });
    group.finish();
}

criterion_group!(benches, bench_total_bp);
criterion_main!(benches);
