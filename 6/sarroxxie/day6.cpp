#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>

const char* INPUT_PATH = "input.txt";

/*
 * The input file is just one single line of characters.
 */

/*
 * Works like the modulo operator in python.
 * Examples for different cases:
 * 
 *   positive_modulo( 13,  3)   =  1
 *   positive_modulo(-13,  3)  =  2
 *   positive_modulo( 13, -3)  = -2
 *   positive_modulo(-13, -3) = -1
 */
inline int positive_modulo(int i, int n) {
    return (i % n + n) % n;
}

/*
 * The goal is to find the first position in the input file where 4 unique
 * characters are appearing in a row.
 *
 * In one cycle, the next character is read from the input file and compared to
 * the contents of the buffer. If there is a duplicate of it already stored in
 * the buffer, the duplicateCounter gets set to the amount of cycles it will
 * take for the duplicate to leave the buffer. The new character gets placed
 * into the buffer at index 'counter % (UNIQUE - 1)' because this way, every new
 * character stays in the array for (UNIQUE - 1) cycles.
 * The task is complete if the duplicateCounter is zero.
 */
int task1() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // amount of unique characters that have to be in a sequence
    const int UNIQUE = 4;

    // stores the last (UNIQUE - 1) characters to check for duplicates
    char buffer[UNIQUE - 1] = {};
    int  counter            = 0;
    // if a duplicate is inserted into the buffer, this counter will get set to
    // the number of iterations that are needed for the duplicate to leave the buffer
    int duplicateCounter = UNIQUE - 1;

    char tmp;
    // label:
    while(input.get(tmp)) {
        for(int i = 0; i < UNIQUE - 1; i++) {
            if(tmp == buffer[i]) {
                duplicateCounter =
                    std::max(positive_modulo(i - counter, UNIQUE - 1) + 1, duplicateCounter);
            }
        }
        // rotating access of the array, this way it is possible to insert a new
        // character which always replaces the one that has stayed in this array the longest
        buffer[counter % (UNIQUE - 1)] = tmp;
        if(duplicateCounter <= 0) {
            return counter + 1;
        }
        // adjusting counters
        counter++;
        duplicateCounter--;
    }

    return 0;
}

/*
 * The goal is to find the first position in the input file where 4 unique
 * characters are appearing in a row.
 *
 * In one cycle, the next character is read from the input file and compared to
 * the contents of the buffer. If there is a duplicate of it already stored in
 * the buffer, the duplicateCounter gets set to the amount of cycles it will
 * take for the duplicate to leave the buffer. The new character gets placed
 * into the buffer at index 'counter % (UNIQUE - 1)' because this way, every new
 * character stays in the array for (UNIQUE - 1) cycles.
 * The task is complete if the duplicateCounter is zero.
 */
int task2() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // amount of unique characters that have to be in a sequence
    const int UNIQUE = 14;

    // stores the last (UNIQUE - 1) characters to check for duplicates
    char buffer[UNIQUE - 1] = {};
    int  counter            = 0;
    // if a duplicate is inserted into the buffer, this counter will get set to
    // the number of iterations that are needed for the duplicate to leave the buffer
    int duplicateCounter = UNIQUE - 1;

    char tmp;
    // label:
    while(input.get(tmp)) {
        for(int i = 0; i < UNIQUE - 1; i++) {
            if(tmp == buffer[i]) {
                duplicateCounter =
                    std::max(positive_modulo(i - counter, UNIQUE - 1) + 1, duplicateCounter);
            }
        }
        // rotating access of the array, this way it is possible to insert a new
        // character which always replaces the one that has stayed in this array the longest
        buffer[counter % (UNIQUE - 1)] = tmp;
        if(duplicateCounter <= 0) {
            return counter + 1;
        }
        // adjusting counters
        counter++;
        duplicateCounter--;
    }

    return 0;
}

/*
 * Runs the task, measures their runtime and prints the results.
 */
int main() {
    std::string                           result;
    std::chrono::system_clock::time_point start, end;
    std::chrono::duration<double>         duration;
    double                                millis;

    // task1
    start    = std::chrono::system_clock::now();
    result   = std::to_string(task1());
    end      = std::chrono::system_clock::now();
    duration = end - start;
    millis   = duration.count() * 1000;

    std::cout << "TASK 1: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";


    // task2
    start    = std::chrono::system_clock::now();
    result   = std::to_string(task2());
    end      = std::chrono::system_clock::now();
    duration = end - start;
    millis   = duration.count() * 1000;

    std::cout << "TASK 2: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";

    return 0;
}