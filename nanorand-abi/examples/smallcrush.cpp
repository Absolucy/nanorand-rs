#include "nanorand.h"
#include <iostream>
extern "C"
{
#include <TestU01.h>
}

static nanorand::ChaCha chacha_rng;
static nanorand::WyRand wyrand_rng;
static nanorand::Pcg64 pcg_rng;

unsigned int gen_chacha(void)
{
	return nanorand::chacha20_next_u32(&chacha_rng);
}

unsigned int gen_wyrand(void)
{
	return nanorand::wyrand_next_u32(&wyrand_rng);
}

unsigned int gen_pcg(void)
{
	return nanorand::pcg64_next_u32(&pcg_rng);
}

int main()
{
	chacha_rng = nanorand::new_chacha20();
	wyrand_rng = nanorand::new_wyrand();
	pcg_rng = nanorand::new_pcg64();

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
