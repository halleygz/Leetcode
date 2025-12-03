class MinStack:
    #in every push and in every pop update the min value

    def __init__(self):
        self.stack = []
        self.minimum = []
        

    def push(self, x: int) -> None:
        self.stack.append(x)
        if not self.minimum or x <= self.minimum[-1]:
            self.minimum.append(x)

    def pop(self) -> None:
        if self.stack[-1] == self.minimum[-1]:
            self.minimum.pop()
        self.stack.pop()
        

    def top(self) -> int:
        return self.stack[-1]
        

    def getMin(self) -> int:
        if len(self.minimum) == 0:
            return 
        else:
            return self.minimum[-1]
            
        


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()