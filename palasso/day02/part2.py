def main(input: str) -> int:
    ranges = [tuple(map(int, r.split("-"))) for r in input.split(",")]
    invalids = 0
    for (start, end) in ranges:
        for id in range(start, end + 1):
            str_id = str(id)
            for times in range(2, len(str_id)+1):
                if len(str_id) % times == 0:
                    window_size = len(str_id) // times
                    parts = [str_id[i * window_size : (i + 1) * window_size] for i in range(times)]
                    if all(part == parts[0] for part in parts):
                        invalids += id
                        break
    return invalids
