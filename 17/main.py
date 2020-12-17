from itertools import product
from typing import Iterator


def read_grid(filename: str, dimensions: int) -> set[tuple]:
    starting_state = set()
    grid = open(filename).read().split('\n')
    for y in range(len(grid)):
        for x in range(len(grid)):
            if grid[y][x] == '#':
                starting_state.add(tuple([x, y] + [0] * (dimensions - 2)))
    return starting_state


def get_bounds(grid: set[tuple[int, ...], bool]) -> list[tuple[int, ...]]:
    return [(min(d), max(d)) for d in list(zip(*grid))]


def get_neighbours(dimensions: int) -> list[tuple[int, ...]]:
    neighbours = list(product(*([[-1, 0, 1]] * dimensions)))
    neighbours.remove(tuple([0] * dimensions))
    return neighbours


def get_cubes(boundaries: list[tuple[int, ...]]) -> Iterator[tuple[int, ...]]:
    return product(*[range(x[0]-1, x[1]+2) for x in boundaries])


def cube_neighbours(cube: tuple[int, ...], neighbours: list[tuple[int, ...]]) -> list[tuple[int, ...]]:
    return [tuple(sum(x) for x in zip(cube, neighbour)) for neighbour in neighbours]


def n_dimensional_conway(filename: str, dimensions: int, generations: int) -> int:
    grid = read_grid(filename, dimensions)
    neighbours = get_neighbours(dimensions)
    new_grid = set()
    for i in range(generations):
        boundaries = get_bounds(grid)
        possible_cubes = get_cubes(boundaries)
        for cube in possible_cubes:
            active_neighbours = 0
            for neighbour in cube_neighbours(cube,neighbours):
                if neighbour in grid:
                    active_neighbours += 1
            if active_neighbours == 3 or (active_neighbours == 2 and cube in grid):
                new_grid.add(cube)
        grid = new_grid
        new_grid = set()
    return len(grid)


def main(filename: str) -> None:
    print(f"Part 1 Answer: {n_dimensional_conway(filename=filename, dimensions=3, generations=6)}")
    print(f"Part 2 Answer: {n_dimensional_conway(filename=filename, dimensions=4, generations=6)}")


if __name__ == "__main__":
    main("input.txt")