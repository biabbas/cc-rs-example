#include<iostream>
extern "C"{
void foo_factorial(int32_t a){
    if(a > 0){
        printf("(%d)*\v",a);
        foo_factorial(a-1);
    }
    else
        printf("\n");
}
}