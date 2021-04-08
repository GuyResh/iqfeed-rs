use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iqfeed_rs::models::Ops;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse trade", |b| {
        b.iter(|| {
            Ops::parse(black_box(
                b"Q,GME,190.0000,1,16:40:18.814943,19,8346145,189.56,190,300,197,199.4600,187.1102,0.0,8717,O,",
            ))
        })
    });

    c.bench_function("parse timestamp", |b| {
        b.iter(|| Ops::parse(black_box(b"T,20210408 14:30:28")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
