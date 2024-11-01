from collections import defaultdict


def part_2(jolts: list[int]) -> int:
    paths_to_point = defaultdict(int)
    paths_to_point[0] = 1
    for j in jolts:
        paths_to_point[j] = paths_to_point[j-1] + paths_to_point[j-2] + paths_to_point[j-3]
    return paths_to_point[max(jolts)]


def part_1(jolts: list[int]) -> int:
    differences = defaultdict(int)
    differences[3] = 1
    previous = 0
    for i in range(len(jolts)):
        differences[jolts[i] - previous] +=1
        previous = jolts[i]
    print(differences)
    return differences[3] * differences[1]


def main() -> None:
    joltages = sorted([int(x) for x in open("input.txt")])
    print(f"Part 1: {part_1(joltages)}")
    print(f"Part 2: {part_2(joltages)}")


if __name__== "__main__":
    main()
