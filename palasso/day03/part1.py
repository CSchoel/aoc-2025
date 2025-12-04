def main(input: str) -> int:
    total = 0
    for bank in input.split("\n"):
        options = [int(x) for x in bank]
        first = 0
        max_pos = 0
        for e, joltage in enumerate(options[:-1]):
            if joltage > first:
                first = joltage
                max_pos = e
        second = 0
        for e, joltage in enumerate(options[max_pos+1:]):
            if joltage > second:
                second = joltage
        joltage = str(first) + str(second)
        total += int(joltage)
    return total
