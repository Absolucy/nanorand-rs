#include "nanorand.h"
#include <iostream>

int main() {
	nanorand::ChaCha rng = nanorand::new_chacha20();
	std::cout << nanorand::chacha_next_u16(&rng) << std::endl;
}
