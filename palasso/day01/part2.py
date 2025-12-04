def parse_instruction(line: str) -> int:
    direction = line[0]
    distance = int(line[1:])
    return -distance if direction == "L" else distance

def main(input: str) -> int:
    instructions = [parse_instruction(line) for line in input.split("\n") if line != ""]

    dial = 50
    zeroes = 0
    for instruction in instructions:
        modifier = 0 if instruction > 0 else -1
        zeroes += abs((dial + modifier + instruction) // 100)
        if dial == 0 and instruction < 0:
            zeroes -= 1
        dial = (dial + instruction) % 100
    return zeroes
