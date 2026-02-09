class Solution:
    def restoreString(self, s: str, indices: List[int]) -> str:
        ordered = [0]*len(s)

        for i in range(len(s)):
            ordered[indices[i]] = s[i]

        return "".join(ordered)
