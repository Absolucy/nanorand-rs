use nanorand::{Pcg64, RNG};

#[test]
fn pcg64_test() {
	let mut rng = Pcg64::new_seed(42);
	println!("8 bytes of randomness: {}", hex::encode(rng.rand()));

	assert_ne!(rng.rand().to_vec(), vec![0; std::mem::size_of::<u64>()])
}
