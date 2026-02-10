class Solution:
    def romanToInt(self, s: str) -> int:
        romanMap = {
            "I":1,
            "V": 5,
            "X":10,
            "L":50,
            "C": 100,
            "D":500,
            "M":1000,
        }
        number = 0
        for i in range(len(s) - 1):
            if romanMap[s[i]] < romanMap[s[i+1]]:
                number -= romanMap[s[i]]
            else:
                number += romanMap[s[i]]
        return number + romanMap[s[-1]]
                
