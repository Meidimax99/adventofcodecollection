#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv) {
    int sum = 0;
    int max = 0;
    int size = 16;
    int lastindex = 0;
    int *sums = malloc(sizeof(int)*size);
    while(!feof(stdin)) {
        int num = 0;
        scanf("%d",&num);
        sum += num;
        char c = getchar();
        c = getchar();
        if(c == '\n' || feof(stdin)) {
            lastindex++;
            sums[lastindex] = sum;
            sum = 0;

            if(lastindex == size) {
                size *= 2;
                sums = realloc(sums, sizeof(int)*size);
            }
        } else {
            ungetc(c, stdin);
        }
    }
    printf("Max: %d\n", sums[lastindex]);
    free(sums);
    return 0;
}
