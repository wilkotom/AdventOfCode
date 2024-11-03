package day05

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/gammazero/deque"
)

func Day05() {

	stacks, instructions := parseFile("05/input.txt")
	part1(stacks, instructions)
	stacks, instructions = parseFile("05/input.txt")
	part2(stacks, instructions)

}

func parseFile(filename string) ([9]deque.Deque[int32], [][3]int) {
	fileContent, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}
	stacks := [9]deque.Deque[int32]{}
	sections := strings.Split(string(fileContent), "\n\n")
	for _, line := range strings.Split(sections[0], "\n") {
		for n, c := range line {
			if c >= 'A' && c <= 'Z' {
				target := (n - 1) / 4
				stacks[target].PushFront(c)
			}
		}
	}
	rawInstrs := strings.Split(sections[1], "\n")
	instructions := make([][3]int, 0, len(rawInstrs))
	for _, line := range rawInstrs {
		var quantity, source, dest int
		_, err := fmt.Fscanf(strings.NewReader(line), "move %d from %d to %d", &quantity, &source, &dest)
		if err != nil {
			log.Fatal(err)
		}
		instr := [3]int{quantity, source - 1, dest - 1}
		instructions = append(instructions, instr)
	}
	return stacks, instructions
}

func part1(stacks [9]deque.Deque[int32], instructions [][3]int) {

	for _, instruction := range instructions {
		for i := 0; i < instruction[0]; i++ {
			o := stacks[instruction[1]].PopBack()
			stacks[instruction[2]].PushBack(o)
		}
	}
	for _, stack := range stacks {
		if stack.Len() > 0 {
			fmt.Printf("%c", stack.Back())
		} else {
			fmt.Print(" ")
		}
	}
	fmt.Println()
}

func part2(stacks [9]deque.Deque[int32], instructions [][3]int) {

	for _, instruction := range instructions {
		tempHold := deque.Deque[int32]{}
		for i := 0; i < instruction[0]; i++ {
			o := stacks[instruction[1]].PopBack()
			tempHold.PushFront(o)
		}
		for tempHold.Len() > 0 {
			stacks[instruction[2]].PushBack(tempHold.PopFront())
		}
	}
	for _, stack := range stacks {
		if stack.Len() > 0 {
			fmt.Printf("%c", stack.Back())
		} else {
			fmt.Print(" ")
		}
	}
	fmt.Println()
}
