#include <stdio.h>
#include <string.h>

void overflow(char *buf)
{
  char *s = "hello";
  strcpy(buf, s);
}

int main(void)
{
  char buf1[10], buf2[10], buf3[10];
  int z = 0x12345678;
  strcpy(buf1, "Hello");
  strcpy(buf2, "safe");
  strcpy(buf3, "world");
  puts(buf1);
  puts(buf2);
  puts(buf3);
  printf("z = 0x%08x\n", z);
  overflow(buf2);
  puts(buf1);
  puts(buf2);
  puts(buf3);
  printf("z = 0x%08x\n", z);
  return 0;
}