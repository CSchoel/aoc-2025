def main(input: str) -> int:
    fresh_ranges, available = input.split("\n\n")
    available = list(map(int, available.splitlines()))
    fresh_ranges = [tuple(map(int, line.split("-"))) for line in fresh_ranges.splitlines()]
    num_available = 0
    for i in available:
        for start, end in fresh_ranges:
            if start <= i <= end:
                num_available += 1
                break
    return num_available