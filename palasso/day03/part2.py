def main(document: str) -> int:
    total = 0
    for bank in document.split("\n"):
        options = [int(x) for x in bank]
        selections = [0]*12
        cutoff = -1
        for i in range(12):
            for e, joltage in enumerate(options):
                if e < cutoff+1 or e >= len(options)-(11-i):
                    continue
                if joltage > selections[i]:
                    selections[i] = joltage
                    cutoff = e
        joltage = "".join(str(x) for x in selections)
        total += int(joltage)
    return total
