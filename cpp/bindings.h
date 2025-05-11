#ifndef BINDINGS_H
#define BINDINGS_H

#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#endif

// Define the function pointer type for the Rust function
typedef int (*add_fn)(int, int);

// Function to load the Rust library
void *load_rust_library();

// Function to get the pointer to the 'add' function
add_fn get_rust_add_function(void *library_handle);

// Function to unload the Rust library
void unload_rust_library(void *library_handle);

#endif // RUST_BINDINGS_H