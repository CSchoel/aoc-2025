def main(input: str) -> int:
    splits = 0
    beams = set()
    for line in input.splitlines():
        if not beams:
            beams.add(line.find("S"))
        else:
            new_beams = set()
            split = False
            for beam in beams:
                if line[beam] == ".":
                    new_beams.add(beam)
                else:
                    if beam > 0 and line[beam - 1] == ".":
                        new_beams.add(beam - 1)
                        split = True
                    if beam < len(line) - 1 and line[beam + 1] == ".":
                        new_beams.add(beam + 1)
                        split = True
                if split:
                    splits += 1
                    split = False
            beams = new_beams
    return splits