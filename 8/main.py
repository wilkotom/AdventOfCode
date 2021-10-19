
def read_instructions(filename: str) -> list[list[str, int]]:
    program = []
    for line in open(filename).readlines():
        opcode, val = line.strip().split()
        program.append([opcode,int(val)])
    return program


def run_program(prog: list[list[str, int]]) -> tuple[bool, int]:
    accumulator = 0
    visited = set()
    instruction = 0
    while instruction not in visited and instruction < len(prog):
        visited.add(instruction)
        if prog[instruction][0] == "nop":
            instruction += 1
        elif prog[instruction][0] == "acc":
            accumulator += prog[instruction][1]
            instruction += 1
        elif prog[instruction][0] == "jmp":
            instruction += prog[instruction][1]
    return instruction not in visited, accumulator


def main(filename: str) -> None:
    program = read_instructions(filename)
    print(f"Part 1: {run_program(program)[1]}")
    possible_flips = []
    for i, instr in enumerate(program):
        if program[i][0] in ("jmp", "nop"):
            possible_flips.append(i)

    for flip in possible_flips:
        new_program = [x.copy() for x in program]
        new_program[flip][0] = "jmp" if new_program[flip][0] == "nop" else "nop"
        result, accumulator = run_program(new_program)
        if result is True:
            print(f"Part 2: {accumulator}")
            break


if __name__ == "__main__":
    main("input.txt")