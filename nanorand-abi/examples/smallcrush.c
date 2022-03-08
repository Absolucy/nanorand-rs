#include "nanorand.h"
#include <TestU01.h>
#include <iostream>

static BufferedChaCha20 *chacha_rng;
static BufferedWyRand *wyrand_rng;
static BufferedPcg64 *pcg_rng;

unsigned int gen_chacha(void)
{
    unsigned int ret;
    chacha20_buffered_fill(chacha_rng, &ret, sizeof(ret));
    return ret;
}

unsigned int gen_wyrand(void)
{
    unsigned int ret;
    wyrand_buffered_fill(wyrand_rng, &ret, sizeof(ret));
    return ret;
}

unsigned int gen_pcg(void)
{
    unsigned int ret;
    pcg64_buffered_fill(pcg_rng, &ret, sizeof(ret));
    return ret;
}

int main()
{
    chacha_rng = chacha20_buffered(new_chacha20());
    wyrand_rng = wyrand_buffered(new_wyrand());
    pcg_rng = pcg64_buffered(new_pcg64());

    unif01_Gen *chacha_testu01 = unif01_CreateExternGenBits("nanorand chacha20", gen_chacha);
    bbattery_SmallCrush(chacha_testu01);
    unif01_DeleteExternGenBits(chacha_testu01);

    unif01_Gen *wyrand_testu01 = unif01_CreateExternGenBits("nanorand wyrand", gen_wyrand);
    bbattery_SmallCrush(wyrand_testu01);
    unif01_DeleteExternGenBits(wyrand_testu01);

    unif01_Gen *pcg_testu01 = unif01_CreateExternGenBits("nanorand pcg64", gen_pcg);
    bbattery_SmallCrush(pcg_testu01);
    unif01_DeleteExternGenBits(pcg_testu01);
}
