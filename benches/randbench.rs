use criterion::{criterion_group, criterion_main, Criterion};
use nanorand::rand::{Pcg64, WyRand, RNG};

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

	c.bench_function("Pcg64", |b| {
		let mut rng = Pcg64::new();
		b.iter(|| rng.rand())
	});

	c.bench_function("wyrand range + global state", |b| {
		b.iter(|| WyRand::rand_global_range(1, 100))
	});

	c.bench_function("wyrand range + internal state", |b| {
		let mut rng = WyRand::new();
		b.iter(|| rng.rand_range(1, 100))
	});

	c.bench_function("Pcg64 range", |b| {
		let mut rng = Pcg64::new();
		b.iter(|| rng.rand_range(1, 100))
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
