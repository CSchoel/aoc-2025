def parse_instruction(line: str) -> int:
    direction = line[0]
    distance = int(line[1:])
    return -distance if direction == "L" else distance

def main(input: str) -> int:
    instructions = [parse_instruction(line) for line in input.split("\n") if line != ""]

    dial = 50
    zeroes = 0
    for instruction in instructions:
        dial = (dial + instruction) % 100
        if dial == 0:
            zeroes += 1
    return zeroes
