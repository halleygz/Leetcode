#include <cmath>
class Solution {
public:
    int fib(int n) {
        double s = std::sqrt(5), a = (1 + s) / 2, b = (1 - s) / 2;
        return round((std::pow(a, n) - std::pow(b, n)) / s);
    }
};