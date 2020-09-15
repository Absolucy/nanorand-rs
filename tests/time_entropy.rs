use nanorand::entropy::emergency_system_time_entropy;

#[test]
fn system_entropy_test() {
	let test_entropy = emergency_system_time_entropy(16);
	println!("16 bytes of entropy: {}", hex::encode(test_entropy));

	let test_entropy_b = emergency_system_time_entropy(std::mem::size_of::<u64>() * 64);
	let unrandom = test_entropy_b
		.iter()
		.fold(0, |a, number| if *number == 42 { a + 1 } else { a });
	println!(
		"{} bytes of entropy has {} instances of '42'",
		std::mem::size_of::<u64>() * 64,
		unrandom
	);
	assert_ne!(test_entropy_b, vec![42; std::mem::size_of::<u64>() * 64])
}
