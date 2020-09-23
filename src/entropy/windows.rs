use super::backup_entropy;

extern "system" {
	#[link_name = "SystemFunction036"]
	fn RtlGenRandom(pBuffer: *mut u8, cbBuffer: usize) -> u32;
}

/// Obtain a random 64-bit number using WinAPI's `RtlGenRandom` function.
pub fn entropy_from_system(amt: usize) -> Vec<u8> {
	let mut entropy: Vec<u8> = vec![42; amt];
	let status: u32 = unsafe { RtlGenRandom(entropy.as_mut_ptr(), amt) };
	if status == 0 {
		entropy
	} else {
		backup_entropy(amt)
	}
}
