#include "common.h"
/* Syscall dispatch overhead: 1M getpid() calls (stubbed to return 1). */
u32 run(void) {
    u32 acc = 0;
    for (u32 i = 0; i < 1000000u; i++) acc += getpid_();
    return acc & 0xff;
}
