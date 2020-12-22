from collections import deque


def load_decks(filename: str) -> tuple[deque, deque] :
    infile = open(filename).read().split('\n\n')
    deck1 = deque([int(x) for x in infile[0].split('\n')[1:]])
    deck2 = deque([int(x) for x in infile[1].split('\n')[1:]])
    return deck1, deck2


def play_combat(deck1: deque, deck2: deque) -> deque:
    while len(deck1) > 0 and len(deck2) > 0:
        card1, card2 = deck1.popleft(), deck2.popleft()
        if card1 > card2:
            deck1.append(card1)
            deck1.append(card2)
        else:
            deck2.append(card2)
            deck2.append(card1)

    return calculate_score(deck1) if len(deck1) > 0 else 0 - calculate_score(deck2)


def play_recursive_combat(deck1: deque, deck2: deque) -> int:
    already_played = set()
    while len(deck1) > 0 and len(deck2) > 0:
        decks = (calculate_score(deck1), calculate_score(deck2))
        if decks in already_played:
            return calculate_score(deck1)
        already_played.add(decks)
        card1, card2 = deck1.popleft(), deck2.popleft()
        if card1 <= len(deck1) and card2 <= len(deck2):
            if play_recursive_combat(deque(list(deck1)[:card1]), deque(list(deck2)[:card2])) > 0:
                deck1.append(card1)
                deck1.append(card2)
            else:
                deck2.append(card2)
                deck2.append(card1)
        elif card1 > card2:
            deck1.append(card1)
            deck1.append(card2)
        else:
            deck2.append(card2)
            deck2.append(card1)

    return calculate_score(deck1) if len(deck1) > 0 else 0 - calculate_score(deck2)


def calculate_score(deck: deque) -> int:
    score = 0
    for i, val in enumerate(list(deck)[::-1]):
        score += (i+1) * val
    return score


def main(filename):
    print(play_combat(*load_decks(filename)))
    print(play_recursive_combat(*load_decks(filename)))


if __name__ == '__main__':
    main("input.txt")