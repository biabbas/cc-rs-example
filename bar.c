#include<stdio.h>
#include<stdint.h>
int32_t bar_function(int32_t x) { printf("Bar now calls %d\n",x); return x++; }