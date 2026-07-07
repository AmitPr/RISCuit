#include "common.h"
/* Call/return-heavy: naive recursive fibonacci. */
__attribute__((noinline)) static u32 fib(u32 n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}
u32 run(void) { return fib(32) & 0xff; }
