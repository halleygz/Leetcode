class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        # find first zero(valid position) i
        # left = 0
        # for right in range(len(nums)):
        #     if nums[right] != 0:
        #         nums[right], nums[left] = nums[left], nums[right]
        #         left += 1
        # return nums
        left = 0
        right = 0
        while right <  len(nums):
            if nums[right] != 0:
                nums[right], nums[left] = nums[left], nums[right]
                left+=1
            right+=1
    
            
