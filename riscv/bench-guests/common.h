#pragma once
typedef unsigned int u32;
typedef unsigned long long u64;

static inline void exit_(u32 code) {
    register u32 a0 __asm__("a0") = code;
    register u32 a7 __asm__("a7") = 93; /* exit */
    __asm__ volatile("ecall" : : "r"(a0), "r"(a7));
    __builtin_unreachable();
}

static inline u32 getpid_(void) {
    register u32 a0 __asm__("a0");
    register u32 a7 __asm__("a7") = 172; /* getpid */
    __asm__ volatile("ecall" : "=r"(a0) : "r"(a7));
    return a0;
}

u32 run(void);

__attribute__((naked, section(".text.start"))) void _start(void) {
    __asm__ volatile("call run\n"
                     "li a7, 93\n"
                     "ecall\n");
}
