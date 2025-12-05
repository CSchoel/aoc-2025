import argparse
import os

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Advent of Code solution.")
    parser.add_argument("day", type=int, choices=list(range(1, 13)), help="Day of the challenge (1-12)")
    parser.add_argument("part", type=int, choices=[1, 2], help="Part of the challenge (1 or 2)")
    parser.add_argument("--input", action="store_true", help="Use actual input instead of sample")
    args = parser.parse_args()
    sample = "input" if args.input else "sample"
    module = __import__(f"day{args.day:02d}.part{args.part}", fromlist=["main"])
    base_path = os.path.dirname(os.path.abspath(__file__))
    input_path = os.path.join(base_path, f"day{args.day:02d}", f"{sample}.txt")
    with open(input_path, "r") as f:
        input = f.read()
    result = module.main(input)
    print("Day", args.day, "- Part", args.part, "- sample input" if not args.input else "- actual input")
    print("Result:", result)
