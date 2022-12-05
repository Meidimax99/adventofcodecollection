#include <stdio.h>
#include <stdlib.h>

#define TOP_ELF_SUM_COUNT 3

void printArray(int arr[], int size) {
	printf("{ ");
	for(int i = 0; i < size -1; i++) {
		printf("%d, ",arr[i]);
	}
	printf("%d }\n", arr[size-1]);
}


int main(int argc, char *argv) {
    int sum = 0;            //Sum for one sub-list
    int size = 16;          //Current size of array
    int elemcount = 0;      //Current number of elements in the array
    int writeindex = 0;     //Current index to write the sum into
    int *sums = malloc(sizeof(int)*size);   //Array holding all sums
    
    //Iterate while there is input
    while(!feof(stdin)) {
        //Adding new number to sum
        int num = 0;
        scanf("%d",&num);
        sum += num;

        //Adjusting writeindex based on new sum
        while(writeindex < elemcount && sum > sums[writeindex]) {
            writeindex++;
        }

        char c = getchar();
        c = getchar();
        
        if(c == '\n' || feof(stdin)) { //End of one sub-list -> new sum to add to array 
            elemcount++;
            //Inserting and sorting
            for(int i = elemcount; i > writeindex ; i--) {
                sums[i] = sums[i-1];
            }
            sums[writeindex] = sum;
            //printArray(sums, elemcount);
            //Cleanup for next iteration
            sum = 0;
            writeindex = 0;

            if(elemcount == size) {
                size *= 2;
                sums = realloc(sums, sizeof(int)*size);
            }

        } else { //Not yet end of sub-list
            ungetc(c, stdin);
        }
    }
    //printArray(sums, elemcount);
    int sumofsums = 0;
    for(int i = 0; i < TOP_ELF_SUM_COUNT; i++) {
        sumofsums += sums[elemcount-1-i];
    }
    printf("The top %d elves carry %d Calories!\n", TOP_ELF_SUM_COUNT,sumofsums);
    free(sums);
    return 0;
}
