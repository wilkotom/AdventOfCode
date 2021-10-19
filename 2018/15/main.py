from collections import deque
import sys

grid = {}


class Combatant:

    def __init__(self, side: str, x, y: int, attack_power: int = 3):
        self.side = side
        self.x = x
        self.y = y
        self.hit_points = 200
        self.attack_power = attack_power

    def __str__(self):
        return self.side

    def __repr__(self):
        return f"{'Elf' if self.side == 'E' else 'Goblin'} at {(self.x, self.y)} HP: {self.hit_points}"

    def __lt__(self, other):
        return self.x < other.x if self.y == other.y else self.y < other.y

    def move_unit(self, combatants):
        if True in [type(grid[(self.x + dx, self.y + dy)]) is Combatant and grid[
            (self.x + dx, self.y + dy)].side != self.side for dx, dy in
                        ((0, -1), (-1, 0), (1, 0), (0, 1))]:
            # Unit is already next to an enemy. Don't move.
            return

        in_range = set()
        for enemy in [c for c in combatants if c.side != self.side and c.hit_points > 0]:
            for square in enemy.empty_neighbours():  # empty_neighbours(enemy.x, enemy.y)):
                in_range.add(square)
        all_square_distances = get_distances((self.x, self.y))
        target = (-1, -1)
        target_distance = sys.maxsize
        for square in in_range:
            if square in all_square_distances and all_square_distances[square] < target_distance:
                target = square
                target_distance = all_square_distances[square]
            elif square in all_square_distances and all_square_distances[square] == target_distance:
                if square[1] < target[1] or square[1] == target[1] and square[0] < target[0]:
                    target = square

        if target != (-1, -1):
            distances_from_target = get_distances(target)
            shortest_path = sys.maxsize
            next_square = (-1, -1)
            for s in self.empty_neighbours():  # empty_neighbours((self.x, self.y)):
                if distances_from_target.get(s, sys.maxsize) < shortest_path:
                    shortest_path = distances_from_target[s]
                    next_square = s
                elif distances_from_target.get(s, sys.maxsize) == shortest_path:
                    if s[1] < next_square[1] or s[1] == next_square[1] and s[0] < next_square[0]:
                        next_square = s
            grid[(self.x, self.y)] = '.'
            grid[next_square] = self
            self.x = next_square[0]
            self.y = next_square[1]

    def attack(self):
        possible_targets = [(self.x + dx, self.y + dy) for dx, dy in ((0, -1), (-1, 0), (1, 0), (0, 1)) if
                            type(grid[(self.x + dx, self.y + dy)]) is Combatant and grid[
                                (self.x + dx, self.y + dy)].side != self.side]
        max_health = 201
        final_target = None
        for target in possible_targets:
            if grid[target].hit_points < max_health:
                final_target = grid[target]
                max_health = final_target.hit_points

        if final_target is not None:
            final_target.hit_points -= self.attack_power
            if final_target.hit_points < 0:
                grid[(final_target.x, final_target.y)] = '.'

    def empty_neighbours(self):
        return [(self.x + dx, self.y + dy) for dx, dy in ((0, 1), (1, 0), (-1, 0), (0, -1)) if
                grid.get((self.x + dx, self.y + dy), '#') == '.']


def get_distances(start):
    distances = {start: 0}
    visited = set()
    next_squares = deque([start])
    while len(next_squares) > 0:
        square = next_squares.popleft()
        distance = sys.maxsize
        if square in visited:
            continue
        for n in [(square[0] + dx, square[1] + dy) for dx, dy in ((0, -1), (-1, 0), (1, 0), (0, 1)) if
                  grid.get((square[0] + dx, square[1] + dy), '#') == '.'
                  or (square[0] + dx, square[1] + dy) == start]:
            candidate_distance = distances.get(n, sys.maxsize) + 1
            distance = candidate_distance if candidate_distance < distance else distance
            if n not in visited and grid[n] == '.':
                next_squares.append(n)
        visited.add(square)
        distances[square] = distance if distance < distances.get(square, sys.maxsize) else distances[square]
    return distances


def print_grid(max_x, max_y):
    for y in range(max_y):
        for x in range(max_x):
            print(grid[(x, y)], end='')
        print()


def part1(filename: str):
    combatants = []
    for y, line in enumerate(open(filename).readlines()):
        for x, c in enumerate(line):
            if c in 'EG':
                grid[x, y] = Combatant(c, x, y)
                combatants.append(grid[x, y])
            else:
                grid[x, y] = c
        y += 1
    turn = 0
    while True:
        for combatant in combatants:
            if combatant.hit_points < 0:
                continue
            combatant.move_unit(combatants)
            combatant.attack()
            remaining_enemies = len([c for c in combatants if c.side != combatant.side and c.hit_points > 0])
            if remaining_enemies == 0:
                total_winning_hp = sum([c.hit_points for c in combatants if c.hit_points > 0])
                print(f"Part 1 conclusion: Combat ended on turn {turn + 1}, remaining health {total_winning_hp}")
                print(f"End score: {turn * total_winning_hp}")
                return

        combatants.sort()
        turn += 1


def part2(filename: str):
    attack_power = 4
    while True:
        combatants = []
        for y, line in enumerate(open(filename).readlines()):
            for x, c in enumerate(line):
                if c == 'G':
                    grid[x, y] = Combatant(c, x, y)
                    combatants.append(grid[x, y])
                elif c == 'E':
                    grid[x, y] = Combatant(c, x, y, attack_power)
                    combatants.append(grid[x, y])
                else:
                    grid[x, y] = c
            y += 1
        turn = 0
        dead_elves = False
        while not dead_elves:
            for combatant in combatants:
                if combatant.hit_points < 0 and combatant.side == 'G':
                    continue
                elif combatant.hit_points < 0 and combatant.side == 'E':
                    dead_elves = True
                    attack_power += 1
                    break
                combatant.move_unit(combatants)
                combatant.attack()
                remaining_enemies = len([c for c in combatants if c.side != combatant.side and c.hit_points > 0])
                if remaining_enemies == 0:
                    total_winning_hp = sum([c.hit_points for c in combatants if c.hit_points > 0])
                    print(f"Part 2 conclusion: Attack power {attack_power}")
                    print(f"Combat ended on turn {turn + 1}, remaining health {total_winning_hp}")
                    print(f"End score: {turn * total_winning_hp}")
                    return

            combatants.sort()
            turn += 1


if __name__ == '__main__':
    file = "input.txt"
    part1(file)
    part2(file)
