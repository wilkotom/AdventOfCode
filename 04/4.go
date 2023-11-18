package day04

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day04() {
	lines, err := helpers.ReadFileToLines("04/input.txt")
	if err != nil {
		fmt.Printf("%v", err)
	}
	bounds := make([][2][2]int, 0, len(lines))
	for _, line := range lines {
		var l1, l2, r1, r2 int
		_, err := fmt.Fscanf(strings.NewReader(line), "%d-%d,%d-%d", &l1, &l2, &r1, &r2)
		if err != nil {
			fmt.Printf("%v\n", err)
		}
		pairs := [2][2]int{{l1, l2}, {r1, r2}}
		bounds = append(bounds, pairs)
	}
	fmt.Printf("Part 1: %d\n", part1(bounds))
	fmt.Printf("Part 1: %d\n", part2(bounds))
}

func part1(ranges [][2][2]int) int {
	answer := 0
	for _, pair := range ranges {
		if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) || (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1]) {
			answer++
		}
	}
	return answer
}

func part2(ranges [][2][2]int) int {
	answer := 0
	for _, pair := range ranges {
		if (pair[0][0] <= pair[1][0] && pair[1][0] <= pair[0][1]) || (pair[1][0] <= pair[0][0] && pair[0][0] <= pair[1][1]) {
			answer++
		}
	}
	return answer
}
