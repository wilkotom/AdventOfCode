def part1(labels: str) -> str:
    cups = [int(label) for label in labels]
    for i in range(0, 10):
        current_cup = cups[i % len(cups)]
        pick_up = [cups[(i+1) % len(cups)], cups[(i+2) % len(cups)], cups[(i+3) % len(cups)]]
        for c in pick_up:
            del cups[cups.index(c)]
        seek = current_cup - 1
        while seek not in cups:
            seek -= 1
            if seek < 1:
                seek = 9
        dest = cups.index(seek)
        cups = cups[:dest+1] + pick_up + cups[dest+1:]
        while cups[i % len(cups)] != current_cup:
            cups = cups[1:] + [cups[0]]
    split = cups.index(1)
    return ''.join([str(x) for x in cups[split+1:] + cups[:split]])


class Cup:
    def __init__(self, label: int) -> None:
        self.label = label
        self.next = None

    def __repr__(self):
        return f"Cup number: {self.label}"


def part2(labels: str, number_cups: int, number_steps: int) -> int:

    labels = [int(label) for label in labels]
    # Need a way of finding the place of a cup given its label - hence dict[int, Cup]
    # Create a million cups with no next cup
    lookup_table = {i: Cup(i) for i in range(1, number_cups + 1)}

    # Set each cup to have the numerically next cup as its successor
    for i in range(1, number_cups):
        lookup_table[i].next_val = lookup_table[i + 1]

    # Point the last cup in the list to the first in the specified order
    lookup_table[number_cups].next_val = lookup_table[labels[0]]

    # Arrange the first 9 cups according to the specified order
    for i in range(len(labels)):
        lookup_table[labels[i]].next_val = lookup_table[labels[(i + 1) % len(labels)]]

    # repoint the last cup in the specified list to the correct place (currently points back to the first cup)
    if number_cups > len(labels):
        lookup_table[labels[-1]].next_val = lookup_table[len(labels) + 1]

    # Start with the first cup in the specifed order
    current_cup = lookup_table[labels[0]]

    for _ in range(number_steps):
        # Remove the selection of 3 from the linked list
        selection = current_cup.next_val
        current_cup.next_val = current_cup.next_val.next_val.next_val.next_val
        seek = current_cup.label - 1 if current_cup.label > 1 else number_cups
        while seek in [current_cup.label, selection.label, selection.next_val.label, selection.next_val.next_val.label]:
            seek -= 1
            if seek < 1:
                seek = number_cups

        next_cup = lookup_table[seek]
        selection.next_val.next_val.next_val = next_cup.next_val
        next_cup.next_val = selection
        current_cup = current_cup.next_val

    return lookup_table[1].next_val.label * lookup_table[1].next_val.next_val.label


if __name__ == "__main__":
    print(part1("389125467"))
    print(part2("389125467", 1000000, 10000000))
