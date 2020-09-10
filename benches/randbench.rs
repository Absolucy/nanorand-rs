use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("get entropy from system", |b| {
		b.iter(|| nanorand::entropy::entropy_from_system())
	});

	c.bench_function("wyrand", |b| {
		b.iter(|| nanorand::rand::wyrand::rand_global())
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
