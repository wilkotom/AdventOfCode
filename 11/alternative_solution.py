from copy import deepcopy
from typing import Callable


def neighbours_part_1(grid: list[str], row: int, seat: int) -> list[tuple[int, int]]:
    result = []
    directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    for direction in directions:
        y, x = row + direction[0], seat + direction[1]
        if 0 <= y < len(grid) and 0 <= x < len(grid[0]) and grid[y][x] != '.':
            result.append((y, x))
    return result


def neighbours_part_2(grid: list[str], row: int, seat: int) -> list[tuple[int, int]]:
    directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    result = []
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
                    result.append((y, x))
            else:
                found = True
    return result


def get_neighbours(grid: list[str], selection: Callable) -> dict[tuple[int,int], list[tuple[int, int]]]:
    result = {}
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] != '.':
                result[(i, j)] = selection(grid, i,j)
    return result


def get_result(grid: list[str], neighbours: dict[tuple[int,int], list[tuple[int, int]]], threshold: int) -> int:
    changed = True

    for neighbour in deepcopy(neighbours):
        if len(neighbours[neighbour]) < threshold:
            grid[neighbour[0]][neighbour[1]] = '#'
            del neighbours[neighbour]
    while changed:
        changed = False
        existing = deepcopy(grid)
        for square in neighbours:
            current = existing[square[0]][square[1]]
            total = 0
            for neighbour in neighbours[square]:
                total += 1 if existing[neighbour[0]][neighbour[1]] == '#' else 0
            if total == 0 and current != '#':
                grid[square[0]][square[1]] = '#'
                changed = True
            elif total >= threshold and grid[square[0]][square[1]] != 'L':
                grid[square[0]][square[1]] = 'L'
                changed = True
    return sum([row.count('#') for row in grid])


def main(filename: str) -> None:
    original_grid = [list(r) for r in open(filename).read().split('\n')]
    neighbours = get_neighbours(original_grid, neighbours_part_1)
    print(f"Part 1: {get_result(deepcopy(original_grid), neighbours, 4)}")
    neighbours = get_neighbours(original_grid, neighbours_part_2)
    print(f"Part 2: {get_result(deepcopy(original_grid), neighbours, 5)}")


if __name__ == "__main__":
    main("input.txt")