class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        # count = 
        return [i[0] for i in Counter(nums).most_common(k)]