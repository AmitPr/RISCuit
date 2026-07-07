#include "common.h"
/* Load/store-heavy: strided read-modify-write over a 1 MiB static buffer. */
static u32 buf[262144];
u32 run(void) {
    u32 acc = 0;
    for (u32 pass = 0; pass < 200; pass++) {
        for (u32 i = 0; i < 262144; i++) {
            buf[i] = buf[i] * 3 + pass;
            acc += buf[i];
        }
    }
    return acc & 0xff;
}
