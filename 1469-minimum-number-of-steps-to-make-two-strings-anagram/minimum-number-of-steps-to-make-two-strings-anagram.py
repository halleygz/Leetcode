class Solution:
    def minSteps(self, s: str, t: str) -> int:
        diff = Counter(s) - Counter(t)
        steps = 0
        if len(diff) != 0:
            for vals in diff.values():
                steps += vals

        return steps