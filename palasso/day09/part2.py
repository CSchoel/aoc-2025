Point = tuple[int, int]
Line = tuple[Point, Point]


def area(x: Point, y: Point) -> int:
    return (abs(x[0] - y[0]) + 1) * (abs(x[1] - y[1]) + 1)


def line_intersects_rectangle(rectangle, line):
    xmin, ymin, xmax, ymax = rectangle
    p1, p2 = line
    x1, y1 = p1
    x2, y2 = p2

    # Vertical line
    if x1 == x2:
        x = x1
        if not (xmin < x < xmax):
            return False
        y_low = max(min(y1, y2), ymin)
        y_high = min(max(y1, y2), ymax)
        return y_low < y_high

    # Horizontal line
    if y1 == y2:
        y = y1
        if not (ymin < y < ymax):
            return False
        x_low = max(min(x1, x2), xmin)
        x_high = min(max(x1, x2), xmax)
        return x_low < x_high


def main(input: str) -> int:
    red_tiles: list[Point] = [tuple(map(int, line.split(","))) for line in input.splitlines()]  # type:ignore
    # connect red tiles into lines
    red_lines: list[Line] = []
    for red1, red2 in zip(red_tiles, red_tiles[1:]+[red_tiles[0]]):
        red_lines.append((red1, red2))
    max_area = 0
    for i, red1 in enumerate(red_tiles):
        for j, red2 in enumerate(red_tiles):
            square = area(red1, red2)
            if square > max_area:
                (x0, y0), (x1, y1) = red1, red2
                xmin, xmax = sorted([x0, x1])
                ymin, ymax = sorted([y0, y1])
                rectangle = (xmin, ymin, xmax, ymax)
                if any(line_intersects_rectangle(rectangle, red_line) for red_line in red_lines):
                    continue
                max_area = square
    return max_area