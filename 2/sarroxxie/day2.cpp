#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>

const char* INPUT_PATH = "input.txt";

/*
 * The gained points are as follows:
 *
 * if you select Rock     -> 1 point
 *               Paper    -> 2 points
 *               Scissors -> 3 points
 *
 * if you lose -> 0 points
 *        draw -> 3 points
 *        win  -> 6 points
 */

int task1() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // clang-format off
    /*
     * Lookup table for all the possible combinations.
     * 'A' and 'X' are rock
     * 'B' and 'Y' are paper
     * 'C' and 'Z' are scissors
     *
     * AX AY AZ
     * BX BY BZ
     * CX CY CZ
     */
    uint8_t lut[3][3] = {4, 8, 3, 
                         1, 5, 9, 
                         7, 2, 6};
    // clang-format on

    int sum = 0;
    // One line always consists of 4 chars in the input file: 'A'|'B'|'C' then
    // 'space' then 'X'|'Y'|'Z' then 'paragraph -> can use fixed size buffer
    char* line = new char[4];
    while(input.getline(line, 4)) {
        sum += lut[line[0] - 'A'][line[2] - 'X'];
    }
    return sum;
}

int task2() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // clang-format off
    /*
     * Lookup table for all the possible combinations.
     * 'A' is rock
     * 'B' is paper
     * 'C' is scissors
     *
     * 'X' means that I need to lose
     * 'Y' means that I need to draw
     * 'Z' means that I need to win
     *
     * AX AY AZ
     * BX BY BZ
     * CX CY CZ
     */
    uint8_t lut[3][3] = {3, 4, 8, 
                         1, 5, 9,
                         2, 6, 7};
    // clang-format on

    int sum = 0;
    // One line always consists of 4 chars in the input file: 'A'|'B'|'C' then
    // 'space' then 'X'|'Y'|'Z' then 'paragraph -> can use fixed size buffer
    char* line = new char[4];
    while(input.getline(line, 4)) {
        sum += lut[line[0] - 'A'][line[2] - 'X'];
    }
    return sum;
}

/*
 * Runs the task, measures their runtime and prints the results.
 */
int main() {
    std::string                           result;
    std::chrono::system_clock::time_point start, end;
    std::chrono::duration<double>         duration;
    // TODO: find out if the data type is correct -> and find out if it is printable
    double millis;

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