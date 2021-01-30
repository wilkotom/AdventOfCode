package main

import (
	"fmt"
	"io/ioutil"
	"math/big"
	"strconv"
	"strings"
)

func newStack(cards []int) {
	for i := 0; i < len(cards)/2; i++ {
		j := len(cards) - i - 1
		cards[i], cards[j] = cards[j], cards[i]
	}
}

func cut(cards []int, position int) {
	if position < 0 {
		position = len(cards) + position
	}
	tempDeck := make([]int, position)
	for i := 0; i < position; i++ {
		tempDeck[i] = cards[i]
	}
	for i := 0; i < len(cards)-position; i++ {
		cards[i] = cards[i+position]
	}
	for i := 0; i < len(tempDeck); i++ {
		cards[len(cards)-position+i] = tempDeck[i]
	}

}

func increment(cards []int, incValue int) {
	tempDeck := make([]int, len(cards))
	for i := 0; i < len(cards); i++ {
		tempDeck[i] = cards[i]
	}
	for i := 0; i < len(cards); i++ {
		cards[(i*incValue)%len(cards)] = tempDeck[i]
	}

}

func part1(filename string, deckSize int, cardNumber int) int {
	cards := make([]int, deckSize)

	for i := 0; i < deckSize; i++ {
		cards[i] = i
	}

	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	for _, line := range strings.Split(string(inputBytes), "\n") {
		instructions := strings.Split(line, " ")
		if instructions[0] == "cut" {
			incValue, _ := strconv.Atoi(instructions[1])
			cut(cards, incValue)
		} else if instructions[2] == "increment" {
			incValue, _ := strconv.Atoi(instructions[3])
			increment(cards, incValue)
		} else {
			newStack(cards)
		}
	}

	for i := 0; i < deckSize; i++ {
		if cards[i] == cardNumber {
			return i
		}
	}
	return -1
}

func collapsePart2Rules(filename string, deckSizeBigInt big.Int) (big.Int, big.Int) {

	deckSize := deckSizeBigInt.Int64()

	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	/*
		Each rule can be reduced to a transformation:
		x => (ax +b) % deckSize
	*/
	var a, b, ad, bd int64
	a = 1
	b = 0
	for _, line := range strings.Split(string(inputBytes), "\n") {
		instructions := strings.Split(line, " ")
		ad = 0
		bd = 0
		if instructions[0] == "cut" {
			ad = 1
			bdi, _ := strconv.Atoi(instructions[1])
			bd = 0 - int64(bdi)
		} else if instructions[2] == "increment" {
			adi, _ := strconv.Atoi(instructions[3])
			ad = int64(adi)
		} else {
			ad = -1
			bd = -1
		}
		// we add deckSize on to guaratee that a and b remain positive
		// Go's modulo operator returns negative modulo where a is negative.
		a = (ad*a + deckSize) % deckSize
		b = (ad*b + bd + deckSize) % deckSize
	}
	return *big.NewInt(a), *big.NewInt(b)
}

func part2(filename string, deckSize big.Int, position int64) {
	/*
		Collapse the rules to a single equation:
		x => (ax +b) % deckSize
	*/

	var a big.Int
	var b big.Int

	a, b = collapsePart2Rules(filename, deckSize)

	/*
		Need to apply the shuffle 101741582076661 times
		Fun with BigInt...
	*/

	numberOfPasses := big.NewInt(101741582076661)

	/*
		After all passes, a = a^(num passes) mod deck size
	*/
	finalA := new(big.Int).Exp(&a, numberOfPasses, &deckSize)

	/*
		After all passes, b = (b * (finalA - 1) * Exp(a , deck size -2, deck size)) mod deck size.
		Use of big numbers means big number libraries. Which makes for unreadable code.

	*/

	finalB := new(big.Int).Mod(
		new(big.Int).Mul(
			new(big.Int).Mul(
				&b,
				new(big.Int).Sub(
					finalA,
					big.NewInt(1))),
			new(big.Int).Exp(
				new(big.Int).Sub(&a, big.NewInt(1)),
				new(big.Int).Sub(&deckSize, big.NewInt(2)),
				&deckSize)),
		&deckSize)

	fmt.Println("Part 2:", new(big.Int).Mod(
		new(big.Int).Mul(
			new(big.Int).Sub(big.NewInt(position), finalB),
			new(big.Int).Exp(
				finalA,
				new(big.Int).Sub(
					&deckSize, big.NewInt(2)),
				&deckSize)),
		&deckSize))

}

func main() {
	fmt.Println("Part 1 answer: ", part1("input.txt", 10007, 2019))
	part2("input.txt", *big.NewInt(119315717514047), 2020)
}
