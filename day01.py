from pathlib import Path


def parse(file_path: Path) -> tuple[list[int], list[int]]:
    left_col: list[int] = list()
    right_col: list[int] = list()

    with open(file_path, "r") as file:
        for line in file.readlines():
            left, right = line.split()
            left_col.append(int(left))
            right_col.append(int(right))

    return left_col, right_col


print(parse(Path("input/day01.txt")))
