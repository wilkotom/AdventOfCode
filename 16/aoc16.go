package main

import (
	"fmt"
	"io/ioutil"
)

func main() {
	filename := "input.txt"
	inputBytes, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}

	numbers := []int{}
	for i := 0; i < 1; i++ {
		for _, character := range inputBytes {
			numbers = append(numbers, int(character)-48)
		}
	}

	basePattern := []int{0, 1, 0, -1}

	for i := 0; i < 100; i++ {
		result := []int{}
		for pos := range numbers {
			total := 0
			for spos, source := range numbers {
				multiplier := basePattern[((spos+1)/(pos+1))%4]
				total = total + source*multiplier
			}
			result = append(result, abs(total)%10)
		}
		numbers = result

	}
	fmt.Println("Part 1 answer: ", numbers[0:8])

	numbers = []int{}
	for j := 0; j < 10000; j++ {
		for i := 0; i < 1; i++ {
			for _, character := range inputBytes {
				numbers = append(numbers, int(character)-48)
			}
		}
	}

	offset := 0
	for _, digit := range numbers[0:7] {
		offset = offset * 10
		offset = offset + digit
	}

	numbers = numbers[offset:]
	for i := 0; i < 100; i++ {

		for j := len(numbers) - 2; j >= 0; j-- {
			// Generate next stage
			numbers[j] = (numbers[j] + numbers[j+1]) % 10
		}
		// fmt.Println(numbers)
	}
	fmt.Println("Part 1 answer: ", numbers[0:8])

}

func abs(num int) int {
	if num < 0 {
		return -num
	}
	return num
}
