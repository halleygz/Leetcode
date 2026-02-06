class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        dups = {}
        dup = []
        for i in nums:
            if i in dups:
                dups[i] += 1
                dup.append(i)
                continue
            dups[i] = 0
        
        return[i for i in set(dup)]
                


        