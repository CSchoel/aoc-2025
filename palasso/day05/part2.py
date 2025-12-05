def main(input: str) -> int:
    fresh_ranges, _ = input.split("\n\n")
    fresh_ranges = [tuple(map(int, line.split("-"))) for line in fresh_ranges.splitlines()]
    fresh_ranges.sort()
    merged_ranges = []
    current_start, current_end = fresh_ranges[0]
    for start, end in fresh_ranges[1:]:
        if start <= current_end + 1:
            current_end = max(current_end, end)
        else:
            merged_ranges.append((current_start, current_end))
            current_start, current_end = start, end
    merged_ranges.append((current_start, current_end))
    total_fresh = sum(end - start + 1 for start, end in merged_ranges)
    return total_fresh