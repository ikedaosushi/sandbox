#include <stdio.h>
#include <stdlib.h>

void swap(char **a, char **b) {
  char *tmp = *a;
  *a = *b;
  *b = tmp;
}

int main(int argc, char *argv[])
{
  int n, i, j;
  char group[100];
  FILE *fp;

  fp = fopen("sample.txt", "r");

  fgets(group, 100, fp);
  n = atoi(group);
  char name[n][256], score[n][100];
  for(i = 0; i < n; i++) {
    fscanf(fp, "%s %s ", name[i], score[i]);
  }

  for(i = 0; i < n-1; i++) {
    for (j = n-1; j > i; j--) {
      if (atoi(score[j-1]) > atoi(score[j])) {
        swap((char**) &name[j-1], (char**) &name[j]);
        swap((char**) &score[j-1], (char**) &score[j]);
      }
    }
  }

  for (i = 0; i < n; i++) {
    printf("%s %s\n", name[i], score[i]);
  }

  fclose(fp);
  return 0;
}