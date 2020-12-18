from functools import reduce
from typing import Union, Callable


def collapse_part_1(expression: list[Union[int, str]]) -> int:
    expression[2] = expression[0] + expression[2] if expression[1] == "+" else expression[0] * expression[2]
    return evaluate_expression(expression[2:], collapse_part_1)


def collapse_part_2(expression: list[Union[int, str]]) -> int:

    def collapse(expr: list[Union[int, str]]) -> list[Union[int, str]]:
        if len(expr) == 1:
            return expr
        elif expr[1] == '+':
            return collapse([expr[0] + expr[2]] + expr[3:])
        else:
            return expr[:2] + collapse(expr[2:])

    return reduce(lambda x, y: x * y, collapse(expression)[::2])


def evaluate_expression(expression: list[Union[int, str]], collapse: Callable[[list[Union[int,str]]], int]) -> int:
    if len(expression) == 1:
        return expression[0]
    if '(' not in expression:
        return collapse(expression)
    else:
        start = expression.index('(')
        counter = i = 0
        for i in range(start, len(expression)):
            if expression[i] == '(':
                counter += 1
            elif expression[i] == ')':
                counter -= 1
            if counter == 0:
                break
        new_expression = expression[start+1:i]
        return evaluate_expression(
            expression[:start] + [evaluate_expression(new_expression, collapse)] + expression[i + 1:], collapse)


def main(filename: str) -> None:
    total_part_1 = 0
    total_part_2 = 0
    for expression in (l.strip().replace(' ', '') for l in open(filename).readlines()):
        total_part_1 += evaluate_expression([x if x in "+*()" else int(x) for x in expression], collapse_part_1)
        total_part_2 += evaluate_expression([x if x in "+*()" else int(x) for x in expression], collapse_part_2)
    print(total_part_1, total_part_2)


if __name__ == "__main__":
    main("input.txt")