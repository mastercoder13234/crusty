#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t add(int32_t left, int32_t right);

void hello_from_rust();

void test_func();

uint32_t gcd(uint32_t left, uint32_t right);

bool is_prime(uint32_t n);

uint32_t gen_prime();

uint32_t modpow(uint32_t base, uint32_t exponent, uint32_t modulus);

uint32_t modinv(uint32_t a, uint32_t m);

bool is_coprime(uint32_t a, uint32_t b);

}  // extern "C"
