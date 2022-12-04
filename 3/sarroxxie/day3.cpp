#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>
#include <algorithm>

const char* INPUT_PATH = "input.txt";

/*
 * Every character has a priority associated with it, 'a' to 'z' being 1 to 26
 * and 'A' to 'Z' being 27 to 52. The first half of a line are the contents of
 * the first rucksack, the second half are the contents of the second rucksack.
 */

const int CHARACTER_COUNT  = 52;
const int UPPERCASE_OFFSET = 26;

// value = priority - 1 (needed for array indexing)
int charToValue(char character) {
    if(character >= 'a')
        return (character - 'a');
    return (character - 'A' + UPPERCASE_OFFSET);
}

/*
 * In every line of characters, there is always EXACTLY one that is contained in
 * the first half and in the second half. The goal is to find this character for
 * each line and add the priority of this character to the overall sum.
 */
int task1() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int sum = 0;
    int strLength;
    // LUT for all the different characters from the first half of the line
    bool values[CHARACTER_COUNT];
    // set all values of the LUT to false
    memset(values, false, sizeof(values));

    std::string line;
    while(std::getline(input, line)) {
        strLength = line.size();
        // fill the LUT at the position of the first half of the line
        for(int i = 0; i < strLength / 2; i++) {
            values[charToValue(line[i])] = true;
        }
        // check for each character of the second half of the line if it is already registered in the LUT
        for(int i = strLength / 2; i < strLength; i++) {
            if(values[charToValue(line[i])]) {
                // priorities start at 1, so it has to be added to the value (which starts at 0)
                sum += charToValue(line[i]) + 1;
                // set all values of the LUT back to false
                memset(values, false, sizeof(values));
                break;
            }
        }
    }
    return sum;
}

/*
 * Every 3 lines share EXACTLY one character. The goal is to find this character
 * for each line and add the priority of this character to the overall sum.
 */
int task2() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int sum = 0;
    // clang-format off
    /* LUT for all the different characters from the first line and second line (those are appended)
    * Layout:
    * 'a', ... 'z', 'A', ... 'Z', 'a', ... 'z', 'A', ... 'Z'
    *         first line                  second line
    * |<----CHARACTER_COUNT---->| |<----CHARACTER_COUNT---->|
    */
    // clang-format on
    bool values[CHARACTER_COUNT * 2];
    // set all values of the LUTs to false
    memset(values, false, sizeof(values));

    std::string line;
    int         strLength;
    while(std::getline(input, line)) {
        // register each character of the first line in the LUT
        strLength = line.size();
        for(int i = 0; i < strLength; i++) {
            values[charToValue(line[i])] = true;
        }
        // register each character of the second line in the LUT
        std::getline(input, line);
        strLength = line.size();
        for(int i = 0; i < strLength; i++) {
            values[charToValue(line[i]) + CHARACTER_COUNT] = true;
        }
        // for each character of the third line check if it is registered in
        // both locations in the LUT (which means that this character is
        // contained in both the first and second line)
        std::getline(input, line);
        strLength = line.size();
        for(int i = 0; i < strLength; i++) {
            if(values[charToValue(line[i])] && values[charToValue(line[i]) + CHARACTER_COUNT]) {
                // priorities start at 1, so it has to be added to the value (which starts at 0)
                sum += charToValue(line[i]) + 1;
                // set all values of the LUT back to false
                memset(values, false, sizeof(values));
                break;
            }
        }
    }
    return sum;
}

/*
 * returns the intersecting set of two strings (that don't have to be sorted)
 */
std::string stringIntersect(std::string first, std::string second) {
    std::sort(first.begin(), first.end());
    std::sort(second.begin(), second.end());

    std::string result;
    std::set_intersection(first.begin(), first.end(), second.begin(),
                          second.end(), std::back_inserter(result));

    return result;
}

/*
 * Alternative implementation to 'task1()' done with built in functions for
 * better readability and adaptability. -> this takes TRIPLE the time to compute
 * compared to 'task1()'!
 */
int task1_alternative() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int         sum = 0;
    std::string line;
    std::string intersection;
    while(std::getline(input, line)) {
        // split strings in half and get intersection
        intersection = stringIntersect(line.substr(0, line.size() / 2),
                                       line.substr(line.size() / 2));
        sum += charToValue(intersection[0]) + 1;
    }
    return sum;
}

/*
 * Alternative implementation to 'task2()' done with built in functions for
 * better readability and adaptiability. -> takes around TWICE the time to
 * compute compared to 'task2()'!
 */
int task2_alternative() {
    // open input file
    std::ifstream input(INPUT_PATH);

    int         sum = 0;
    std::string line;
    std::string intersection;
    while(std::getline(input, line)) {
        // intersection of the first 2 lines
        std::getline(input, intersection);
        intersection = stringIntersect(line, intersection);
        // intersection of the intersection of the first 2 lines with the third line
        std::getline(input, line);
        intersection = stringIntersect(line, intersection);
        sum += charToValue(intersection[0]) + 1;
        intersection = "";
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