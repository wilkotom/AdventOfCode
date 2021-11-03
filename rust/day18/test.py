def calc1(line):
    nums = [int(x) for x in line if x not in ['+', '*']]
    ops = [x for x in line if x in ['+', '*']]
    total = nums.pop(0)

    for n in nums:
        if ops == []:
            break
        else:
            total = eval(str(total) + ops.pop(0) + str(n))

    return total


def calc2(line):

    while '+' in line:
        for i, j in enumerate(line):
            if j == '+':
                total = int(line[i - 1]) + int(line[i + 1])
                line = line[:i - 1] + [total] + line[i + 2:]
                break

    return eval(''.join([str(x) for x in line]))


data = open("test.txt").read().strip().split('\n')

# part 1
p1 = 0

for line in data:
    line = line.replace('(', '( ').replace(')', ' )').split(' ')

    while '(' in line:
        start = 0
        end = 0

        for i, j in enumerate(line):
            if j == '(':
                start = i
            if j == ')':
                end = i
                total = calc1(line[start + 1:end])
                line = line[:start] + [total] + line[end + 1:]
                break
    x = calc1(line)
    print(x)
    p1 += x

print(f'Part 1: {p1}')
