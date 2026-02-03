class Solution:
    def isPalindrome(self, x: int) -> bool:
        normal = x
        reversed = 0
        while normal > 0:
            reversed = (reversed * 10) + (normal % 10)
            normal//=10
        return x == reversed