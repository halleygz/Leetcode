class RandomizedSet:

    def __init__(self):
        self.hashmap = defaultdict()
        self.arr = []

    def insert(self, val: int) -> bool:
        if val not in self.hashmap:
            self.hashmap[val] = len(self.hashmap)
            self.arr.append(val)    
            return True
        else:
            return False
        

    def remove(self, val: int) -> bool:
        if val in self.hashmap:
            idx = self.hashmap[val]
            lastVal = self.arr[-1]
            self.arr[idx] = lastVal
            self.arr.pop()
            self.hashmap[lastVal] = idx
            del self.hashmap[val]
            return True
        else:
            return False        

    def getRandom(self) -> int:
        return self.arr[random.randint(0, len(self.hashmap) - 1)]

        


# Your RandomizedSet object will be instantiated and called as such:
# obj = RandomizedSet()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)
# param_3 = obj.getRandom()