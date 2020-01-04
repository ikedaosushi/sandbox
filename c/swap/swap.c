#include <stdio.h>

void swapString(char** a, char** b) {
    char *tmp = *a;
    *a = *b;
    *b = tmp;
}

int main(void)
{
    char* a = "Hellooooo";
    char* b = "World";
    swapString(&a, &b);
    printf("%s %s\n", a, b);

    char* c = "Hello";
    char* d = "Worldoooo";
    swapString(&c, &d);
    printf("%s %s\n", c, d);
    return 0;
}
