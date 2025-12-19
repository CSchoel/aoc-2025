from itertools import combinations_with_replacement

def main(input: str) -> int:
    button_presses = 0
    for machine in input.splitlines():
        lights, *buttons, voltage = machine.split(" ")
        lights = [1 if light == "#" else 0 for light in lights.strip("[]")]
        buttons = [tuple(map(int, button.strip("()").split(","))) for button in buttons]
        presses = 0
        found = False
        while not found:
            presses += 1
            combinations = combinations_with_replacement(buttons, presses)
            for combination in combinations:
                state = [0] * len(lights)
                for button in combination:
                    for pos in button:
                        state[pos] += 1
                if lights == [i%2 for i in state]:
                    button_presses += presses
                    found = True
                    break
    return button_presses