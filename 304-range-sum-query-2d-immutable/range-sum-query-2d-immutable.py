class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        R, C = len(matrix), len(matrix[0])

        self.prefArr = [[0] * (C + 1) for _ in range(R + 1)]

        for r in range(R):
            prefix = 0
            for c in range(C):
                prefix += matrix[r][c]
                above = self.prefArr[r][c + 1]
                self.prefArr[r + 1][c + 1] = prefix + above

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        r1, c1, r2, c2 = row1 + 1, col1 + 1, row2 + 1, col2 + 1

        total = self.prefArr[r2][c2]
        upper = self.prefArr[r1-1][c2]
        left = self.prefArr[r2][c1-1]

        area = total - upper - left + self.prefArr[r1-1][c1-1]
        return area


# Your NumMatrix object will be instantiated and called as such:
# obj = NumMatrix(matrix)
# param_1 = obj.sumRegion(row1,col1,row2,col2)
