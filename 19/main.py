import re
from functools import cache

rules = {}


@cache
def parse_rules(rule_id) -> str:
    if rule_id in "ab|":
        return rule_id
    sub_rules = rules[rule_id].split(' ')
    if rule_id in sub_rules:
        if len(sub_rules) == 4:
            return "" + parse_rules(sub_rules[0]) + "+"
        elif len(sub_rules) == 6:
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
    all_rules = re.compile("^" + parse_rules("0") + "$")
    total = sum((int(all_rules.match(m) is not None) for m in messages.split("\n")))
    print(f"Total matches: {total}")


if __name__ == "__main__":
    main("input.txt")