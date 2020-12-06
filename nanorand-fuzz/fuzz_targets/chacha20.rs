#![no_main]
use c2_chacha::{
	stream_cipher::{generic_array::GenericArray, NewStreamCipher, SyncStreamCipher},
	ChaCha20,
};
use libfuzzer_sys::fuzz_target;
use nanorand::crypto::chacha;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 8;

fuzz_target!(|data: ([u8; KEY_LEN], [u8; NONCE_LEN])| {
	let (key, nonce) = data;

	let reference_keystream = {
		let mut state = ChaCha20::new(
			GenericArray::from_slice(&key),
			GenericArray::from_slice(&nonce),
		);
		let mut keystream = [0u8; 256];
		state.apply_keystream(&mut keystream);
		keystream
	};

	let our_keystream = {
		let mut state = chacha::chacha_init(key, nonce);
		let mut keystream: Vec<u8> = Vec::with_capacity(reference_keystream.len());

		while reference_keystream.len() > keystream.len() {
			chacha::chacha_block(20, state)
				.iter()
				.for_each(|packed| keystream.extend_from_slice(&packed.to_le_bytes()));
			chacha::chacha_increment_counter(&mut state);
		}
		keystream
	};

	assert_eq!(our_keystream, reference_keystream);
});
