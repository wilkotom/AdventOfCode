from functools import wraps
from time import time

def timer(func):
    @wraps(func)
    def _time_it(*args, **kwargs):
        start = int(round(time() * 1000))
        try:
            return func(*args, **kwargs)
        finally:
            end_ = int(round(time() * 1000)) - start
            print(f"Total execution time: {end_ if end_ > 0 else 0} ms")
    return _time_it


class ListNode:
    def __init__(self, label, next_node=None, prev_node=None):
        self.label = label
        self.next_node = next_node
        self.prev_node = prev_node


@timer
def part1(iterations):

    elf_a = ListNode(3)
    elf_b = ListNode(7)
    elf_a.next_node = elf_b
    elf_b.next_node = elf_a

    list_start = elf_a
    list_end = elf_b

    for i in range(iterations):
        next_digits = str(elf_a.label + elf_b.label)
        for digit in [int(n) for n in next_digits]:
            new_end = ListNode(digit, list_start)
            list_end.next_node = new_end
            list_end = new_end
        for _ in range(elf_a.label + 1):
            elf_a = elf_a.next_node
        for _ in range(elf_b.label + 1):
            elf_b = elf_b.next_node

    current = list_start
    for i in range(iterations):
        current = current.next_node

    for i in range(10):
        print(current.label, end='')
        current = current.next_node
    print()

@timer
def part2(number_string):
    number_len = len(number_string)
    number = int(number_string)
    elf_a = ListNode(3)
    elf_b = ListNode(7, elf_a, elf_a)
    elf_a.next_node = elf_b
    elf_a.prev_node = elf_b

    list_start = elf_a
    look_back = elf_b
    look_back_index = 1
    list_end = elf_b

    found = False
    while not found:
        next_digits = str(elf_a.label + elf_b.label)
        for digit in [int(n) for n in next_digits]:
            new_end = ListNode(digit, list_start, list_end )
            list_end.next_node = new_end
            list_end = new_end
        for _ in range(elf_a.label + 1):
            elf_a = elf_a.next_node
        for _ in range(elf_b.label + 1):
            elf_b = elf_b.next_node
        while not found and look_back is not list_end:
            previous_digits = prev_nums(look_back, number_len)
            # print(look_back_index, previous_digits)
            if previous_digits == number:
                found = True
            else:
                look_back_index += 1
                look_back = look_back.next_node
    print(look_back_index - number_len)

def prev_nums(right_node, length=6):
    if length == 0:
        return 0
    else:
        return right_node.prev_node.label + 10 * prev_nums(right_node.prev_node, length -1)

@timer
def part_1_strings(iterations):
    recipes = "37"
    elf_a = 0
    elf_b = 1
    for i in range(iterations):
        next_digits = str(int(recipes[elf_a]) + int(recipes[elf_b]))
        recipes += next_digits
        elf_a = (elf_a + int(recipes[elf_a]) +1) % len(recipes)
        elf_b = (elf_b + int(recipes[elf_b]) +1) % len(recipes)
    print(recipes[iterations:iterations+10])

@timer
def part_2_strings(sought):
    recipes = "37"
    elf_a = 0
    elf_b = 1
    iterations = 0
    found = False
    while found is False:
        next_digits = str(int(recipes[elf_a]) + int(recipes[elf_b]))
        recipes += next_digits
        elf_a = (elf_a + int(recipes[elf_a]) +1) % len(recipes)
        elf_b = (elf_b + int(recipes[elf_b]) +1) % len(recipes)
        while iterations < len(recipes) -1 and len(sought) < len(recipes):
            if recipes[iterations - len(sought):iterations] == sought:
                found = True
                break
            else:
                iterations += 1


    print(iterations - len(sought))


if __name__== '__main__':
    part1(894501)


    part2("894501")

    part_1_strings(894501)
    part_2_strings("894501")