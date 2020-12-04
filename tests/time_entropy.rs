use nanorand::entropy::emergency_system_time_entropy;

#[test]
fn system_entropy_test() {
	let mut test_entropy_out = [0u8; 16];
	emergency_system_time_entropy(&mut test_entropy_out);
	println!("16 bytes of entropy: {}", hex::encode(test_entropy_out));

	let mut test_entropy_b_out = [0u8; std::mem::size_of::<u64>() * 6];
	emergency_system_time_entropy(&mut test_entropy_b_out);
	let unrandom = test_entropy_b_out
		.iter()
		.fold(0, |a, number| if *number == 0 { a + 1 } else { a });
	println!(
		"{} bytes of entropy has {} instances of '0'",
		std::mem::size_of::<u64>() * 64,
		unrandom
	);
	assert_ne!(test_entropy_b_out, [0u8; std::mem::size_of::<u64>() * 6])
}
