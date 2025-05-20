#include <iostream>
#include "libload.h"
#include "bindings.h"
#include <cstdint>

int main()
{
	const char *lib_path =
#ifdef _WIN32
		"rsa.dll";
#else
		"librsa.so";
#endif

	lib_handle_t rust_lib = getlib(lib_path);
	if (!rust_lib)
	{
		return 1;
	}

	int num1 = 20;
	int num2 = 7;
	int result = add(num1, num2);
	std::cout << "Result from Rust add function: " << num1 << " + " << num2 << " = " << result << std::endl;

	std::cout << "Greatest common denominator of 15 and 93 is: " << gcd(15, 93) << std::endl;

	std::cout << "True or False: Is 65537 prime? " << is_prime(65537) << std::endl;

	std::cout << "Random Prime: " << gen_prime() << std::endl;

	std::cout << "Modpow(2, 3, 5): " << modpow(2, 3, 5) << std::endl;

	// New tests:
	uint32_t a = 3;
	uint32_t m = 11;
	std::cout << "Modular inverse of " << a << " mod " << m << " is: " << modinv(a, m) << std::endl;

	uint32_t x = 35;
	uint32_t y = 64;
	std::cout << "Are " << x << " and " << y << " coprime? " << (is_coprime(x, y) ? "Yes" : "No") << std::endl;

	hello_from_rust();
	test_func();

	freelib(rust_lib);
	return 0;
}
