#include "common.h"
/* Branch-heavy: Collatz over many seeds; data-dependent branches. */
u32 run(void) {
    u32 total = 0;
    for (u32 seed = 1; seed < 300000; seed++) {
        u32 n = seed;
        while (n != 1) {
            if (n & 1) n = 3 * n + 1;
            else n >>= 1;
            total++;
        }
    }
    return total & 0xff;
}
