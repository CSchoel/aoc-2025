def main(input: str) -> int:
    matrix = [[0 if i == "." else 1 for i in row] for row in input.split("\n")]
    accessed = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            if matrix[i][j] == 1:
                adjacent = 0
                for k in [i-1, i, i+1]:
                    if 0 <= k < len(matrix):
                        for l in [j-1, j, j+1]:
                            if 0 <= l < len(matrix[0]):
                                if not (k == i and l == j):
                                    adjacent += matrix[k][l]
                if adjacent < 4:
                    accessed += 1
    return accessed