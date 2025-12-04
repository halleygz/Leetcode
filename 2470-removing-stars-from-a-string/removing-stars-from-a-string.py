class Solution:
    def removeStars(self, s: str) -> str:
        stack = []
        outie = ""
        for c in s:
            if c == "*":
                if stack:
                    stack.pop()
                continue
            stack.append(c)

        for j in range(len(stack)):
            outie+=stack[j]
        
        return outie





