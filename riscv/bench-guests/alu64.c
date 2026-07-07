#include "common.h"
/* Native 64-bit ALU workload: u64 xorshift. */
u32 run(void) {
    unsigned long long x = 0xdeadbeefcafebabeull;
    for (u32 i = 0; i < 50000000u; i++) {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
    }
    return (u32)(x & 0xff);
}
