def main(input: str) -> int:
    lines = input.splitlines()
    presents = {}
    regions = []
    for i in range(len(lines)):
        if lines[i] != "":
            if lines[i][-1] == ":":
                presents[lines[i][:-1]] = [line for line in lines[i + 1 : i + 4]]
                continue
            elif ":" in lines[i]:
                shape, filling =lines[i].split(":")
                shape = shape.split("x")
                shape = tuple(map(int, shape))
                filling = filling.strip().split(" ")
                filling = {e: int(x) for e, x in enumerate(filling) if x != "0"}
                regions.append((shape, filling))
    total = 0
    for region in regions:
        shape, filling = region
        area = shape[0] * shape[1]
        max_filling = sum(filling.values())*9
        if area >= max_filling:
            total += 1
    return total