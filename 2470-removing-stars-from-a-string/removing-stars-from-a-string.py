class Solution:
    def removeStars(self, s: str) -> str:
        stack = []
        for c in s:
            if c == "*":
                if stack:
                    stack.pop()
                continue
            stack.append(c)
        
        return "".join(stack)





