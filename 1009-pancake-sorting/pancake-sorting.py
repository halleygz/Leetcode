class Solution:
    def pancakeSort(self, arr: List[int]) -> List[int]:
        res = []
        for i in range(len(arr),1,-1):
            max_idx = arr.index(max(arr[:i]))

            if max_idx == i-1:
                continue
            if max_idx != 0:
                res.append(max_idx+1)
                arr[:max_idx+1] = reversed(arr[:max_idx+1])
            res.append(i)
            arr[:i] = reversed(arr[:i])
        return res


