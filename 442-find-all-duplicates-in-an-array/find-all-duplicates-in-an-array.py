class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        dups = []
        for i in range(len(nums)):
            num = abs(nums[i])
            if nums[num-1]>0:
                nums[num-1] *= -1
            else:
                dups.append(num)
        return dups
                


        