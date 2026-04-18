#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

#include "rust_lib.h"

int main(void) {
    uint64_t result = rust_lib_add(20, 22);
    printf("rust_lib_add(20, 22) = %" PRIu64 "\n", result);
    return 0;
}
