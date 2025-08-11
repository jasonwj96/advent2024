from pathlib import Path


def parse(file_path: Path) -> tuple[list[int], list[int]]:
    left_col = []
    right_col = []

    with file_path.open("r") as file:
        for line in file:
            left, right = line.strip().split()
            left_col.append(int(left))
            right_col.append(int(right))

    return left_col, right_col


def part1(left: list[int], right: list[int]) -> int:
    left.sort()
    right.sort()

    return sum((abs(a - b) for a, b in zip(left, right)))


def part2(left: list[int], right: list[int]) -> int:
    total = 0
    counts = dict.fromkeys(set(left), 0)

    for i in right:
        if i in counts:
            counts[i] += 1

    for i in left:
        total += i * counts[i]

    return total


if __name__ == "__main__":
    input_path = Path("input/day01.txt")
    left, right = parse(input_path)

    result_part1 = part1(left, right)
    print("Part 1:", result_part1)

    result_part2 = part2(left, right)
    print("Part 2:", result_part2)
