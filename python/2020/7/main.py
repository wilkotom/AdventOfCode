from functools import cache

rules = {}


@cache
def can_contain(colour: str, bag: str) -> bool:
    if not rules[bag]:
        return False
    elif colour in rules[bag]:
        return True
    else:
        return True in [can_contain(colour, nextbag) for nextbag in rules[bag]]

@cache
def contains_count(colour: str) -> bool:
    if not rules[colour]:
        return 0
    total = 0
    for bag in rules[colour]:
        total += rules[colour][bag] * (contains_count(bag) +1)
    return total


def main() -> None:
    for rule in open("input.txt").readlines():
        rule = rule.replace('bags', 'bag').replace(' contain ', ':').strip('.\n')
        bag, contents = rule.split(':')
        rules[bag] = {}
        if contents != "no other bag":
            for content in contents.split(', '):
                quantity, colour = content.split(' ', 1)
                rules[bag][colour] = int(quantity)
    print(f'Part 1: {[can_contain("shiny gold bag", bag) for bag in rules].count(True)}')
    print(f'Part 2: {contains_count("shiny gold bag")}')


if __name__ == "__main__":
    main()