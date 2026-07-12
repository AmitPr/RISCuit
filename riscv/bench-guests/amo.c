#include "common.h"
/* AMO rd==rs2 hazard: amoadd.w a5,a5,(a0) must store old+rs2, and rd must
   receive the old value. Exit 42 if correct. */
static volatile u32 w;
u32 run(void) {
    w = 10;
    register unsigned long r5 asm("a5") = 3;
    register volatile u32 *p asm("a0") = &w;
    __asm__ volatile("amoadd.w a5, a5, (a0)" : "+r"(r5) : "r"(p) : "memory");
    if (r5 != 10 || w != 13) return 7;   /* buggy emulators: w == 20 */
    w = 0;
    r5 = 1;
    __asm__ volatile("amoswap.w a5, a5, (a0)" : "+r"(r5) : "r"(p) : "memory");
    if (r5 != 0 || w != 1) return 8;     /* buggy: w == 0 (store lost) */
    return 42;
}
