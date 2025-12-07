from collections import Counter

def main(input: str) -> int:
    beams = Counter()
    for line in input.splitlines():
        if not beams:
            beams[line.find("S")] += 1
        else:
            new_beams = Counter()
            for beam in beams:
                if line[beam] == ".":
                    new_beams[beam] += beams[beam]
                else:
                    if beam > 0 and line[beam - 1] == ".":
                        new_beams[beam - 1] += beams[beam]
                    if beam < len(line) - 1 and line[beam + 1] == ".":
                        new_beams[beam + 1] += beams[beam]
            beams = new_beams
    return sum(beams.values())