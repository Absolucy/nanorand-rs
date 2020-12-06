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

/* Generated with cbindgen:0.15.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace nanorand {

	using ChaCha = ChaCha;

	using Pcg64 = Pcg64;

	using WyRand = WyRand;

	extern "C" {

	uint8_t (chacha_next(ChaCha *rng))[64];

	bool chacha_next_bool(ChaCha *rng);

	uint16_t chacha_next_u16(ChaCha *rng);

	uint32_t chacha_next_u32(ChaCha *rng);

	uint64_t chacha_next_u64(ChaCha *rng);

	uint8_t chacha_next_u8(ChaCha *rng);

	uint16_t chacha_range_u16(ChaCha *rng, uint16_t lower, uint16_t upper);

	uint32_t chacha_range_u32(ChaCha *rng, uint32_t lower, uint32_t upper);

	uint64_t chacha_range_u64(ChaCha *rng, uint64_t lower, uint64_t upper);

	uint8_t chacha_range_u8(ChaCha *rng, uint8_t lower, uint8_t upper);

	ChaCha new_chacha(uint8_t rounds);

	ChaCha new_chacha12();

	ChaCha new_chacha20();

	ChaCha new_chacha8();

	Pcg64 new_pcg64();

	Pcg64 new_pcg64_seed(u128 seed);

	uint8_t (pcg64_next(Pcg64 *rng))[8];

	bool pcg64_next_bool(Pcg64 *rng);

	uint16_t pcg64_next_u16(Pcg64 *rng);

	uint32_t pcg64_next_u32(Pcg64 *rng);

	uint64_t pcg64_next_u64(Pcg64 *rng);

	uint8_t pcg64_next_u8(Pcg64 *rng);

	uint16_t pcg64_range_u16(Pcg64 *rng, uint16_t lower, uint16_t upper);

	uint32_t pcg64_range_u32(Pcg64 *rng, uint32_t lower, uint32_t upper);

	uint64_t pcg64_range_u64(Pcg64 *rng, uint64_t lower, uint64_t upper);

	uint8_t pcg64_range_u8(Pcg64 *rng, uint8_t lower, uint8_t upper);

	uint8_t (wyrand_next(WyRand *rng))[8];

	bool wyrand_next_bool(WyRand *rng);

	uint16_t wyrand_next_u16(WyRand *rng);

	uint32_t wyrand_next_u32(WyRand *rng);

	uint64_t wyrand_next_u64(WyRand *rng);

	uint8_t wyrand_next_u8(WyRand *rng);

	uint16_t wyrand_range_u16(WyRand *rng, uint16_t lower, uint16_t upper);

	uint32_t wyrand_range_u32(WyRand *rng, uint32_t lower, uint32_t upper);

	uint64_t wyrand_range_u64(WyRand *rng, uint64_t lower, uint64_t upper);

	uint8_t wyrand_range_u8(WyRand *rng, uint8_t lower, uint8_t upper);

	} // extern "C"

} // namespace nanorand
