class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        count = Counter(nums)

        for i in nums:
            if count[i] > 1:
                return True

        return False 
        