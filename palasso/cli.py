if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Run Advent of Code solution.")
    parser.add_argument("day", type=int, help="Day of the challenge (1-25)")
    parser.add_argument("part", type=int, choices=[1, 2], help="Part of the challenge (1 or 2)")
    parser.add_argument("--input", action="store_true", help="Use actual input instead of sample")
    args = parser.parse_args()
    sample = "input" if args.input else "sample"
    module = __import__(f"day{args.day:02d}.part{args.part}", fromlist=["main"])
    with open(f"palasso/day{args.day:02d}/{sample}.txt", "r") as f:
        input = f.read()
    result = module.main(input)
    print("Day:", args.day, "Part:", args.part, "Using sample input:" if not args.input else "Using actual input")
    print("Result:", result)
