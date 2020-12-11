from itertools import product
from copy import deepcopy
from typing import Callable

directions = filter(lambda y: not y[0] == y[1] == 0,
                    product([-1, 0, 1], [-1, 0, 1]))


def get_adjacent_occupants(grid: list[str], row: int, seat: int) -> int:
    cols = filter(lambda n: 0 <= n < len(grid[0]), [seat + x for x in [-1, 0, 1]])
    seats = filter(lambda n: 0 <= n < len(grid), [row + x for x in [-1, 0, 1]])
    neighbours = filter(lambda x: x != (row, seat), product(seats, cols))
    count = 0
    for square in neighbours:
        count += 1 if grid[square[0]][square[1]] == "#" else 0
    return count


def get_visible_occupants(grid: list[str], row: int, seat: int) -> int:
    count = 0
    for direction in directions:
        multiplier = 1
        found = False
        while not found:
            y, x = row + (multiplier * direction[0]), seat + (multiplier * direction[1])
            if 0 <= y < len(grid) and 0 <= x < len(grid[0]):
                if grid[y][x] == '.':
                    multiplier += 1
                else:
                    found = True
                    count += 1 if grid[y][x] == '#' else 0
            else:
                found = True
    return count


def get_next_grid(grid: list[str], selection: Callable[[list[str], int, int], int], threshold: int) \
        -> tuple[bool, list[str]]:
    next_grid = []
    changed = False
    for row in range(len(grid)):
        new_row = ''
        for seat in range(len(grid[row])):
            if grid[row][seat] == 'L':
                if selection(grid, row, seat) == 0:
                    new_row += '#'
                    changed = True
                else:
                    new_row += 'L'
            elif grid[row][seat] == '#':
                if selection(grid, row, seat) < threshold:
                    new_row += '#'
                else:
                    new_row += 'L'
                    changed = True
            else:
                new_row += '.'
        next_grid.append(new_row)
    return changed, next_grid


def part1(grid: list[list[int]]) -> tuple[bool, list[str]]:
    return get_next_grid(grid, get_adjacent_occupants, 4)


def part2(grid: list[list[int]]) -> tuple[bool, list[str]]:
    return get_next_grid(grid, get_visible_occupants, 5)


def main(filename: str) -> None:
    original_grid = open(filename).read().split('\n')

    next_stage = (True, deepcopy(original_grid))
    while next_stage[0]:
        next_stage = part1(next_stage[1])
    print(sum([row.count('#') for row in next_stage[1]]))

    next_stage = (True, deepcopy(original_grid))
    while next_stage[0]:
        next_stage = part2(next_stage[1])

    print(sum([row.count('#') for row in next_stage[1]]))


if __name__ == "__main__":
    main("input.txt")