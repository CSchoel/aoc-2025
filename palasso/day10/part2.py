from itertools import product

def gaussian_elimination(A, b):
    m, n = len(A), len(A[0])
    # Augmented matrix
    M = [A[i][:] + [b[i]] for i in range(m)]

    row = 0
    pivots = []

    for col in range(n):
        # Find pivot
        pivot = None
        for r in range(row, m):
            if M[r][col] != 0:
                pivot = r
                break
        if pivot is None:
            continue

        # Swap rows
        M[row], M[pivot] = M[pivot], M[row]
        pivots.append(col)

        # Normalize pivot row
        pivot_val = M[row][col]
        for j in range(col, n + 1):
            M[row][j] /= pivot_val

        # Eliminate below
        for r in range(row + 1, m):
            factor = M[r][col]
            for j in range(col, n + 1):
                M[r][j] -= factor * M[row][j]

        row += 1
        if row == m:
            break

    return M, pivots

def back_substitute(M, pivots, free_variables, free_values):
    n = len(M[0]) - 1
    x = [0] * n

    for j, free_value in zip(free_variables, free_values):
        x[j] = free_value

    for i in reversed(range(len(pivots))):
        col = pivots[i]
        s = M[i][-1]
        for j in range(col + 1, n):
            s -= M[i][j] * x[j]
        x[col] = s

    return x

def solve(A, b) -> int:
    M, pivots = gaussian_elimination(A, b)
    n = len(A[0])
    free_variables = [j for j in range(n) if j not in pivots]

    result = max(b) * n + 1
    if not free_variables:
        x = back_substitute(M, pivots, [], [])
        result = min(result, sum(x))

    for values in product(range(max(b) + 1), repeat=len(free_variables)):
        x = back_substitute(M, pivots, free_variables, values)
        if all(v >= 0 for v in x):
            result = min(result, sum(x))

    return int(result)

def main(input: str) -> int:
    button_presses = 0
    for machine in input.splitlines():
        lights, *buttons, voltage = machine.split(" ")
        lights = [False if light == "." else True for light in lights.strip("[]")]
        buttons = [tuple(map(int, button.strip("()").split(","))) for button in buttons]
        voltage = list(map(int, voltage.strip("{}").split(",")))
        magic = [[1 if counter in button else 0 for button in buttons] for counter in range(len(voltage))]
        presses = solve(A=magic, b=voltage)
        button_presses += presses
    return button_presses
