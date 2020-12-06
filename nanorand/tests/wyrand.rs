use nanorand::{WyRand, RNG};

#[test]
fn wyrand_test() {
	let mut rng = WyRand::new_seed(42);
	println!("8 bytes of randomness: {}", hex::encode(rng.rand()));

	assert_ne!(rng.rand().to_vec(), vec![0; std::mem::size_of::<u64>()])
}
