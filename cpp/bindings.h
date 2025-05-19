#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t add(int32_t left, int32_t right);

void hello_from_rust();

void test_func();

uint64_t gcd(uint64_t left, uint64_t right);

bool is_prime(uint64_t n);

uint64_t gen_prime();

}  // extern "C"
