#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>

const char* INPUT_PATH = "input.txt";

/*
 * The input file contains 2 ranges in each line. The syntax for one line is like
 * in this example: "9-14,10-24".
 */

/*
 * The goal is to find out for each line if one of the ranges is contained in
 * the other one. The result is the sum of on how many lines this occurred.
 */
int task1() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int sum = 0;
    // min1 and max1 define the 1st range, min2 and max2 define the 2nd range
    int min1, max1, min2, max2;
    while(input >> min1) {
        // skip '-'
        input.get();
        input >> max1;
        // skip ','
        input.get();
        input >> min2;
        // skip '-'
        input.get();
        input >> max2;

        // --1st range contains 2nd range || 2nd range contains 1st range--
        if((min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2))
            sum++;
    }
    return sum;
}

/*
 * The goal is to find out for each line if the two ranges overlap. The result
 * is the sum of on how many lines this occurred.
 */
int task2() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int sum = 0;
    // min1 and max1 define the 1st range, min2 and max2 define the 2nd range
    int min1, max1, min2, max2;
    while(input >> min1) {
        // skip '-'
        input.get();
        input >> max1;
        // skip ','
        input.get();
        input >> min2;
        // skip '-'
        input.get();
        input >> max2;

        // inverted case of where the ranges don't overlap
        if(!(max1 < min2 || max2 < min1))
            sum++;
    }
    return sum;
}

/*
* Runs the task, measures their runtime and prints the results.
*/
int main()
{
    std::string result;
    std::chrono::system_clock::time_point start, end;
    std::chrono::duration<double> duration;
    double millis;

    // task1
    start = std::chrono::system_clock::now();
    result = std::to_string(task1());
    end = std::chrono::system_clock::now();
    duration = end - start;
    millis = duration.count() * 1000;

    std::cout << "TASK 1: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";


    // task2
    start = std::chrono::system_clock::now();
    result = std::to_string(task2());
    end = std::chrono::system_clock::now();
    duration = end - start;
    millis = duration.count() * 1000;

    std::cout << "TASK 2: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";

    return 0;
}