def calculate(op: str, numbers: list[int]) -> int:
    if op == "+":
        return sum(numbers)
    else:  # op == "*"
        result = 1
        for n in numbers:
            result *= n
        return result

def main(input: str) -> int:
    rows = input.splitlines()
    seq = ["".join(row[i] for row in rows) for i in range(len(rows[0]))]
    total = 0
    start = True
    for s in seq:
        if start:
            numbers = []
            op = s[-1]
            numbers.append(int(s[:-1].strip()))
            start = False
        else:
            if s.strip() == "":
                total += calculate(op, numbers)
                start = True
            else:
                numbers.append(int(s.strip()))
    total += calculate(op, numbers)
    return total