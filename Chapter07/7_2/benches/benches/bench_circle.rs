use benches::Circle;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let c1 = Circle { radius: 1.0 };
    let c2 = Circle { radius: 2.0 };

    c.bench_function("test smaller", |b| b.iter(|| c1.smaller(&c2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
