use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use nanorand::rand::{Pcg64, WyRand, RNG};

fn criterion_benchmark(c: &mut Criterion) {
	let mut entropy_group = c.benchmark_group("entropy");
	entropy_group.throughput(Throughput::Bytes(std::mem::size_of::<u64>() as u64 * 4096));

	entropy_group.bench_function("time-based entropy", |b| {
		b.iter(|| {
			black_box(nanorand::entropy::emergency_system_time_entropy(
				std::mem::size_of::<u64>() * 4096,
			));
		})
	});

	entropy_group.bench_function("system entropy", |b| {
		b.iter(|| {
			black_box(nanorand::entropy::entropy_from_system(
				std::mem::size_of::<u64>() * 4096,
			));
		})
	});
	entropy_group.finish();

	let mut rng_group = c.benchmark_group("rngs");
	rng_group.throughput(Throughput::Bytes(std::mem::size_of::<u64>() as u64 * 1024));

	rng_group.bench_function("wyrand", |b| {
		let mut rng = WyRand::new();
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.generate());
			}
			black_box(n);
		})
	});

	rng_group.bench_function("pcg64", |b| {
		let mut rng = Pcg64::new();
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.generate());
			}
			black_box(n);
		})
	});

	rng_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
