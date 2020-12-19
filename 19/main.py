import re
from functools import cache

rules = {}


@cache
def parse_rules(rule_id) -> str:
    print(rule_id)
    if rule_id in "ab|":
        return rule_id
    sub_rules = rules[rule_id].split(' ')
    if rule_id in sub_rules:
        if len(sub_rules) == 4:
            return "" + parse_rules(sub_rules[0]) + "+"
        if len(sub_rules) == 6:
            generated_rules = []
            for i in range(1, 5):  # Handling only 5 depths of recursion for speed - dataset only goes this deep
                generated_rules.append(f"({parse_rules(sub_rules[0])}{{{i}}}{parse_rules(sub_rules[1])}{{{i}}})")
            return "(" + '|'.join(generated_rules) + ")"
    new_rules = ''.join([parse_rules(x) for x in sub_rules])
    return new_rules if len(new_rules) == 1 or "|" not in new_rules else \
        "(" + ''.join([parse_rules(x) for x in sub_rules]) + ")"


def rules_to_dict(raw_rules: str) -> None:
    for raw_rule in raw_rules.split("\n"):
        number, rule = raw_rule.split(': ')
        rules[number] = rule.strip('"')


def main(filename: str) -> None:
    raw_rules, messages = open(filename).read().split('\n\n')
    rules_to_dict(raw_rules)
    print("^" + parse_rules("0") + "$")
    all_rules = re.compile("^" + parse_rules("0") + "$")
    total = 0
    for message in messages.split("\n"):
        print(message, all_rules.match(message))
        if all_rules.match(message):
            total += 1
    print(f"Part 1 total: {total}")


if __name__ == "__main__":
    main("input.txt")

# 423: too high