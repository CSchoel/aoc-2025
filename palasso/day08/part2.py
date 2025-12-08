Point = tuple[int, int, int]


def euclidean_distance(x: Point, y: Point) -> float:
    return sum([(i-j)**2 for i, j in zip(x, y)])**(1/2)


def main(input: str) -> int:
    box: dict[int, Point] = {e: tuple(map(int, line.split(","))) for e, line in enumerate(input.splitlines())}  # type:ignore
    distance_matrix = [[euclidean_distance(box[x], box[y]) for y in box.keys()] for x in box.keys()]
    pairs_with_distance = {distance: (x, y) for x, vector in enumerate(distance_matrix) for y, distance in enumerate(vector) if x < y}
    sorted_distances = sorted(pairs_with_distance.keys())
    circuits = []
    for distance in sorted_distances:
        circuits.append(set(pairs_with_distance[distance]))
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
        if len(circuits) == 1:
            if all(x in circuits[0] for x in box.keys()):
                break
    x, y = pairs_with_distance[distance]
    return box[x][0]*box[y][0]