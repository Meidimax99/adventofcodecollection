#include <stdio.h>
#include <string.h>


int main(int argc, char *argv) {
    char buffer[128];
    while(!feof(stdin)) {
        fgets(buffer,128,stdin);
        int stringlength = strlen(buffer)-1; //Remove \n
    }
}