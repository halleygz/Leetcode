class Solution:
    def frequencySort(self, s: str) -> str:
        count = Counter(s)

        count_arr = []

        for i,j in count.items():
            count_arr.append([j,i])

        sorted_str = ""

        for s in sorted(count_arr, reverse=True):
            sorted_str += s[0]*s[1]

        return sorted_str
        
        