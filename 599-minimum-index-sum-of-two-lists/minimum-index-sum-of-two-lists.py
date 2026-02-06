class Solution:
    def findRestaurant(self, list1: List[str], list2: List[str]) -> List[str]:
        ans  = []
        min_idx = len(list1) + len(list2)

        for i in range(len(list1)):
            if list1[i] in list2:
                j = list2.index(list1[i])
                if i + j < min_idx:
                    min_idx = i + j
                    ans = [list1[i]]
                elif i+j == min_idx:
                    ans.append(list1[i])
        return ans