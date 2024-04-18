use criterion::{criterion_group, criterion_main, Criterion};

use rshapes::{traits::Distance, Ellipse, Point2};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("point outside ellipse, but within bbox", |b| {
    let ellipse = Ellipse::new([128, 88].into(), (18, 10));
    let point = Point2::new(145, 96);

    b.iter(|| assert_eq!(ellipse.distance(&point).round(), 3.0));
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
