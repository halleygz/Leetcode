class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        n = len(matrix)
        m = len(matrix[0])

        self.prefix = [[0]*m for _ in range(n)]

        for i in range(n):
            for j in range(m):
                
                self.prefix[i][j] = matrix[i][j]

                if i > 0:
                    self.prefix[i][j] += self.prefix[i-1][j]
                if j > 0:
                    self.prefix[i][j] += self.prefix[i][j-1]
                if i > 0 and j > 0:
                    self.prefix[i][j] -= self.prefix[i-1][j-1]

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        range = self.prefix[row2][col2]


        if row1 > 0:
            range -= self.prefix[row1-1][col2]
        if col1 > 0:
            range -= self.prefix[row2][col1-1]
        if row1 > 0 and col1 > 0:
            range += self.prefix[row1-1][col1-1]

        return range

        


# Your NumMatrix object will be instantiated and called as such:
# obj = NumMatrix(matrix)
# param_1 = obj.sumRegion(row1,col1,row2,col2)