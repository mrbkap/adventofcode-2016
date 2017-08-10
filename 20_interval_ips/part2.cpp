#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <set>
using namespace std;

struct Interval
{
    uint32_t start;
    uint32_t end;

    bool contains(uint32_t p) const {
        return start <= p && p <= end;
    }

    bool operator<(const Interval& rhs) const {
        if (start < rhs.start) {
            return true;
        }
        if (start > rhs.start) {
            return false;
        }
        return end < rhs.end;
    }
};

istream& operator >>(istream& in, Interval& ival) {
    char c;
    in >> ival.start;
    in >> c; // -
    in >> ival.end;
    return in;
}

int
main(int argc, char* argv[])
{
    set<Interval> ivals;

    ifstream input(argc > 1 ? argv[1] : "input");
    assert(input);
    string line;
    while (true) {
        Interval next;
        if (!(input >> next)) {
            break;
        }

        ivals.insert(move(next));
    }

    uint64_t nips = 4294967296;
    Interval last{ 0, 0 };
    for (auto& ival : ivals) {
        if (!last.contains(ival.start)) {
            nips -= (last.end - last.start) + 1;
            last.start = ival.start;
            last.end = ival.end;
        } else if (ival.end > last.end) {
            last.end = ival.end;
        }
    }

    nips -= (last.end - last.start) + 1;
    cout << nips << '\n';
}
