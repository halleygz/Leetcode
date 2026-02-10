class Solution:
    def separateDigits(self, nums: List[int]) -> List[int]: 
        return [int(digit) for n in nums for digit in str(n)]

        