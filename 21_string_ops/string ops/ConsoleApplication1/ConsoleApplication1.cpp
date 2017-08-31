// ConsoleApplication1.cpp : Defines the entry point for the console application.
//

#include "stdafx.h"
#include <algorithm>
#include <string>
#include <regex>
#include <fstream>
#include <iostream>
using namespace std;

const bool PART1 = true;

int
main()
{
	regex swappos{ R"(swap position (\d+) with position (\d+))" },
		swaplet{ R"(swap letter (.) with letter (.))" },
		rotsteps{ R"(rotate (left|right) (\d+) steps?)" },
		rotlet{ R"(rotate based on position of letter (.))" },
		reversepos{ R"(reverse positions (\d+) through (\d+))" },
		movepos{ R"(move position (\d+) to position (\d+))" };

	string pass = "abcdefgh";
	ifstream input("../../input.txt");
	if (!input) {
		cout << "No file\n";
		return 0;
	}

	string line;
	while (getline(input, line)) {
		smatch match;
		if (regex_match(line, match, swappos)) {
			int first = std::stoi(match.str(1));
			int second = std::stoi(match.str(2));
			std::swap(pass[first], pass[second]);
		}
		else if (regex_match(line, match, swaplet)) {
			char first = match.str(1)[0];
			char second = match.str(2)[0];

			std::swap(pass[pass.find(first)], pass[pass.find(second)]);
		}
		else if (regex_match(line, match, rotsteps)) {
			bool left = match.str(1) == "left";
			int steps = stoi(match.str(2)) % pass.size();
			auto first = left ? pass.begin() + steps : pass.end() - steps;
			rotate(pass.begin(), first, pass.end());
		}
		else if (regex_match(line, match, rotlet)) {
			char letter = match.str(1)[0];
			int idx = pass.find(letter);
			// nrotates = 1 + idx + 4-calc
			int rotates = 1 + idx + (idx >= 4 ? 1 : 0);
			rotates %= pass.size();
			rotate(pass.begin(), pass.end() - rotates, pass.end());
		}
		else if (regex_match(line, match, reversepos)) {
			int first = std::stoi(match.str(1));
			int second = std::stoi(match.str(2));
			reverse(pass.begin() + first, pass.begin() + second + 1);
		}
		else if (regex_match(line, match, movepos)) {
			int first = std::stoi(match.str(1));
			int second = std::stoi(match.str(2));
			char c = pass[first];
			pass.erase(first, 1);
			pass.insert(pass.begin() + second, c);
		}

		cout << pass << '\n';
	}

	cout << "Final answer: " << pass << '\n';
    return 0;
}

