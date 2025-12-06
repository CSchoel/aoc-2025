def calculate(op: str, numbers: list[int]) -> int:
    if op == "+":
        return sum(numbers)
    else:  # op == "*"
        result = 1
        for n in numbers:
            result *= n
        return result

def main(input: str) -> int:
    problems = []
    for line in input.splitlines():
        if problems == []:
            problems = [ [] for _ in range(len(line.split())) ]
        for e, number in enumerate(line.split()):
            problems[e].append(number)
    operations = [problem[-1] for problem in problems]
    numbers = [list(map(int, problem[:-1])) for problem in problems]
    results = [calculate(op, nums) for op, nums in zip(operations, numbers)]
    return sum(results)