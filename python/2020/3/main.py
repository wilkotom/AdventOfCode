hillside = []
with open("input.txt") as file:
    while row := file.readline():
        hillside.append(row.strip())


def check_slope(xinc: int, yinc: int) -> int:
    x = 0
    y = 0
    trees = 0
    while y < len(hillside)-1:
        x += xinc
        y += yinc
        trees += 1 if hillside[y][x % len(hillside[y])] == "#" else 0
    return (trees)

print(check_slope(1,1) * check_slope(3,1) * check_slope(5,1) * check_slope(7,1) * check_slope(1,2))
