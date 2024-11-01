from typing import Callable, Union, Any
from functools import reduce


def build_rules(unparsed_rules: str) -> dict[str, Callable]:
    rules = {}
    for rule in unparsed_rules.split('\n'):
        rule_name, rule = rule.split(': ')
        rules[rule_name] = eval(f"lambda x: {rule.replace('-', ' <= x <= ')}")
    return rules


def evaluate_part1_rules(ticket: list[int], rules: dict[str, Callable]) -> list[int]:
    invalid = []
    for value in ticket:
        if True not in (rules[rule](value) for rule in rules):
            invalid.append(value)
    return invalid


def evaluate_part2_rules(tickets: list[list[int]], rules: dict[str, Callable]) -> list[Union[int, None]]:
    ticket_values = list(zip(*tickets))
    possible_rules: dict[int, set[Any]] = {}
    for i in range(len(tickets[0])):
        possible_rules[i] = set(rules.keys())

    for i in range(len(ticket_values)):
        for rule in rules:
            possible_rule = False not in [rules[rule](x) for x in ticket_values[i]]
            if not possible_rule:
                possible_rules[i].remove(rule)

    fields: list[Union[int, None]] = [None] * len(tickets[0])
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
    valid_tickets = []
    for ticket in all_tickets:
        errors = evaluate_part1_rules(ticket, rules)
        if len(errors) == 0:
            valid_tickets.append(ticket)
        scanning_error_rate += sum(errors)
    print(f"Part 1 answer: {scanning_error_rate}")
    headings = evaluate_part2_rules(valid_tickets, rules)
    print("Part 2 answer:", reduce(lambda x, y: x*y, [my_ticket[i] for i, heading in enumerate(headings)
                                                      if 'departure' in heading]))


if __name__ == "__main__":
    main("input.txt")
