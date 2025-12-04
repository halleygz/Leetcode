class MyCircularQueue:

    def __init__(self, k: int):
        self.queue = [0] * k
        self.remains = k
        self.k = k
        

    def enQueue(self, value: int) -> bool:
        if self.remains > 0:
            self.queue[self.k-self.remains] = value
            self.remains-=1
            return True
        else:
            return False


    def deQueue(self) -> bool:
        if self.remains < self.k:
            self.queue[0] = 0
            self.queue.pop(0)
            self.queue.append(0)
            self.remains += 1
            return True
        else:
            return False

    def Front(self) -> int:
        if self.remains < self.k:
            return self.queue[0]
        else: 
            return -1
        

    def Rear(self) -> int:
        if self.remains == 0:
            return self.queue[-1]
        elif self.remains < self.k:
            return self.queue[self.k-self.remains - 1]
        else:
            return -1
        

    def isEmpty(self) -> bool:
        if self.remains == self.k:
            return True
        else:
            return False
        

    def isFull(self) -> bool:
        if self.remains == 0:
            return True
        else:
            return False
        


# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue(k)
# param_1 = obj.enQueue(value)
# param_2 = obj.deQueue()
# param_3 = obj.Front()
# param_4 = obj.Rear()
# param_5 = obj.isEmpty()
# param_6 = obj.isFull()