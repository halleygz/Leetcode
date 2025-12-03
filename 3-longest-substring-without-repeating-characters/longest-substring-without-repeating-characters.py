class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        store = set()
        maxSize = 0
        j = 0
        for i in range(len(s)):
            while s[i] in store:
                store.remove(s[j])
                j+=1
            store.add(s[i])
            maxSize = max(maxSize, i - j + 1)
        return maxSize


