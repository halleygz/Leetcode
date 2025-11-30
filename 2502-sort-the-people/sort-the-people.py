class Solution:
    # using bubble sort for practice
    def sortPeople(self, names: List[str], heights: List[int]) -> List[str]:
        for j in range(len(heights)):
            for i in range(0, len(heights) - j - 1):
                if heights[i] < heights[i + 1]:
                    names[i], names[i+1] = names[i+1], names[i]
                    heights[i], heights[i+1] = heights[i+1], heights[i]

        return names