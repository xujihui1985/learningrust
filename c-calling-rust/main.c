#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>

#include "rust_lib.h"

int main(void) {
    uint64_t result = rust_lib_add(20, 22);
    printf("rust_lib_add(20, 22) = %" PRIu64 "\n", result);

    const char *name = "Sean";
    size_t name_len = rust_lib_c_string_len(name);
    printf("rust_lib_c_string_len(\"%s\") = %zu\n", name, name_len);

    char *greeting = rust_lib_greeting_new(name);
    if (greeting == NULL) {
        fputs("rust_lib_greeting_new failed\n", stderr);
        return 1;
    }
    printf("rust_lib_greeting_new(\"%s\") = %s\n", name, greeting);
    rust_lib_string_free(greeting);

    RustLibPoint point = {
        .x = 3.0,
        .y = 4.0,
    };
    double distance = rust_lib_point_distance_from_origin(point);
    printf("rust_lib_point_distance_from_origin({3.0, 4.0}) = %.1f\n", distance);

    int32_t values[] = {1, 2, 3, 4, 5};
    size_t values_len = sizeof(values) / sizeof(values[0]);
    int64_t sum = rust_lib_sum_i32(values, values_len);
    printf("rust_lib_sum_i32({1, 2, 3, 4, 5}) = %" PRId64 "\n", sum);

    RustLibI32Stats stats = {0};
    bool stats_ok = rust_lib_analyze_i32(values, values_len, &stats);
    if (!stats_ok) {
        fputs("rust_lib_analyze_i32 failed\n", stderr);
        return 1;
    }
    printf("rust_lib_analyze_i32({1, 2, 3, 4, 5}) = "
           "{ len: %zu, sum: %" PRId64 ", min: %" PRId32 ", max: %" PRId32 ", mean: %.1f }\n",
           stats.len,
           stats.sum,
           stats.min,
           stats.max,
           stats.mean);

    return 0;
}
