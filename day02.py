from pathlib import Path


def parse(file_path: Path) -> list[tuple[int, ...]]:
    output: list[tuple[int]] = []

    with file_path.open("r") as file:
        for line in file.read().splitlines():
            output.append(tuple([int(n) for n in line.split(" ")]))

    return output


def is_safe(row: tuple[int, ...]) -> bool:
    if len(row) < 2:
        return False

    diffs = [row[i + 1] - row[i] for i in range(len(row) - 1)]
    inc = all(1 <= d <= 3 for d in diffs)
    dec = all(-3 <= d <= -1 for d in diffs)
    return inc or dec


def part1(rows: list[tuple[int, ...]]) -> int:
    total = 0

    for row in rows:
        if len(row) < 2:
            continue

        if is_safe(row):
            total += 1

    return total


def part2(rows: list[tuple[int, ...]]) -> int:
    total = 0

    for row in rows:
        if is_safe(row):
            total += 1
        else:
            for i in range(len(row)):
                new_row = row[:i] + row[i + 1:]
                if is_safe(new_row):
                    total += 1
                    break

    return total


if __name__ == "__main__":
    rows = parse(Path("input/day02.txt"))

    result = part1(rows)
    print("Part 1:", result)

    result = part2(rows)
    print("Part 2:", result)
