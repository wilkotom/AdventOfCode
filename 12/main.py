class Ferry:
    x: int = 0
    y: int = 0
    bearing: int = 90
    waypoint_x: int = 10
    waypoint_y: int = 1

    def move_part_1(self, instruction: str) -> None:
        bearings = {0: 'N', 90: 'E', 180: 'S', 270: 'W'}
        direction = instruction[0]
        value = int(instruction[1:])
        if direction == "F":
            direction = bearings[self.bearing]
        if direction in "NS":
            self.y += value * (1 if direction == "N" else -1)
        elif direction in "EW":
            self.x += value * (1 if direction == "E" else -1)
        elif direction in "RL":
            self.bearing = (self.bearing + value * (1 if direction == "R" else -1)) % 360

    def move_part_2(self, instruction: str) -> None:
        direction = instruction[0]
        value = int(instruction[1:])
        if direction == "F":
            self.x, self.y = self.x + value * self.waypoint_x, self.y + value * self.waypoint_y
        elif direction in "NS":
            self.waypoint_y += value * (1 if direction == "N" else -1)
        elif direction in "EW":
            self.waypoint_x += value * (1 if direction == "E" else -1)
        elif direction in "LR":
            for _ in range(((value if direction == "R" else 360 - value) // 90) % 360):
                self.waypoint_x, self.waypoint_y = self.waypoint_y, -self.waypoint_x


def main(filename: str) -> None:
    instructions = [line.strip() for line in open(filename)]

    ferry = Ferry()
    [ferry.move_part_1(i) for i in instructions]
    print(f"Part 1 Answer: {abs(ferry.x) + abs(ferry.y)}")

    ferry = Ferry()
    [ferry.move_part_2(i) for i in instructions]
    print(f"Part 2 Answer: {abs(ferry.x) + abs(ferry.y)}")


if __name__ == "__main__":
    main("input.txt")