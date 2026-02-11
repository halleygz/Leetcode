class Solution:
    def findCommonResponse(self, responses: List[List[str]]) -> str:
        combined = []
        for res in responses:
            for r in set(res):
                combined.append(r)
        
        count = Counter(combined)

        most_common = combined[0]

        for i in combined:
            if count[i] > count[most_common]:
                most_common = i
            if count[i] == count[most_common]:
                if i < most_common:
                    most_common = i
                    
        
        return most_common
