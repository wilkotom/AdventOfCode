package day03

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day03() {

	lines, err := helpers.ReadFileToLines("inputs/03.txt")
	if err != nil {
		fmt.Printf("%v", err)
	}
	result, err := part1(lines)
	if err != nil {
		fmt.Printf("%v", err)
	}
	fmt.Printf("Part 1: %v\n", result)
	result, err = part2(lines)

	if err != nil {
		fmt.Printf("%v", err)
	}
	fmt.Printf("Part 2: %v\n", result)

}

func part1(lines []string) (uint32, error) {
	total := uint32(0)
	for _, line := range lines {
		left := helpers.CreateSet(strings.Split(line[:len(line)/2], ""))
		right := helpers.CreateSet(strings.Split(line[len(line)/2:], ""))

		intersection := left.Intersection(right)
		if len(intersection) > 1 {
			return 0, fmt.Errorf("found multiple matches in line %s", line)
		}
		for k := range intersection {
			total += uint32(helpers.LetterScore(k[0]))
		}
	}
	return total, nil
}

func part2(lines []string) (uint32, error) {
	total := uint32(0)
	for i := 0; i < len(lines); i += 3 {
		left := helpers.CreateSet(strings.Split(lines[i], ""))
		mid := helpers.CreateSet(strings.Split(lines[i+1], ""))
		right := helpers.CreateSet(strings.Split(lines[i+2], ""))
		intersection := left.Intersection(right).Intersection(mid)
		if len(intersection) > 1 {
			return 0, fmt.Errorf("found multiple matches in lines:\n%v", lines[i:i+2])
		}
		for k := range intersection {
			total += uint32(helpers.LetterScore(k[0]))
		}

	}
	return total, nil
}
