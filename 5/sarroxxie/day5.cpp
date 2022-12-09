#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <ctime>
#include <vector>
#include <stack>

const char* INPUT_PATH = "input.txt";

/*
 * The input file contains a list of 9 stacks (start configuration) and a list
 * of operations that should be executed on these stacks.
 *
 * Start configuration syntax (example, only similar to input):
 *
 *     [G]
 *     [B] [C]
 * [A] [D] [E] [F]
 *  1   2   3   4
 *
 * Operations syntax:
 *
 * "move 3 from 2 to 4"
 * -> move the first 3 items (from the top) from the 2nd stack to the 4th
 */

// these constants contain information on the input file structure
const int NUMBER_OF_BINS = 9;
const int INITIAL_OFFSET = 1;
const int STRIDE         = 4;

/*
 * Skips a specified amount of characters from the input file.
 */
void skipChars(std::ifstream* input, unsigned int skips) {
    for(int i = 0; i < skips; i++) {
        input->get();
    }
}

/*
 * Moves a certain amount of elements from one stack onto the other. This
 * happens one element at a time.
 */
template <typename T>
void move(int amount, std::stack<T>* origin, std::stack<T>* destination) {
    for(int i = 0; i < amount; i++) {
        destination->push(origin->top());
        origin->pop();
    }
}

/*
 * From the start configuration, execute all operations. The output should be
 * the characters that are on top of each stack. The "move" operator moves
 * elements one at a time.
 */
std::string task1() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // initiate start configuration
    std::vector<std::string> startConfig;
    std::string              line;
    while(std::getline(input, line)) {
        if(line == "")
            break;
        startConfig.push_back(line);
    }

    // fill the stacks with the start configuration
    std::stack<char> stacks[NUMBER_OF_BINS];
    char             tmp;
    for(int i = startConfig.size() - 2; i >= 0; i--) {
        for(int j = 0; j < NUMBER_OF_BINS; j++) {
            // need to extract the relevant information from the syntax of the input file
            tmp = startConfig[i][j * STRIDE + INITIAL_OFFSET];
            if(tmp != ' ')
                stacks[j].push(tmp);
        }
    }
    int amount, origin, destination;
    while(input.good()) {
        // skip "move"
        skipChars(&input, 4);
        input >> amount;
        // skip "from"
        skipChars(&input, 5);
        input >> origin;
        // skip "to"
        skipChars(&input, 3);
        input >> destination;
        // skip 'paragraph'
        skipChars(&input, 2);

        // the input file starts counting at 1....
        move(amount, &stacks[origin - 1], &stacks[destination - 1]);
    }

    std::string result = "";
    for(int i = 0; i < NUMBER_OF_BINS; i++) {
        result += stacks[i].top();
    }

    return result;
}

/*
 * Moves a certain amount of elements from one stack onto the other. The order
 * of the elements gets retained.
 */
template <typename T>
void move2(int amount, std::stack<T>* origin, std::stack<T>* destination) {
    char* buffer = new char[amount];
    for(int i = 0; i < amount; i++) {
        buffer[i] = origin->top();
        origin->pop();
    }
    for(int i = amount - 1; i >= 0; i--) {
        destination->push(buffer[i]);
    }
    delete[] buffer;
}

/*
 * From the start configuration, execute all operations. The output should be
 * the characters that are on top of each stack. The "move" operator moves
 * elements all at once -> the order is retained.
 */
std::string task2() {
    // open input file
    std::ifstream input(INPUT_PATH);

    // initiate start configuration
    std::vector<std::string> startConfig;
    std::string              line;
    while(std::getline(input, line)) {
        if(line == "")
            break;
        startConfig.push_back(line);
    }

    // fill the stacks with the start configuration
    std::stack<char> stacks[NUMBER_OF_BINS];
    char             tmp;
    for(int i = startConfig.size() - 2; i >= 0; i--) {
        for(int j = 0; j < NUMBER_OF_BINS; j++) {
            // need to extract the relevant information from the syntax of the input file
            tmp = startConfig[i][j * STRIDE + INITIAL_OFFSET];
            if(tmp != ' ')
                stacks[j].push(tmp);
        }
    }
    int amount, origin, destination;
    while(input.good()) {
        // skip "move"
        skipChars(&input, 4);
        input >> amount;
        // skip "from"
        skipChars(&input, 5);
        input >> origin;
        // skip "to"
        skipChars(&input, 3);
        input >> destination;
        // skip 'paragraph'
        skipChars(&input, 2);

        // the input file starts counting at 1....
        move2(amount, &stacks[origin - 1], &stacks[destination - 1]);
    }

    std::string result = "";
    for(int i = 0; i < NUMBER_OF_BINS; i++) {
        result += stacks[i].top();
    }

    return result;
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
    result   = task1();
    end      = std::chrono::system_clock::now();
    duration = end - start;
    millis   = duration.count() * 1000;

    std::cout << "TASK 1: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";


    // task2
    start    = std::chrono::system_clock::now();
    result   = task2();
    end      = std::chrono::system_clock::now();
    duration = end - start;
    millis   = duration.count() * 1000;

    std::cout << "TASK 2: " << result << "\n";
    std::cout << "finished in " << millis << "ms\n";

    return 0;
}