package day02

import (
	"bufio"
	"fmt"
	"os"
)

func Day02() {

	file, _ := os.Open("inputs/02.txt")
	scanner := bufio.NewScanner(file)
	var matches [][]uint8
	for scanner.Scan() {
		line := scanner.Text()
		matches = append(matches, []uint8{line[0], line[2]})
	}
	fmt.Printf("Part 1: %v\n", part1(matches))
	fmt.Printf("Part 2: %v\n", part2(matches))
	defer file.Close()
}

func part2(pairs [][]uint8) uint64 {
	var total uint64
	for _, pair := range pairs {
		switch pair[0] {
		case 'A': //Rock
			switch pair[1] {
			case 'X': // lose - play scissors
				total += 3
			case 'Y': // draw - play rock
				total += 1 + 3
			case 'Z': // win - play paper
				total += 2 + 6
			}
		case 'B': // Paper
			switch pair[1] {
			case 'X':
				total += 1
			case 'Y':
				total += 2 + 3
			case 'Z':
				total += 3 + 6
			}
		case 'C': // Scissors
			switch pair[1] {
			case 'X':
				total += 2
			case 'Y':
				total += 3 + 3
			case 'Z':
				total += 1 + 6
			}
		}
	}
	return total
}

func part1(pairs [][]uint8) uint64 {
	var total uint64
	for _, pair := range pairs {
		switch pair[0] {
		case 'A': //Rock
			switch pair[1] {
			case 'X':
				total += 1 + 3
			case 'Y':
				total += 2 + 6
			case 'Z':
				total += 3
			}
		case 'B': // Paper
			switch pair[1] {
			case 'X':
				total += 1
			case 'Y':
				total += 2 + 3
			case 'Z':
				total += 3 + 6
			}
		case 'C': // Scissors
			switch pair[1] {
			case 'X':
				total += 1 + 6
			case 'Y':
				total += 2
			case 'Z':
				total += 3 + 3
			}
		}
	}
	return total
}
