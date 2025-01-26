#include <stdlib.h>
// #include <sys/syscall.h>
// #include <sys/types.h>

// void _start() {
//   void* ptr = malloc(100);
//   syscall(SYS_exit, (int)ptr);
// }

int main() {
  void* ptr = malloc(100);
  return (int)ptr;
}