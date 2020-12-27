/*
 * Copyright (c) 2020 aspen
 * This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held
 * liable for any damages arising from the use of this software.
 * Permission is granted to anyone to use this software for any purpose, including commercial applications, and to
 * alter it and redistribute it freely, subject to the following restrictions:
 *   1. The origin of this software must not be misrepresented; you must not claim that you wrote the original
 *      software. If you use this software in a product, an acknowledgment in the product documentation would be
 *      appreciated but is not required.
 *   2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original
 *      software.
 *   3. This notice may not be removed or altered from any source distribution.
*/


#pragma once

/* Generated with cbindgen:0.16.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

typedef char u128_placeholder[16];


namespace nanorand {

/**
 * An instance of the ChaCha random number generator.
 * Seeded from the system entropy generator when available.
 * **This generator _is theoretically_ cryptographically secure.**
 */
struct ChaCha {
    uint32_t state[16];
    uint8_t rounds;
};

/**
 * An instance of the Pcg64 random number generator.
 * Seeded from the system entropy generator when available.
 * **This generator is _NOT_ cryptographically secure.**
 */
struct Pcg64 {
    u128_placeholder seed;
    u128_placeholder state;
    u128_placeholder inc;
};

/**
 * An instance of the WyRand random number generator.
 * Seeded from the system entropy generator when available.
 * **This generator is _NOT_ cryptographically secure.**
 * #[repr(transparent)]
 */
using WyRand = uint64_t;

