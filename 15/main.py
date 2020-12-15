def main(filename: str) -> None:
    numbers = [int(x) for x in open(filename).read().split(',')]
    spoken = {number: (0, turn+1) for turn, number in enumerate(numbers)}
    last = numbers[-1]
    game_length = 30000000
    for i in range(len(spoken) + 1, game_length + 1):
        last = 0 if spoken[last][0] == 0 else spoken[last][-1] - spoken[last][-2]
        spoken[last] = (0, i) if last not in spoken else (spoken[last][-1], i)
    print(f"Last number spoken was {last}")


if __name__ == '__main__':
    main("test.txt")