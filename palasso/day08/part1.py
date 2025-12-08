MAX_CONNECTIONS = 10
Point = tuple[int, int, int]


def euclidean_distance(x: Point, y: Point) -> float:
    return sum([(i-j)**2 for i, j in zip(x, y)])**(1/2)


def main(input: str) -> int:
    box: dict[int, Point] = {e: tuple(map(int, line.split(","))) for e, line in enumerate(input.splitlines())}  # type:ignore
    distance_matrix = [[euclidean_distance(box[x], box[y]) for y in box.keys()] for x in box.keys()]
    pairs_with_distance = {distance: (x, y) for x, vector in enumerate(distance_matrix) for y, distance in enumerate(vector) if x < y}
    sorted_distances = sorted(pairs_with_distance.keys())[:MAX_CONNECTIONS]
    circuits = [set(pairs_with_distance[distance]) for distance in sorted_distances]
    merged = True
    while merged:
        merged = False
        for i in range(len(circuits)):
            for j in range(i + 1, len(circuits)):
                if circuits[i].intersection(circuits[j]):
                    circuits[i].update(circuits[j])
                    del circuits[j]
                    merged = True
                    break
            if merged:
                break
    boxes_in_circuits = sorted(map(len, circuits))[-3:]
    result = 1
    for i in boxes_in_circuits:
        result *= i
    return result