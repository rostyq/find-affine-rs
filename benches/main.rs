use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::{Matrix3, Matrix2x3};
use find_affine;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_affine::from_smatrices", |b| {
        b.iter(|| {
            find_affine::from_smatrices(
                black_box(Matrix3::<f64>::new(
                    -1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0
                )),
                black_box(Matrix2x3::<f64>::new(2.0, 3.0, -3.0, -2.0, 1.5, -1.5)),
                0.001
            )
            .unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);