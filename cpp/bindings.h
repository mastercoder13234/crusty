#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t add(int32_t left, int32_t right);

void hello_from_rust();

void test_func();

int32_t gcd(int32_t left, int32_t right);

}  // extern "C"
