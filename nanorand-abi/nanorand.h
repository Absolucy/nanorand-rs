/*
 * Copyright (c) 2022 Lucy <lucy@absolucy.moe>
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

#ifndef NANORAND_H
#define NANORAND_H

#define DEFINE_NANORAND_BASE(type, buffered_type, name)                                                                \
    type *new_##name(void);                                                                                            \
    void free_##name(type *rng);                                                                                       \
    void name##_fill(type *rng, void *buffer, usize length);                                                           \
    buffered_type *name##_buffered(type *rng);                                                                         \
    void free_##name##_buffered(buffered_type *rng);                                                                   \
    void name##_buffered_fill(buffered_type *rng, void *buffer, usize length);

#define DEFINE_NANORAND_RANGES(type, name)                                                                             \
    u8 name##_range_u8(type *rng, u8 lower, u8 upper);                                                                 \
    u16 name##_range_u16(type *rng, u16 lower, u16 upper);                                                             \
    u32 name##_range_u32(type *rng, u32 lower, u32 upper);                                                             \
    u64 name##_range_u64(type *rng, u64 lower, u64 upper);                                                             \
    usize name##_range_usize(type *rng, usize lower, usize upper);                                                     \
    i8 name##_range_i8(type *rng, i8 lower, i8 upper);                                                                 \
    i16 name##_range_i16(type *rng, i16 lower, i16 upper);                                                             \
    i32 name##_range_i32(type *rng, i32 lower, i32 upper);                                                             \
    i64 name##_range_i64(type *rng, i64 lower, i64 upper);                                                             \
    isize name##_range_isize(type *rng, isize lower, isize upper);

#include <stdint.h>
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;
typedef uintptr_t usize;
typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;
typedef intptr_t isize;

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct _WyRand WyRand;
    typedef struct _BufferedWyRand BufferedWyRand;
    DEFINE_NANORAND_BASE(WyRand, BufferedWyRand, wyrand);
    DEFINE_NANORAND_RANGES(WyRand, wyrand);

    typedef struct _Pcg64 Pcg64;
    typedef struct _BufferedPcg64 BufferedPcg64;
    DEFINE_NANORAND_BASE(Pcg64, BufferedPcg64, pcg64);
    DEFINE_NANORAND_RANGES(Pcg64, pcg64);

    typedef struct _ChaCha8 ChaCha8;
    typedef struct _BufferedChaCha8 BufferedChaCha8;
    DEFINE_NANORAND_BASE(ChaCha8, BufferedChaCha8, chacha8);
    DEFINE_NANORAND_RANGES(ChaCha8, chacha8);

    typedef struct _ChaCha12 ChaCha12;
    typedef struct _BufferedChaCha12 BufferedChaCha12;
    DEFINE_NANORAND_BASE(ChaCha12, BufferedChaCha12, chacha12);
    DEFINE_NANORAND_RANGES(ChaCha12, chacha12);

    typedef struct _ChaCha20 ChaCha20;
    typedef struct _BufferedChaCha20 BufferedChaCha20;
    DEFINE_NANORAND_BASE(ChaCha20, BufferedChaCha20, chacha20);
    DEFINE_NANORAND_RANGES(ChaCha20, chacha20);

#ifdef __cplusplus
}
#endif

#endif
