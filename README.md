
# nanorand

`nanorand` is a Rust crate, meant for fast  random number generation with **zero required dependencies**.

## Examples

```rs
use nanorand::{RNG, WyRand};

fn main() {
    let mut rng = WyRand::new();
    println!("Random 64-bit number: {}", rng.generate::<u64>());
}
```

## Why should I use this over...

* `rand` - The standard rand crate is a complex beast. It contains unsafe code in the core implementations, and while it has much more options than we do, that's kind of the point. We're straight to the point, while rand is everything and the kitchen sink.  
* `fastrand`, `oorandom`, `random-fast-rng`, or `randomize` - These are all minimal, zero-dep implementations of the PCG family of RNGs (Pcg32 and Pcg64). While these are decent, they are _much_ slower than wyrand (which beats the speed of these Pcg32 implementations while providing 64 random bits), and do not provide CSPRNGs.  
* `getrandom` - The getrandom crate just provides OS entropy sources. It is not meant for random number generation. In fact, it is useful for seeding nanorand's RNGs on platforms where we can't do that ourselves.

Benchmark output (from the [benches/randbench.rs](benches/randbench.rs)) [can be seen in this text file](benchmark-run-9-15-2020.txt). It was ran on a `Standard_D4s_v3` Azure virtual machine.

## RNG Implementations

**RNG**|**nanorand type**|**Output Size**|**Cryptographically Secure**|**Speed**|**Notes**|**Original Implementation**
:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:
wyrand|[nanohash::WyRand](src/rand/wyrand.rs)|64 bits (`u64`)|🚫|4 GB/s||https://github.com/lemire/testingRNG/blob/master/source/wyrand.h
Pcg64|[nanohash::Pcg64](src/rand/pcg64.rs)|64 bits (`u64`)|🚫|1 GB/s||https://github.com/rkern/pcg64
ChaCha|[nanohash::ChaCha](src/rand/chacha.rs)|512 bits (`[u32; 16]`)|✅|90 MB/s (ChaCha8), 40 MB/s (ChaCha20)|Currently only works in **Nightly** Rust, will work with Stable 1.47 (see [rust#74060](https://github.com/rust-lang/rust/pull/74060))|https://cr.yp.to/chacha.html


## Entropy Sources

* Unix-like (Linux, Android, macOS, iOS, FreeBSD, OpenBSD) - first `/dev/urandom`, else `/dev/random`, else system time. (`#[forbid(unsafe_code)]`)
* Windows - `BCryptGenRandom` with system-preferred RNG. (`#[deny(unsafe_code)]`)

## Feature Flags

* `std` (default) - Enables Rust `std` lib features, such as seeding from OS entropy sources.  
* `wyrand` (default) - Enable the "wyrand" RNG.
* `pcg64` (default) - Enable the "Pcg64" RNG.
* `chacha` (**Nightly-only**) - Enable the "ChaCha" RNG.
* `rdseed` - On x86/x86_64 platforms, the `rdseed` intrinsic will be used when OS entropy isn't available.
* `zeroize` - Implement the [Zeroize](https://crates.io/crates/zeroize) trait for all RNGs.

## License

The zlib/libpng License

Copyright (c) `2020` `aspen`

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

1.  The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    an acknowledgment in the product documentation would be appreciated but is
    not required.

2.  Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.

3.  This notice may not be removed or altered from any source distribution.
