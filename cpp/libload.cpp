#include "libload.h"
#include <iostream>
#include <string>

lib_handle_t getlib(const char *library_path)
{
#ifdef _WIN32
	HINSTANCE lib = LoadLibraryA(library_path);
	if (!lib)
	{
		std::cerr << "Error loading library: " << library_path << std::endl;
	}
	return lib;
#else
	void *lib = dlopen(library_path, RTLD_NOW);
	if (!lib)
	{
		std::cerr << "Error loading library: " << library_path << " - " << dlerror() << std::endl;
	}
	return lib;
#endif
}

void freelib(lib_handle_t library_handle)
{
#ifdef _WIN32
	FreeLibrary(library_handle);
#else
	dlclose(library_handle);
#endif
}