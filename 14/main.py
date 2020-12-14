def split_bit_masks(bitmask: str) -> tuple[int, int, int]:
    ones = zeros = exes = 0
    for i, c in enumerate(bitmask[::-1]):
        if c == '1' or c == 'X':
            if c == '1':
                ones += 2 ** i
            exes += 2 ** i
        elif c == '0':
            zeros += 2 ** i
    return ones, zeros, exes


def get_decrements(bitmask: str, power: int = 0) -> list[int]:
    if bitmask == "":
        return [0]
    else:
        if bitmask[0] == 'X':
            poss_decrements = get_decrements(bitmask[1:], power + 1)
            return [n for n in poss_decrements] + [2 ** power + n for n in poss_decrements]
        else:
            return get_decrements(bitmask[1:], power + 1)


def main(filename: str) -> None:
    part1_memory = {}
    part2_memory = {}
    ones = zeros = exes = 0
    fluctuations = []
    for line in open(filename):
        instruction, _, value = line.strip().split()
        if instruction == 'mask':
            ones, zeros, exes = split_bit_masks(value)
            fluctuations = get_decrements(value[::-1])
        elif instruction[:4] == 'mem[':
            part1_memory[int(instruction[4:-1])] = (int(value) | ones) & ~ zeros
            addresses = [(int(instruction[4:-1]) | exes) & ~ decrement for decrement in fluctuations]
            for address in addresses:
                part2_memory[address] = int(value)
        else:
            print(f"Didn't understand {instruction}")
    print(f"Part 1: {sum(part1_memory.values())}")
    print(f"Part 2: {sum(part2_memory.values())}")


if __name__ == "__main__":
    main("input.txt")
