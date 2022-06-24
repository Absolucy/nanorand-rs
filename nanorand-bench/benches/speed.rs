use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use nanorand::rand::Rng;
use rand_core::{block::BlockRngCore, RngCore, SeedableRng};
use random_fast_rng::Random;

fn criterion_benchmark(c: &mut Criterion) {
	let mut entropy_group = c.benchmark_group("entropy");
	entropy_group.throughput(Throughput::Bytes(std::mem::size_of::<u64>() as u64 * 4096));

	entropy_group.bench_function("system entropy", |b| {
		let mut out = [0u8; std::mem::size_of::<u64>() * 4096];
		b.iter(|| {
			nanorand::entropy::system(&mut out);
		})
	});

	entropy_group.finish();

	let mut wyrand_group = c.benchmark_group("WyRand");
	wyrand_group.throughput(Throughput::Bytes(8));

	wyrand_group.bench_function("nanorand", |b| {
		let mut rng = nanorand::rand::WyRand::new();
		b.iter(|| {
			black_box(rng.rand());
		})
	});

	wyrand_group.bench_function("wyhash", |b| {
		let mut seed = 42_u64;
		b.iter(|| {
			black_box(wyhash::wyrng(&mut seed));
		})
	});

	wyrand_group.finish();

	let mut chacha_group = c.benchmark_group("ChaCha");
	// ChaCha has 512-bit output
	chacha_group.throughput(Throughput::Bytes(64));

	chacha_group.bench_function("nanorand<8>", |b| {
		let mut rng = nanorand::rand::ChaCha8::new();
		b.iter(|| {
			black_box(rng.rand());
		})
	});

	chacha_group.bench_function("nanorand<12>", |b| {
		let mut rng = nanorand::rand::ChaCha12::new();
		b.iter(|| {
			black_box(rng.rand());
		})
	});

	chacha_group.bench_function("nanorand<20>", |b| {
		let mut rng = nanorand::rand::ChaCha20::new();
		b.iter(|| {
			black_box(rng.rand());
		})
	});

	chacha_group.bench_function("rand_chacha<8>", |b| {
		let mut rng = rand_chacha::ChaCha8Core::from_seed([42; 32]);
		b.iter(|| {
			let mut out = Default::default();
			rng.generate(&mut out);
			black_box(out);
		})
	});

	chacha_group.bench_function("rand_chacha<12>", |b| {
		let mut rng = rand_chacha::ChaCha12Core::from_seed([42; 32]);
		b.iter(|| {
			let mut out = Default::default();
			rng.generate(&mut out);
			black_box(out);
		})
	});

	chacha_group.bench_function("rand_chacha<20>", |b| {
		let mut rng = rand_chacha::ChaCha20Core::from_seed([42; 32]);
		b.iter(|| {
			let mut out = Default::default();
			rng.generate(&mut out);
			black_box(out);
		})
	});

	chacha_group.finish();

	let mut pcg32_group = c.benchmark_group("Pcg32");
	pcg32_group.throughput(Throughput::Bytes(4));

	pcg32_group.bench_function("oorandom", |b| {
		let mut rng = oorandom::Rand32::new(42);
		b.iter(|| {
			black_box(rng.rand_u32());
		})
	});

	pcg32_group.bench_function("randomize", |b| {
		let mut rng = randomize::PCG32::seed(42, 0);
		b.iter(|| {
			black_box(rng.next_u32());
		})
	});

	pcg32_group.bench_function("rand_pcg", |b| {
		let mut rng = rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
		b.iter(|| {
			black_box(rng.next_u32());
		})
	});

	pcg32_group.bench_function("random_fast_rng", |b| {
		let mut rng = random_fast_rng::FastRng::new();
		b.iter(|| {
			black_box(rng.get_u32());
		})
	});

	pcg32_group.finish();

	let mut pcg64_group = c.benchmark_group("Pcg64");
	pcg64_group.throughput(Throughput::Bytes(8));

	pcg64_group.bench_function("nanorand", |b| {
		let mut rng = nanorand::rand::Pcg64::new();
		b.iter(|| {
			black_box(rng.rand());
		})
	});

	pcg64_group.bench_function("oorandom", |b| {
		let mut rng = oorandom::Rand64::new(42);
		b.iter(|| {
			black_box(rng.rand_u64());
		})
	});

	pcg64_group.bench_function("randomize", |b| {
		let mut rng = randomize::PCG64::default();
		b.iter(|| {
			black_box(rng.next_u64());
		})
	});

	pcg64_group.bench_function("rand_pcg", |b| {
		let mut rng = rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
		b.iter(|| {
			black_box(rng.next_u64());
		})
	});

	pcg64_group.bench_function("fastrand", |b| {
		let rng = fastrand::Rng::with_seed(42);
		b.iter(|| {
			black_box(rng.u64(..));
		})
	});

	pcg64_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
