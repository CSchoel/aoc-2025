Point = tuple[int, int]

def area(x: Point, y: Point) -> int:
    return (abs(x[0] - y[0]) + 1) * (abs(x[1] - y[1]) + 1)

def main(input: str) -> int:
    red_tiles: list[Point] = [tuple(map(int, line.split(","))) for line in input.splitlines()]  # type:ignore
    max_area = 0
    for red1 in red_tiles:
        for red2 in red_tiles:
            square = area(red1, red2)
            if square > max_area:
                max_area = square
    return max_area