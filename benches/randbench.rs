use criterion::{criterion_group, criterion_main, Criterion};
use nanorand::rand::{WyRand, RNG};

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("get entropy from system", |b| {
		b.iter(|| nanorand::entropy::entropy_from_system())
	});

	c.bench_function("wyrand + global state", |b| {
		b.iter(|| WyRand::rand_global())
	});

	c.bench_function("wyrand + internal state", |b| {
		let mut rng = WyRand::new();
		b.iter(|| rng.rand())
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
