#include "common.h"
/* ALU-heavy: xorshift32 PRNG, ~600M dynamic instructions. */
u32 run(void) {
    u32 x = 0xdeadbeef;
    for (u32 i = 0; i < 100000000u; i++) {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
    }
    return x & 0xff;
}
