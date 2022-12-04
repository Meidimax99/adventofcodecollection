#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>
#include <list>
#include <vector>

const char* INPUT_PATH = "input.txt";

int task1() {
    int           max = 0;
    int           sum = 0;
    std::ifstream input(INPUT_PATH);
    std::string   line;
    // iterate over file -> add all numbers up until an empty line is reached ->
    // set new maximum if the new sum is greater
    while(std::getline(input, line)) {
        // compare current sum with maximum
        if(line == "") {
            max = std::max(max, sum);
            sum = 0;
            continue;
        }
        // convert line to integer and add to sum
        sum += std::stoi(line);
    }
    return max;
}


int task2() {
    // this list will always contain the 3 maxima and will remain sorted throughout
    std::list<int> max = {0, 0, 0};
    int            sum = 0;
    std::ifstream  input(INPUT_PATH);
    std::string    line;
    // iterate over file -> add all numbers up until an empty line is reached ->
    // insert into maxima list if it is a new maximum
    while(std::getline(input, line)) {
        // compare current sum with maxima
        if(line == "") {
            std::list<int>::iterator it;
            for(it = max.begin(); it != max.end(); ++it) {
                // compare if current sum is greater than a maximum and if yes,
                // insert it in that location and pop the smallest of the old maxima out
                if(sum > *it) {
                    max.insert(it, sum);
                    max.pop_back();
                    sum = 0;
                    break;
                }
            }
            sum = 0;
            continue;
        }
        // convert line to integer and add to sum
        sum += std::stoi(line);
    }
    sum = 0;
    for(const int m : max) {
        sum += m;
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
