def main():
    generations = 50000000000 -1
    mappings = set()
    start, transitions = open("input.txt").read().split('\n\n')
    for mapping in transitions.split('\n'):
        state, result = mapping.split(' => ')
        if result == '#':
            mappings.add(state)
    state = start.split(': ')[1]
    pots = set()
    seen = set()
    for num, contents in enumerate(state):
        if contents == '#':
            pots.add(num)
    print(mappings)
    print(''.join(['#' if x in pots else '.' for x in range(min(pots), max(pots)+1)]))
    for i in range(generations):
        new_pots = set()
        for j in range(min(pots)-4, max(pots)+5):
            surrounds = ''.join(['#' if x in pots else '.' for x in range(j-2, j+3)])
            if surrounds in mappings:
                new_pots.add(j)
        # print(pots,new_pots,i)
        pots = new_pots
        rendered = ''.join(['#' if x in pots else '.' for x in range(min(pots), max(pots)+1)])
        if rendered in seen:
            print(f"Equilibrium at generation {i}. First pot is {min(pots)}")
            break
        seen.add(rendered)
        print(i, sum(pots), rendered)
        # print(sum(pots), sorted(list(pots)))
    print(sum([x-i + generations for x in pots]))
    print(sum(pots))


if __name__ == '__main__':
    main()