extern "C" {

/**
 * Get the raw 512-bit output from the provided RNG.
 * You need to free this yourself!
 */
uint8_t *chacha_next(ChaCha *rng);

/**
 * Generate a random boolean value from the provided RNG
 */
bool chacha_next_bool(ChaCha *rng);

/**
 * Generate a random 16-bit unsigned integer from the provided RNG
 */
uint16_t chacha_next_u16(ChaCha *rng);

/**
 * Generate a random 32-bit unsigned integer from the provided RNG
 */
uint32_t chacha_next_u32(ChaCha *rng);

/**
 * Generate a random 64-bit unsigned integer from the provided RNG
 */
uint64_t chacha_next_u64(ChaCha *rng);

/**
 * Generate a random 8-bit unsigned integer from the provided RNG
 */
uint8_t chacha_next_u8(ChaCha *rng);

/**
 * Generate a random 16-bit unsigned integer within a specified range from the provided RNG
 */
uint16_t chacha_range_u16(ChaCha *rng, uint16_t lower, uint16_t upper);

/**
 * Generate a random 32-bit unsigned integer within a specified range from the provided RNG
 */
uint32_t chacha_range_u32(ChaCha *rng, uint32_t lower, uint32_t upper);

/**
 * Generate a random 64-bit unsigned integer within a specified range from the provided RNG
 */
uint64_t chacha_range_u64(ChaCha *rng, uint64_t lower, uint64_t upper);

/**
 * Generate a random 8-bit unsigned integer within a specified range from the provided RNG
 */
uint8_t chacha_range_u8(ChaCha *rng, uint8_t lower, uint8_t upper);

/**
 * Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
 */
uintptr_t chacha_range_usize(ChaCha *rng, uintptr_t lower, uintptr_t upper);

/**
 * Create a ChaCha RNG, using the specified number of rounds
 */
ChaCha new_chacha(uint8_t rounds);

/**
 * Create a ChaCha RNG, using 12 rounds
 */
ChaCha new_chacha12();

/**
 * Create a ChaCha RNG, using 20 rounds
 */
ChaCha new_chacha20();

/**
 * Create a ChaCha RNG using 8 rounds
 */
ChaCha new_chacha8();

/**
 * Create a new Pcg64 RNG, using system-provided entropy.
 */
Pcg64 new_pcg64();

/**
 * Create a new Pcg64 RNG, using system-provided entropy.
 */
WyRand new_wyrand();

/**
 * Create a new Pcg64 RNG, using a given seed.
 */
WyRand new_wyrand_with_seed(uint64_t seed);

/**
 * Get the raw 64-bit output from the provided RNG.
 * You need to free this yourself!
 */
uint8_t *pcg64_next(Pcg64 *rng);

/**
 * Generate a random boolean value from the provided RNG
 */
bool pcg64_next_bool(Pcg64 *rng);

/**
 * Generate a random 16-bit unsigned integer from the provided RNG
 */
uint16_t pcg64_next_u16(Pcg64 *rng);

/**
 * Generate a random 32-bit unsigned integer from the provided RNG
 */
uint32_t pcg64_next_u32(Pcg64 *rng);

/**
 * Generate a random 64-bit unsigned integer from the provided RNG
 */
uint64_t pcg64_next_u64(Pcg64 *rng);

/**
 * Generate a random 8-bit unsigned integer from the provided RNG
 */
uint8_t pcg64_next_u8(Pcg64 *rng);

/**
 * Generate a random pointer-sized unsigned integer from the provided RNG
 */
uintptr_t pcg64_next_usize(Pcg64 *rng);

/**
 * Generate a random 16-bit unsigned integer within a specified range from the provided RNG
 */
uint16_t pcg64_range_u16(Pcg64 *rng, uint16_t lower, uint16_t upper);

/**
 * Generate a random 32-bit unsigned integer within a specified range from the provided RNG
 */
uint32_t pcg64_range_u32(Pcg64 *rng, uint32_t lower, uint32_t upper);

/**
 * Generate a random 64-bit unsigned integer within a specified range from the provided RNG
 */
uint64_t pcg64_range_u64(Pcg64 *rng, uint64_t lower, uint64_t upper);

/**
 * Generate a random 8-bit unsigned integer within a specified range from the provided RNG
 */
uint8_t pcg64_range_u8(Pcg64 *rng, uint8_t lower, uint8_t upper);

/**
 * Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
 */
uintptr_t pcg64_range_usize(Pcg64 *rng, uintptr_t lower, uintptr_t upper);

/**
 * Get the raw 64-bit output from the provided RNG.
 * You need to free this yourself!
 */
uint8_t *wyrand_next(WyRand *rng);

/**
 * Generate a random boolean value from the provided RNG
 */
bool wyrand_next_bool(WyRand *rng);

/**
 * Generate a random 16-bit unsigned integer from the provided RNG
 */
uint16_t wyrand_next_u16(WyRand *rng);

/**
 * Generate a random 32-bit unsigned integer from the provided RNG
 */
uint32_t wyrand_next_u32(WyRand *rng);

/**
 * Generate a random 64-bit unsigned integer from the provided RNG
 */
uint64_t wyrand_next_u64(WyRand *rng);

/**
 * Generate a random 8-bit unsigned integer from the provided RNG
 */
uint8_t wyrand_next_u8(WyRand *rng);

/**
 * Generate a random pointer-sized unsigned integer from the provided RNG
 */
uintptr_t wyrand_next_usize(WyRand *rng);

/**
 * Generate a random 16-bit unsigned integer within a specified range from the provided RNG
 */
uint16_t wyrand_range_u16(WyRand *rng, uint16_t lower, uint16_t upper);

/**
 * Generate a random 32-bit unsigned integer within a specified range from the provided RNG
 */
uint32_t wyrand_range_u32(WyRand *rng, uint32_t lower, uint32_t upper);

/**
 * Generate a random 64-bit unsigned integer within a specified range from the provided RNG
 */
uint64_t wyrand_range_u64(WyRand *rng, uint64_t lower, uint64_t upper);

/**
 * Generate a random 8-bit unsigned integer within a specified range from the provided RNG
 */
uint8_t wyrand_range_u8(WyRand *rng, uint8_t lower, uint8_t upper);

/**
 * Generate a random pointer-sized unsigned integer within a specified range from the provided RNG
 */
uintptr_t wyrand_range_usize(WyRand *rng, uintptr_t lower, uintptr_t upper);

} // extern "C"

} // namespace nanorand
