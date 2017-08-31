// ConsoleApplication1.cpp : Defines the entry point for the console application.
//

#include <algorithm>
#include <string>
#include <regex>
#include <fstream>
#include <iostream>
#include <variant>
#include <vector>
using namespace std;

const bool PART1 = true;

struct SwapPos {
    int first;
    int second;
};

struct SwapLet {
    char first;
    char second;
};

struct RotSteps {
    bool left;
    int steps;
};

struct RotLet {
    char letter;
};

struct ReversePos {
    int first;
    int second;
};

struct MovePos {
    int first;
    int second;
};

typedef variant<SwapPos, SwapLet, RotSteps, RotLet, ReversePos, MovePos> instr;

vector<instr>
parse()
{
    regex swappos{ R"(swap position (\d+) with position (\d+))" },
        swaplet{ R"(swap letter (.) with letter (.))" },
        rotsteps{ R"(rotate (left|right) (\d+) steps?)" },
        rotlet{ R"(rotate based on position of letter (.))" },
        reversepos{ R"(reverse positions (\d+) through (\d+))" },
        movepos{ R"(move position (\d+) to position (\d+))" };
    vector<instr> program;

    string pass = "abcdefgh";
    ifstream input("input.txt");
    if (!input) {
        std::cout << "No file\n";
        return program;
    }

    string line;
    while (getline(input, line)) {
        smatch match;
        if (regex_match(line, match, swappos)) {
            int first = std::stoi(match.str(1));
            int second = std::stoi(match.str(2));
            program.push_back(SwapPos{ first, second });
        }
        else if (regex_match(line, match, swaplet)) {
            char first = match.str(1)[0];
            char second = match.str(2)[0];
            program.push_back(SwapLet{ first, second });
        }
        else if (regex_match(line, match, rotsteps)) {
            bool left = match.str(1) == "left";
            int steps = stoi(match.str(2)) % pass.size();
            program.push_back(RotSteps{ left, steps });
        }
        else if (regex_match(line, match, rotlet)) {
            char letter = match.str(1)[0];
            program.push_back(RotLet{ letter });
        }
        else if (regex_match(line, match, reversepos)) {
            int first = std::stoi(match.str(1));
            int second = std::stoi(match.str(2));
            program.push_back(ReversePos{ first, second });
        }
        else if (regex_match(line, match, movepos)) {
            int first = std::stoi(match.str(1));
            int second = std::stoi(match.str(2));
            program.push_back(MovePos{ first, second });
        }
    }

    return program;
}

int
main()
{
    string pass = "fbgdceah";
    ifstream input("../../input.txt");
    if (!input) {
        cout << "No file\n";
        return 0;
    }

    cout << pass << '\n';
    auto program = parse();
    for (auto i = program.rbegin(), e = program.rend(); i != e; ++i) {
        if (auto* swappos = get_if<SwapPos>(&*i)) {
            // swapping is the same!
            swap(pass[swappos->first], pass[swappos->second]);
        }
        else if (auto* swaplet = get_if<SwapLet>(&*i)) {
            // swapping is the same!
            swap(pass[pass.find(swaplet->first)], pass[pass.find(swaplet->second)]);
        }
        else if (auto* rotsteps = get_if<RotSteps>(&*i)) {
            bool left = !rotsteps->left; // we're reversing it!
            auto first = left ? pass.begin() + rotsteps->steps : pass.end() - rotsteps->steps;
            rotate(pass.begin(), first, pass.end());
        }
        else if (auto* rotlet = get_if<RotLet>(&*i)) {
            int idx = pass.find(rotlet->letter);
            int rot = (idx / 2 + (idx % 2 || !idx ? 1 : 5));
            rotate(pass.begin(), pass.begin() + rot, pass.end());
        }
        else if (auto* reversepos = get_if<ReversePos>(&*i)) {
            reverse(pass.begin() + reversepos->first, pass.begin() + reversepos->second + 1);
        }
        else if (auto* movepos = get_if<MovePos>(&*i)) {
            char c = pass[movepos->second];
            pass.erase(movepos->second, 1);
            pass.insert(pass.begin() + movepos->first, c);
        }

        cout << pass << '\n';
    }
    cout << "Final answer: " << pass << '\n';
    return 0;
}

