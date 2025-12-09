class Solution:
    def reverse(self, x: int) -> int:
        MAX_SIGNED_DIV_10 = 214748364
        MIN_SIGNED_DIV_10 = -214748364

        MAX_SIGNED_LAST_DIGIT = 7
        MIN_SIGNED_LAST_DIGIT = -8

        rev = 0
        while x != 0:
            digit = int(math.fmod(x,10))
            x = int(x/10)
            if (rev > MAX_SIGNED_DIV_10) or ((rev == MAX_SIGNED_DIV_10) and (digit > MAX_SIGNED_LAST_DIGIT)):
                return 0
            if (rev < MIN_SIGNED_DIV_10) or ((rev == MIN_SIGNED_DIV_10) and (digit < MIN_SIGNED_LAST_DIGIT)):
                return 0
            rev = rev * 10 + digit
        return rev