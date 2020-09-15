use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use nanorand::rand::RNG;
use rand_core::RngCore;
use random_fast_rng::Random;

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

	#[cfg(feature = "wyrand")]
	{
		rng_group.bench_function("wyrand", |b| {
			let mut rng = nanorand::rand::WyRand::new();
			b.iter(|| {
				let mut n: u64 = u64::MIN;
				for _ in 0..1024 {
					n = n.wrapping_add(rng.generate());
				}
				black_box(n);
			})
		});
	}

	#[cfg(feature = "pcg64")]
	{
		rng_group.bench_function("pcg64", |b| {
			let mut rng = nanorand::rand::Pcg64::new();
			b.iter(|| {
				let mut n: u64 = u64::MIN;
				for _ in 0..1024 {
					n = n.wrapping_add(rng.generate());
				}
				black_box(n);
			})
		});
	}

	#[cfg(feature = "chacha")]
	{
		rng_group.bench_function("chacha8", |b| {
			let mut rng = nanorand::rand::ChaCha::new(8);
			b.iter(|| {
				let mut n: u64 = u64::MIN;
				for _ in 0..1024 {
					n = n.wrapping_add(rng.generate());
				}
				black_box(n);
			})
		});

		rng_group.bench_function("chacha20", |b| {
			let mut rng = nanorand::rand::ChaCha::new(20);
			b.iter(|| {
				let mut n: u64 = u64::MIN;
				for _ in 0..1024 {
					n = n.wrapping_add(rng.generate());
				}
				black_box(n);
			})
		});
	}

	rng_group.finish();

	let mut other_rngs = c.benchmark_group("other-rng-crates");
	other_rngs.throughput(Throughput::Bytes(std::mem::size_of::<u64>() as u64 * 1024));

	other_rngs.bench_function("wyhash wyrand", |b| {
		let mut seed = 42_u64;
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(wyhash::wyrng(&mut seed));
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("oorandom pcg32", |b| {
		let mut rng = oorandom::Rand32::new(42);
		b.iter(|| {
			let mut n: u32 = u32::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.rand_u32());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("oorandom pcg64", |b| {
		let mut rng = oorandom::Rand64::new(42);
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.rand_u64());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("randomize pcg32", |b| {
		let mut rng = randomize::PCG32::seed(42, 0);
		b.iter(|| {
			let mut n: u32 = u32::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.next_u32());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("randomize pcg64", |b| {
		let mut rng = randomize::PCG64::seed(42, 0);
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.next_u64());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("rand pcg32", |b| {
		let mut rng = rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
		b.iter(|| {
			let mut n: u32 = u32::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.next_u32());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("rand pcg64", |b| {
		let mut rng = rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.next_u64());
			}
			black_box(n);
		})
	});

	other_rngs.bench_function("random-fast-rng pcg32", |b| {
		let mut rng = random_fast_rng::FastRng::new();
		b.iter(|| {
			let mut n: u64 = u64::MIN;
			for _ in 0..1024 {
				n = n.wrapping_add(rng.gen());
			}
			black_box(n);
		})
	});

	other_rngs.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
