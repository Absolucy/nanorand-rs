const CHACHA_TAU: &[u8] = b"expand 16-byte k";

#[inline(always)]
fn chacha_rotl(a: u32, b: u32) -> u32 {
	(a << b) | (a >> (32 - b))
}

#[inline(always)]
fn chacha_quarter_round(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
	x[a] += x[b];
	x[d] ^= x[a];
	x[d] = chacha_rotl(x[d], 16);
	x[c] += x[d];
	x[b] ^= x[c];
	x[b] = chacha_rotl(x[b], 12);
	x[a] += x[b];
	x[d] ^= x[a];
	x[d] = chacha_rotl(x[d], 8);
	x[c] += x[d];
	x[b] ^= x[c];
	x[b] = chacha_rotl(x[b], 7);
}

#[inline(always)]
fn chacha_pack(x: &[u8], a: usize) -> u32 {
	let mut res = 0_u32;
	res |= ((x[a] as u32) << 0 * 8) as u32;
	res |= ((x[a + 1] as u32) << 1 * 8) as u32;
	res |= ((x[a + 2] as u32) << 2 * 8) as u32;
	res |= ((x[a + 3] as u32) << 3 * 8) as u32;
	res
}

pub fn chacha_block(rounds: u8, input: [u32; 16]) -> [u32; 16] {
	let mut x = input;
	if rounds % 2 != 0 {
		panic!("ChaCha rounds must be divisble by 2!")
	}
	for _ in (0..rounds).step_by(2) {
		// Odd rounds
		chacha_quarter_round(&mut x, 0, 4, 8, 12);
		chacha_quarter_round(&mut x, 1, 5, 9, 13);
		chacha_quarter_round(&mut x, 2, 6, 10, 14);
		chacha_quarter_round(&mut x, 3, 7, 11, 15);
		// Even rounds
		chacha_quarter_round(&mut x, 0, 5, 10, 15);
		chacha_quarter_round(&mut x, 1, 6, 11, 14);
		chacha_quarter_round(&mut x, 2, 7, 8, 13);
		chacha_quarter_round(&mut x, 3, 4, 9, 12);
	}
	x.iter_mut()
		.zip(input.iter().copied())
		.for_each(|(l, r)| *l = l.wrapping_add(r));
	x
}

pub fn chacha_init(key: [u8; 32], nonce: [u8; 16]) -> [u32; 16] {
	let mut state = [0u32; 16];
	state[0] = chacha_pack(CHACHA_TAU, 0);
	state[1] = chacha_pack(CHACHA_TAU, 4);
	state[2] = chacha_pack(CHACHA_TAU, 8);
	state[3] = chacha_pack(CHACHA_TAU, 12);

	state[4] = chacha_pack(&key, 0);
	state[5] = chacha_pack(&key, 4);
	state[6] = chacha_pack(&key, 8);
	state[7] = chacha_pack(&key, 12);
	state[8] = chacha_pack(&key, 16);
	state[9] = chacha_pack(&key, 20);
	state[10] = chacha_pack(&key, 24);
	state[11] = chacha_pack(&key, 28);

	state[12] = 0;
	state[13] = chacha_pack(&nonce, 0);
	state[14] = chacha_pack(&nonce, 4);
	state[15] = chacha_pack(&nonce, 8);
	state
}
