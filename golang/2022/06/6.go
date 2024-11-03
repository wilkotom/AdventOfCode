package day06

import (
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day06() {
	lines, err := helpers.ReadFileToLines("inputs/06.txt")
	if err != nil {
		fmt.Printf("%v\n", err)
	}
	fmt.Printf("Part 1: %v\n", solution(lines[0], 4))
	fmt.Printf("Part 2: %v\n", solution(lines[0], 14))

}

func solution(line string, length int) int {
	position := 0
	for {
		lineWindow := line[position : position+length]
		symbols := helpers.CreateSet[uint8]([]uint8(lineWindow))
		if len(symbols) == length {
			return position + length
		}
		position++
	}

}
