package day24

import (
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day25() error {
	input, err := helpers.ReadFileToLines("./inputs/25.txt")
	if err != nil {

		return err
	}
	total := 0
	for _, line := range input {
		decimal := snafuToDigital(line)

		total += decimal
	}
	fmt.Printf("Part 1: %v\n", decimalToSnafu(total))
	return nil
}

func snafuToDigital(snafu string) int {
	total := 0
	for _, c := range snafu {
		total *= 5
		switch c {
		case '0':
			continue
		case '1':
			total += 1
		case '2':
			total += 2
		case '-':
			total -= 1
		case '=':
			total -= 2
		}

	}
	return total
}

func decimalToSnafu(number int) string {
	snafuOutput := ""

	for number > 0 {
		remainder := number % 5
		switch remainder {
		case 0:
			snafuOutput = "0" + snafuOutput
		case 1:
			snafuOutput = "1" + snafuOutput
		case 2:
			snafuOutput = "2" + snafuOutput
		case 3:
			snafuOutput = "=" + snafuOutput
			number += 5
		case 4:
			snafuOutput = "-" + snafuOutput
			number += 5
		}
		number /= 5
	}
	return snafuOutput
}
