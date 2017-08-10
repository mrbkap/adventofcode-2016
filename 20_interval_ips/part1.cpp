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
main()
{
    set<Interval> ivals;

    string line;
    while (true) {
        Interval next;
        if (!(cin >> next)) {
            break;
        }

        ivals.insert(move(next));
    }

    uint32_t min = 0;
    for (auto& ival : ivals) {
        if (ival.contains(min)) {
            min = ival.end + 1;
        }
    }

    cout << min << '\n';
}
