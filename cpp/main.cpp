#include <iostream>
#include "bindings.h"

int main()
{
	void *rust_lib = load_rust_library();
	if (!rust_lib)
	{
		return 1;
	}

	add_fn rust_add = get_rust_add_function(rust_lib);
	if (!rust_add)
	{
		unload_rust_library(rust_lib);
		return 1;
	}

	int num1 = 20;
	int num2 = 7;
	int result = rust_add(num1, num2);

	std::cout << "Result from Rust add function: " << num1 << " + " << num2 << " = " << result << std::endl;

	unload_rust_library(rust_lib);

	return 0;
}