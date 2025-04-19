class Solution {
public:
    int passThePillow(int n, int time) {
        int count = 1;

        bool flag = false;
        for (int i = 1; i <= time; ++i) {
            if (!flag) {
                count++;

                if (count == n) {
                    flag = true;
                }
            } else if (flag) {
                count--;

                if (count == 1) {
                    flag = false;
                }
            }
        }
        return count;
    }
};