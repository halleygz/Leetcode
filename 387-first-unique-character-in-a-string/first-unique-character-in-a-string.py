class Solution:
    def firstUniqChar(self, s: str) -> int:
        count = Counter(s)
        for c in range(len(s)):
            if count[s[c]] == 1:
                return c
        return -1