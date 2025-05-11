#include "bindings.h"
#include <iostream>

#ifdef _WIN32
HINSTANCE load_rust_library()
{
	HINSTANCE lib = LoadLibraryA("librsa.dll");
	if (!lib)
	{
		std::cerr << "Error loading librsa.dll" << std::endl;
	}
	return lib;
}

FARPROC get_rust_add_function(HINSTANCE library_handle)
{
	FARPROC func = GetProcAddress(library_handle, "add");
	if (!func)
	{
		std::cerr << "Error getting function pointer to 'add'" << std::endl;
	}
	return func;
}

void unload_rust_library(HINSTANCE library_handle)
{
	FreeLibrary(library_handle);
}

#else // Linux/macOS

void *load_rust_library()
{
	void *lib = dlopen("./librsa.so", RTLD_NOW);
	if (!lib)
	{
		std::cerr << "Error loading ./librsa.so: " << dlerror() << std::endl;
	}
	return lib;
}

void *get_rust_add_function(void *library_handle)
{
	void *func = dlsym(library_handle, "add");
	if (!func)
	{
		std::cerr << "Error getting function pointer to 'add'" << std::endl;
	}
	return func;
}

void unload_rust_library(void *library_handle)
{
	dlclose(library_handle);
}

#endif