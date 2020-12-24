from collections import deque, Counter


def part1(filename: str) -> set[tuple[int, int]]:
    instructions = deque(open(filename).read())
    x = y = 0
    black_tiles = set()
    while len(instructions) > 0 :
        instruction = instructions.popleft()
        if instruction == '\n':
            if (x, y) in black_tiles:
                black_tiles.remove((x, y))
            else:
                black_tiles.add((x, y))
            x = y = 0
        elif instruction in 'ns':
            y = y + 1 if instruction == 'n' else y - 1
            instruction = instructions.popleft()
            x = x + 1 if instruction == 'e' else x -1
        else:
            x = x+ 2 if instruction == 'e' else x - 2

    if (x, y) in black_tiles:
        black_tiles.remove((x, y))
    else:
        black_tiles.add((x, y))

    return black_tiles


def part2(black_tiles: set[tuple[int, int]], generations: int) -> int:
    adjoining = [(1, 1), (2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1)]
    state = {tile: True for tile in black_tiles}
    for i in range(generations):
        new_state = {}
        for tile in list(state.keys()):
            for neighbour in [(n[0] + tile[0], n[1] + tile[1]) for n in adjoining]:
                if neighbour not in state:
                    state[neighbour] = False
        for tile in state:
            neighbour_count = Counter([state.get((n[0] + tile[0], n[1] + tile[1]), False) for n in adjoining])[True]
            if neighbour_count == 2 or (neighbour_count == 1 and state[tile]):
                new_state[tile] = True
        state = new_state
    return len(state)


def main(filename: str) -> None:
    grid = part1(filename)
    print(f"Part 1 answer: {len(grid)}")
    print(f"Part 2 answer: {part2(grid ,100)}")


if __name__ == "__main__":
    main("input.txt")