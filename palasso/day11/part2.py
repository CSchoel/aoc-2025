START, MIDDLE1, MIDDLE2, END = "svr", "dac", "fft", "out"

cache = {}

def main(input: str) -> int:
    graph = {line.split(":")[0]: set(line.split(":")[1].strip().split()) for line in input.splitlines()}
    def counts(start: str, end: str) -> int:
        if (start, end) not in cache:
            cache[(start, end)] = sum([counts(neighbor, end) if end not in graph[neighbor] else 1 for neighbor in graph[start] if neighbor != END])
        return cache[(start, end)]
    full_paths = [counts(START, MIDDLE1) * counts(MIDDLE1, MIDDLE2) * counts(MIDDLE2, END),
                  counts(START, MIDDLE2) * counts(MIDDLE2, MIDDLE1) * counts(MIDDLE1, END)]
    return sum(full_paths)