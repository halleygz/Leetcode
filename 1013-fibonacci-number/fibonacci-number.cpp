#include <cmath>
class Solution {
public:
    int fib(int n) {
        double sqrt_5 = std::sqrt(5);
        double phi = (1 + sqrt_5) / 2;
        double psi = (1 - sqrt_5) /2;

        double term1 = std::pow(phi, n) / sqrt_5;
        double term2 = std::pow(psi, n) / sqrt_5;

        return static_cast<int>(round(term1 - term2));
    }
};