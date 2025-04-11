class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        dicts={}
        for i in range(len(nums)):
            compl = target - nums[i]
            if compl in dicts:
                return([i,dicts[compl]])
            dicts[nums[i]] = i
        return([])