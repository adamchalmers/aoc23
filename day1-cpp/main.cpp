#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int numFromDigits(const std::vector<int> *const digits) {
    int firstDigit = digits->at(0);
    auto secondDigit = digits->at(digits->size()-1);
    return firstDigit * 10 + secondDigit;
}

int parser1(const std::string line) {
    std::vector<int> digits = {};
    for (auto curr : line) { 
        if (curr >= '0' && curr <= '9') {
            digits.push_back(curr-'0');
        }
    } 
    
    return numFromDigits(&digits);
}

int parser2(const std::string *line) {
    std::vector<std::tuple<std::string, int>> numerals = {
        {"zero", 0},
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8},
        {"nine", 9},
        {"0", 0},
        {"1", 1},
        {"2", 2},
        {"3", 3},
        {"4", 4},
        {"5", 5},
        {"6", 6},
        {"7", 7},
        {"8", 8},
        {"9", 9},
    };

    std::vector<int> digits = {};
    // Use unsigned long because that's the type returned by length().
    for (unsigned long i = 0; i < line->length(); i++) { 
        auto slice = line->substr(i, line->size()-1);

        for (auto numeral : numerals) {
            auto spelling = std::get<0>(numeral);
            auto value = std::get<1>(numeral);
            if (slice.rfind(spelling, 0) == 0) {
                digits.push_back(value);
                break;
            }
        }
    } 
    
    return numFromDigits(&digits);
}

int main() {
    std::ifstream file("../day1/input.txt");
    std::string str; 
    auto sumQ1 = 0;
    auto sumQ2 = 0;
    while (std::getline(file, str))
    {
        sumQ1 += parser1(str);
        sumQ2 += parser2(&str);
    }
    std::cout << "Q1: " << sumQ1 << std::endl;
    std::cout << "Q2: " << sumQ2 << std::endl;
}