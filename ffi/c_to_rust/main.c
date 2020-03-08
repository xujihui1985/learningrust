#include "my_library.h"
#include <stdio.h>

int main() 
{
	NativeVec* v = new_vec();
	printf("length of vec is %d\n", v->vec_len);
	for (int i = 0; i < v->vec_len; i++) 
	{
		printf("vec value is is %d\n", *(v->vec + i));
		*(v->vec + i) = 123;
	}

	printf("addr %p\n", v->vec);
	mutate_vec(v);
	return 0;
}
