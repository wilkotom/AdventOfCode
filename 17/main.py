from itertools import product


def read_grid(filename: str, dimensions: int) -> set[tuple]:
    starting_state = set()
    grid = open(filename).read().split('\n')
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == '#':
                starting_state.add(tuple([x, y] + [0] * (dimensions - 2)))
    return starting_state


def get_neighbours(dimensions: int) -> set[tuple[int, ...]]:
    neighbours = set(product(*([[-1, 0, 1]] * dimensions)))
    neighbours.remove(tuple([0] * dimensions))
    return neighbours


def get_potential_cubes(grid: set[tuple], neighbours: set[tuple[int, ...]]) -> set[tuple[int, ...]]:
    results = set()
    for cube in grid:
        [results.add(n) for n in cube_neighbours(cube, neighbours)]
    return set(results)


def cube_neighbours(cube: tuple[int, ...], neighbours: set[tuple[int, ...]]) -> list[tuple[int, ...]]:
    return [tuple(sum(x) for x in zip(cube, neighbour)) for neighbour in neighbours]


def n_dimensional_conway(filename: str, dimensions: int, generations: int) -> int:
    grid = read_grid(filename, dimensions)
    neighbours = get_neighbours(dimensions)
    new_grid = set()
    for i in range(generations):
        possible_cubes = get_potential_cubes(grid, neighbours)
        for cube in possible_cubes:
            active_neighbours = sum((int(neighbour in grid) for neighbour in cube_neighbours(cube, neighbours)))
            if active_neighbours == 3 or (active_neighbours == 2 and cube in grid):
                new_grid.add(cube)
        grid = new_grid
        new_grid = set()
    return len(grid)


def main(filename: str) -> None:
    print(f"Part 1 Answer: {n_dimensional_conway(filename=filename, dimensions=3, generations=6)}")
    print(f"Part 2 Answer: {n_dimensional_conway(filename=filename, dimensions=4, generations=6)}")


if __name__ == "__main__":
    main("otherinput.txt")
