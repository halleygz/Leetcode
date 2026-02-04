class Solution:
    def isCovered(self, ranges: List[List[int]], left: int, right: int) -> bool:
        for i in range(left,  right+1):
            covered = False
            for l, r in ranges:
                if l <= i <= r:
                    covered = True
                    break
            if not covered:
                return False
        return True