class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        # find first zero(valid position) i
        for i in range(len(nums)):
            if nums[i] == 0:
                for j in range(i, len(nums)):
                    if nums[j] != 0:
                        nums[i] = nums[j]
                        nums[j] = 0
                        break
                    if j == len(nums) - 1:
                        return
        # fix that index and look for a non zero number j
        # swap num with the zero
        #
            
