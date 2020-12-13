from functools import reduce


def chinese_remainder_gaussian(a, mod_prod, n):
    result = 0
    for i in range(len(n)):
        result += a[i] * mod_prod // n[i] * inverse_mod((mod_prod // n[i]), n[i])
    return result % mod_prod


def inverse_mod(a: int, b: int) -> tuple[int, int, int]:
    d = b
    x0, x1, y0, y1 = 0, 1, 1, 0
    while a != 0:
        (q, a), b = divmod(b, a), a
        y0, y1 = y1, y0 - q * y1
        x0, x1 = x1, x0 - q * x1
    return x0 % d


def main(filename: str) -> None:
    # Part 1
    with open(filename) as file:
        departure = int(file.readline().strip())
        buses = [int(x) for x in file.readline().split(',') if x != 'x']
    next_bus_time = [bus + departure - (departure % bus) for bus in buses]
    next_bus = next_bus_time.index(min(next_bus_time))
    print(f"Part 1: {buses[next_bus] * (next_bus_time[next_bus] - departure)}")

    # Part 2
    with open(filename) as file:
        file.readline()
        bus_ids = file.readline().strip().split(',')
    pairs = []
    count = 0
    for bus_id in bus_ids:
        if bus_id != 'x':
            pairs.append((int(bus_id), count))
        count += 1
    moduli_product = reduce(lambda x, y: x * y, [p[0] for p in pairs])
    print(f"Part 2 answer: {chinese_remainder_gaussian([p[0] - p[1] for p in pairs], moduli_product, [p[0] for p in pairs])}")


if __name__ == "__main__":
    main("input.txt")

