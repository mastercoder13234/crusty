#include <iostream>

#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#endif

typedef int (*add_fn)(int, int);

int main()
{
#ifdef _WIN32
	HINSTANCE rust_lib = LoadLibraryA("rsa-lib/target/release/rsa_lib.dll");
	if (!rust_lib)
	{
		std::cerr << "Error loading rsa-lib/target/release/rsa_lib.dll" << std::endl;
		return 1;
	}
#else
	void *rust_lib = dlopen("./rsa-lib/target/release/librsa_lib.so", RTLD_NOW);
	if (!rust_lib)
	{
		std::cerr << "Error loading ./rsa-lib/target/release/librsa_lib.so: " << dlerror() << std::endl;
		return 1;
	}
#endif

	add_fn rust_add = (add_fn)
#ifdef _WIN32
		GetProcAddress(rust_lib, "add");
#else
		dlsym(rust_lib, "add");
#endif

	if (!rust_add)
	{
		std::cerr << "Error getting function pointer to 'add'" << std::endl;
#ifdef _WIN32
		FreeLibrary(rust_lib);
#else
		dlclose(rust_lib);
#endif
		return 1;
	}

	int num1 = 10;
	int num2 = 5;
	int result = rust_add(num1, num2);

	std::cout << "Result from Rust add function: " << num1 << " + " << num2 << " = " << result << std::endl;

#ifdef _WIN32
	FreeLibrary(rust_lib);
#else
	dlclose(rust_lib);
#endif

	return 0;
}