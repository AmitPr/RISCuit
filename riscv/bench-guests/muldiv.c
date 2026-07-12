#include "common.h"
/* M-extension heavy: multiply/divide/remainder chains. */
u32 run(void) {
    u32 acc = 0x12345678;
    for (u32 i = 1; i < 20000000u; i++) {
        acc = acc * 2654435761u + i;
        acc ^= acc / ((i & 0xffff) + 3);
        acc += acc % 97;
    }
    return acc & 0xff;
}
