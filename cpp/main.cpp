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

	std::cout << "Greatest common demominator of 15 and 93 is: " << gcd(15, 93) << std::endl;

	std::cout << "True or False: Is 65537 prime? " << is_prime(65537) << std::endl;

	std::cout << "Random Prime: " << gen_prime() << std::endl;

	hello_from_rust();

	test_func();

	freelib(rust_lib);

	return 0;
}