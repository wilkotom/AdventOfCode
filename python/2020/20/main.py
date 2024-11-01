from collections import Counter
from functools import reduce
grids = {}


class ImageFragment:

    def __init__(self, id: str, grid: list[str]) -> None:
        self.id = id
        self.grid = [list(x) for x in grid]
        self.orientation = 0
        self.right = self.left = self.up = self.down = ''

    def __repr__(self) -> str:
        return f' Grid ID: {self.id}\n' + '\n'.join([''.join(c) for c in self.grid]) + \
               f'\n Right: {self.right}, Down: {self.down}, Left: {self.left}, Up: {self.up}\n\n'

    def rotate(self) -> None:
        self.grid = list(zip(*self.grid))[::-1]
        self.right, self.down, self.left, self.up = self.up,  self.right, self.down, self.left
        self.orientation = (self.orientation - 90) % 360

    @property
    def top_edge(self) -> list[str]:
        return list(self.grid[0])

    @property
    def bottom_edge(self) -> list[str]:
        return list(self.grid[-1])

    @property
    def left_edge(self) -> list[str]:
        return [line[0] for line in self.grid]

    @property
    def right_edge(self) -> list[str]:
        return [line[-1] for line in self.grid]

    @property
    def is_corner(self) -> bool:
        return Counter([self.right, self.down, self.left, self.up])[''] == 2

    def flip_top_to_bottom(self) -> None:
        self.grid = self.grid[::-1]
        self.up, self.down = self. down, self.up

    def flip_right_to_left(self) -> None:
        self.grid = [r[::-1] for r in self.grid]
        self.right, self.left = self.left, self.right


def join_pieces(joined: set) -> bool:
    # print(f"Joined: {joined}")
    joined_pieces = len(joined)
    for aligned_piece in list(joined):
        for piece in grids:
            if grids[piece].top_edge == grids[aligned_piece].bottom_edge:
                grids[piece].up = aligned_piece
                grids[aligned_piece].down = piece
                joined.add(piece)
            elif grids[piece].bottom_edge == grids[aligned_piece].top_edge:
                grids[piece].down = aligned_piece
                grids[aligned_piece].up = piece
                joined.add(piece)
            elif grids[piece].right_edge == grids[aligned_piece].left_edge:
                grids[piece].right = aligned_piece
                grids[aligned_piece].left = piece
                joined.add(piece)
            elif grids[piece].left_edge == grids[aligned_piece].right_edge:
                grids[piece].left = aligned_piece
                grids[aligned_piece].right = piece
                joined.add(piece)
    return joined_pieces != len(joined)


def file_to_grid(filename: str) -> None:
    data = open(filename).read().split('\n\n')
    for grid in [g.split('\n') for g in data]:
        grid_id = grid[0].strip(':').split()[1]
        grids[grid_id] = ImageFragment(grid_id, grid[1:])


def find_sea_monsters(grid: list[list[str]]) -> int:
    count = 0
    for y in range(1, len(grid)-2):
        for x in range(len(grid[0]) - 19):
            if '.' not in (grid[y][x], grid[y + 1][x + 1], grid[y + 1][x + 4], grid[y][x + 5], grid[y][x + 6],
                           grid[y + 1][x + 7], grid[y + 1][x + 10], grid[y][x + 11], grid[y][x + 12],
                           grid[y + 1][x + 13], grid[y + 1][x + 16], grid[y][x + 17], grid[y - 1][x + 18],
                           grid[y][x + 18], grid[y][x + 19]):
                grid[y][x] = grid[y + 1][x + 1] = grid[y + 1][x + 4] = grid[y][x + 5] = grid[y][x + 6] = \
                grid[y + 1][x + 7] = grid[y + 1][x + 10] = grid[y][x + 11] = grid[y][x + 12] = \
                grid[y + 1][x + 13] = grid[y + 1][x + 16] = grid[y][x + 17] = grid[y - 1][x + 18] = \
                grid[y][x + 18] = grid[y][x + 19] = '\033[92m#\033[0m'
                count += 1
    return count


def main(filename: str) -> None:
    file_to_grid(filename)
    # print(grids)
    starting_piece = list(grids.keys())[0]
    all_pieces = set(grids.keys())
    joined = set()
    joined.add(starting_piece)
    while len(joined) < len(grids):
        while join_pieces(joined):
            pass
        for piece in all_pieces.difference(joined):
            grids[piece].rotate()
        for _ in range(4):
            while join_pieces(joined):
                pass
            for piece in all_pieces.difference(joined):
                grids[piece].rotate()
        for piece in all_pieces.difference(joined):
            grids[piece].flip_right_to_left()
        for _ in range(4):
            while join_pieces(joined):
                pass
            for piece in all_pieces.difference(joined):
                grids[piece].rotate()
        for piece in all_pieces.difference(joined):
            grids[piece].flip_top_to_bottom()
        for _ in range(4):
            while join_pieces(joined):
                pass
            for piece in all_pieces.difference(joined):
                grids[piece].rotate()
        for piece in all_pieces.difference(joined):
            grids[piece].flip_right_to_left()
        for _ in range(4):
            while join_pieces(joined):
                pass
            for piece in all_pieces.difference(joined):
                grids[piece].rotate()

    print("Part 1 answer: ", reduce(lambda x, y: x*y, [int(grids[gid].id) for gid in grids if grids[gid].is_corner]))

    grid_ordering = []
    starting_point = [gid for gid in grids if grids[gid].left == '' and grids[gid].up == ''][0]
    while starting_point != '':
        next_line = grids[starting_point].down
        point = starting_point
        this_line = []
        while point != '':
            this_line.append(point)
            point = grids[point].right
        grid_ordering.append(this_line)
        starting_point = next_line

    for grid in grids:
        grids[grid].grid = [x[1:-1] for x in grids[grid].grid[1:-1]]

    new_grid = []
    for squares in grid_ordering:
        for line in range(8):
            new_line = []
            for grid in squares:
                new_line += grids[grid].grid[line]
            new_grid.append(new_line)

    monster_count = 0
    for _ in range(3):
        monster_count = find_sea_monsters(new_grid)
        if monster_count > 0:
            break
        new_grid = [list(x) for x in zip(*new_grid)][::-1]
    if monster_count == 0:
        new_grid = new_grid[::-1]
        for _ in range(3):
            monster_count = find_sea_monsters(new_grid)
            if monster_count > 0:
                break
            new_grid = [list(x) for x in zip(*new_grid)][::-1]
    if monster_count == 0:
        new_grid = [x[::-1] for x in new_grid]
        for _ in range(3):
            monster_count = find_sea_monsters(new_grid)
            if monster_count > 0:
                break
            new_grid = [list(x) for x in zip(*new_grid)][::-1]
    if monster_count == 0:
        new_grid = new_grid[::-1]
        for _ in range(3):
            monster_count = find_sea_monsters(new_grid)
            if monster_count > 0:
                break
            new_grid = [list(x) for x in zip(*new_grid)][::-1]
    print('\n'.join([''.join(x) for x in new_grid]))
    print(f"Found {monster_count} monsters")
    totals = Counter([square for line in new_grid for square in line])
    print(f"Water roughness: {totals['#']}")


if __name__ == '__main__':
    main("input.txt")
