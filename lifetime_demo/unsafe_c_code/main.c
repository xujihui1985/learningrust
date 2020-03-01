#include <stdio.h>
#include <stdlib.h>

int* some_function();

int main() 
{
	int a = 5;
	int *pA = &a;
	printf("number of a is %d \n", *pA);

	int* b = some_function();
	printf("number of b is %d \n", *b);
	return 0;
}

int* some_function()
{
	int myInt = 7777;
	return &myInt;
}