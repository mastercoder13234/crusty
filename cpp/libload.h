#ifndef BINDINGS_H
#define BINDINGS_H

#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#include <string> // For std::string
#endif

// Define a generic library handle type
#ifdef _WIN32
typedef HINSTANCE lib_handle_t;
#else
typedef void *lib_handle_t;
#endif

// Generic function to load a dynamic library
lib_handle_t getlib(const char *library_path);

// Generic function to unload a dynamic library
void freelib(lib_handle_t library_handle);

#endif // BINDINGS_H