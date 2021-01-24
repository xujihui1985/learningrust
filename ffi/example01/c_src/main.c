#include "header.h"
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int main(void) {
  int myarr[10] = {1,2,3,4,5,6,7, 8, 9, 10};
  int total = sum_of_even(myarr, 10);
  printf("total is %d\n", total);
  return 0;
}
