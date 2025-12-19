START, END = "you", "out"

def main(input: str) -> int:
    graph = {line.split(":")[0]: set(line.split(":")[1].strip().split()) for line in input.splitlines()}
    num_paths = 1
    def count_paths(current_node: str):
        nonlocal num_paths
        if current_node != END:
            num_paths -= 1
            for neighbor in graph[current_node]:
                num_paths += 1
                count_paths(neighbor)
    count_paths(START)
    return num_paths