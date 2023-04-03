#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include <sys/mman.h>
#include <errno.h>

const int cons = 29;
const char *instructions = "\x48\x31\xFF\xB8\x3C\x00\x00\x00\x0F\x05";
const size_t instructions_len = 10;

int main(int argc, char **argv) {
    printf("main at %p\n", &main);
    printf("instruction at %p\n", instructions);

    size_t region = (size_t)instructions & ~0xFFF;
    printf("region is %p\n", (void*)region);

    int ret = mprotect(
        (void*)region,
        0x1000, // 4k
        PROT_READ | PROT_EXEC
    );
    if (ret != 0) {
        printf("mprotect failed: %d\n", errno);
        return 1;
    }
    void (*f)(void) = (void*)instructions;
    printf("jumping...\n");
    f();
    printf("after jump\n");
    int numb = 13;
    printf("numb is at %p\n", &numb);
    numb = 127;
    printf("wrote to numb, now = %d\n", numb);

    void *aptr = malloc(sizeof(int));
    printf("cons is at %p\n", &cons);
    printf("aptr is at %p\n", aptr);
    free(aptr);
}