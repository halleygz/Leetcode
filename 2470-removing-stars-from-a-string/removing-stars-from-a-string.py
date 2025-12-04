class Solution:
    def removeStars(self, s: str) -> str:
        i = 0
        while i < len(s):
            if s[i] == "*":
                s = s[:i] + s[i+1:]
                i-=1
                s = s[:i] + s[i+1:]
                i-=1
            i+=1
        return s



