from typing import Callable
from functools import reduce


def build_rules(unparsed_rules: str) -> dict[str, Callable]:
    rules = {}
    for rule in unparsed_rules.split('\n'):
        rule_name, rule = rule.split(': ')
        rule = rule.replace('-', ' <= x <= ')
        rules[rule_name] = eval(f"lambda x: {rule}")
    return rules


def evaluate_part1_rules(ticket: list[str], rules: dict[str, Callable]) -> list[int]:
    invalid = []
    for value in ticket:
        if True not in (rules[rule](value) for rule in rules):
            invalid.append(value)
    return invalid


def evaluate_part2_rules(tickets: list[list[int]], rules: dict[str, Callable]) -> list[str]:
    ticket_values = list(zip(*tickets))
    possible_rules = {}
    for i in range(len(tickets[0])):
        possible_rules[i] = set(rules.keys())

    for i in range(len(ticket_values)):
        for rule in rules:
            possible_rule = False not in [rules[rule](x) for x in ticket_values[i]]
            if not possible_rule:
                possible_rules[i].remove(rule)

    fields = [None] * len(tickets[0])
    while None in fields:
        for i in possible_rules:
            if len(possible_rules[i]) == 1:
                fields[i] = list(possible_rules[i])[0]
                for j in possible_rules:
                    if fields[i] in possible_rules[j]:
                        possible_rules[j].remove(fields[i])
    return fields


def main(filename: str) -> None:
    data = open(filename).read().split('\n\n')
    rules = build_rules(data[0])
    my_ticket = [int(n) for n in data[1].split('\n')[1].split(',')]
    all_tickets = [my_ticket] + [[int(n) for n in x.split(',')] for x in data[2].split('\n')[1:]]
    scanning_error_rate = 0
    valid_tickets = [my_ticket]
    for ticket in all_tickets:
        errors = evaluate_part1_rules(ticket, rules)
        if len(errors) == 0:
            valid_tickets.append(ticket)
        scanning_error_rate += sum(errors)
    print(f"Part 1 answer: {scanning_error_rate}")
    headings = evaluate_part2_rules(valid_tickets, rules)
    print("Part 2 answer:", reduce(lambda x, y: x*y, [my_ticket[i] for i, heading in enumerate(headings)
                                                      if 'departure' in heading]))


if __name__== "__main__":
    main("input.txt")

    column5 = [131, 849, 794, 269, 210, 621, 205, 685, 852, 888, 580, 784, 642, 682, 205, 468, 571, 155, 751, 829, 471, 305, 655, 78, 61, 351, 220, 626, 565, 634, 434, 110, 70, 75, 129, 772, 354, 666, 681, 355, 73, 77, 896, 850, 83, 584, 476, 646, 631, 238, 596, 841, 805, 872, 361, 687, 587, 54, 281, 463, 473, 862, 730, 511, 652, 462, 752, 399, 165, 594, 589, 433, 605, 800, 120, 681, 363, 450, 132, 319, 323, 848, 296, 520, 145, 135, 802, 475, 200, 601, 912, 178, 303, 831, 851, 356, 105, 223, 626, 783, 241, 569, 462, 894, 801, 656, 239, 77, 746, 847, 881, 476, 363, 297, 173, 164, 843, 575, 527, 918, 493, 740, 109, 905, 657, 117, 761, 593, 434, 884, 206, 356, 560, 598, 71, 116, 166, 911, 139, 715, 635, 72, 274, 361, 905, 570, 595, 353, 705, 754, 718, 108, 178, 286, 228, 922, 910, 451, 788, 919, 620, 89, 720, 801, 896, 640, 537, 355, 461, 555, 466, 420, 365, 449, 374, 359, 620, 0, 802, 826, 175, 906, 249, 592, 888, 103, 627, 583, 793, 509, 161, 841, 920]