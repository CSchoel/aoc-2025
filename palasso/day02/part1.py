def main(input: str) -> int:
    ranges = [tuple(map(int, r.split("-"))) for r in input.split(",")]
    invalids = 0
    for (start, end) in ranges:
        for id in range(start, end + 1):
            str_id = str(id)
            if len(str_id) % 2 == 0:
                if str_id[:len(str_id)//2] == str_id[len(str_id)//2:]:
                    invalids += id
    return invalids
