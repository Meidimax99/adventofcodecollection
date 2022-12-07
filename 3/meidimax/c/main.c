#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdlib.h>

#define ITEM_NUM 52

bool isUpperCase(char c)
{
    return !(c & 0x20) ? true : false;
}
char toUpperCase(char c)
{
    return c & ~0x20;
}

int itemTypeToPriority(char itemType)
{
    int priority = 0;
    if (isUpperCase(itemType))
    {
        priority += 26;
    }
    return priority + (toUpperCase(itemType) - 64);
}

void printArray(int arr[], int size)
{
    printf("{ ");
    for (int i = 0; i < size - 1; i++)
    {
        printf("%2d, ", arr[i]);
    }
    printf("%d }\n", arr[size - 1]);
}

typedef struct
{
    int *firstCompartment;
    int *secondCompartment;
} UnorderedRucksackStruct, *UnorderedRuckSack;

int *makeItemCompartment()
{
    return calloc((ITEM_NUM + 1), sizeof(int));
}
UnorderedRuckSack makeRucksack()
{
    UnorderedRuckSack rucksack = malloc(sizeof(UnorderedRucksackStruct));
    rucksack->firstCompartment = makeItemCompartment();
    rucksack->secondCompartment = makeItemCompartment();
    return rucksack;
}

void addItemToCompartment(int *compartment, char item)
{
    compartment[itemTypeToPriority(item)]++;
}

void addItem(UnorderedRuckSack rucksack, char item, int compartment)
{
    if (compartment == 1)
    {
        addItemToCompartment(rucksack->firstCompartment, item);
    }
    else
    {
        addItemToCompartment(rucksack->secondCompartment, item);
    }
}

void unpackStringIntoRucksack(UnorderedRuckSack rucksack, char *str)
{
    int str_len = strlen(str) - 1;
    for (int i = 0; i < str_len; i++)
    {
        addItem(rucksack, str[i], (i > (str_len / 2) - 1) + 1);
    }
}

void emptyRucksack(UnorderedRuckSack rucksack)
{
    for (int i = 0; i < ITEM_NUM + 1; i++)
    {
        rucksack->firstCompartment[i] = 0;
        rucksack->secondCompartment[i] = 0;
    }
}

int getCommonItemTypePriority(UnorderedRuckSack rucksack)
{
    for (int i = 1; i < ITEM_NUM + 1; i++)
    {
        if (rucksack->firstCompartment[i] > 0 && rucksack->secondCompartment[i] > 0)
        {
            return i;
        }
    }
    return 0;
}

void printRucksack(UnorderedRuckSack rucksack)
{
    int array[ITEM_NUM + 1];
    for (int i = 0; i < ITEM_NUM + 1; i++)
    {
        array[i] = i;
    }
    printArray(array, ITEM_NUM + 1);
    printArray(rucksack->firstCompartment, ITEM_NUM + 1);
    printArray(rucksack->secondCompartment, ITEM_NUM + 1);
}

// overmodelled approach
int main1(int argc, char *argv)
{
    char buffer[128];
    UnorderedRuckSack rucksack = makeRucksack();
    int prioritySum = 0;
    while (!feof(stdin))
    {
        fgets(buffer, 128, stdin);
        unpackStringIntoRucksack(rucksack, buffer);
        int priority = getCommonItemTypePriority(rucksack);
        printRucksack(rucksack);
        printf("%d\n", priority);
        emptyRucksack(rucksack);
        prioritySum += priority;
    }
    printf("The sum of priorities is %d\n", prioritySum);
    return 0;
}

int getCommonItemPriority(char *str)
{
    int stringlength = strlen(str) - 1; // remove /n
    for (int i = 0; i < stringlength / 2; i++)
    {
        for (int j = (stringlength / 2); j < stringlength; j++)
        {
            if (str[i] == str[j])
            {
                return itemTypeToPriority(str[i]);
            }
        }
    }
    return 0;
}
// naive approach
int main(int argc, char *argv)
{
    char buffer[128];
    int prioritySum = 0;
    while (!feof(stdin))
    {
        fgets(buffer, 128, stdin);
        prioritySum += getCommonItemPriority(buffer);
    }
    printf("The sum of priorities is %d\n", prioritySum);
    return 0;
}