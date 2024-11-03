package day10

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

type Instr struct {
	Instruction string
	Value       int
}

func Day10() {
	instructions, err := helpers.ReadFileToLines("inputs/10.txt")
	if err != nil {
		panic(err)
	}
	var instr string
	var value int
	var program []Instr
	for _, line := range instructions {
		_, err := fmt.Fscanf(strings.NewReader(line), "%s %v", &instr, &value)
		if err != nil {
			program = append(program, Instr{"noop", 0})
		} else {
			program = append(program, Instr{"noop", 0})
			program = append(program, Instr{"addx", value})
		}
	}
	fmt.Printf("Part 1: %v\n", part1(program))
}

func part1(program []Instr) int {
	result := 0
	x_register := 1
	difference := 0
	fmt.Print("X")

	for i, instruction := range program {
		if (i+1)%40 == 0 {
			fmt.Println()
		}
		if (i+1-20)%40 == 0 {
			result += x_register * (i + 1)
		}
		x_register += instruction.Value
		difference = x_register - ((i + 1) % 40)
		if -1 <= difference && difference <= 1 {
			fmt.Print("X")
		} else {
			fmt.Print(" ")
		}

	}

	return result
}
