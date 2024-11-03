package day21

import (
	"fmt"
	"strconv"

	"github.com/wilkotom/AoC2022/helpers"
)

type Operation int

const (
	Add Operation = iota
	Subtract
	Multiply
	Divide
)

type Monkey struct {
	LeftMonkey  string
	RightMonkey string
	Value       int
	Operation   Operation
}

func Day21() error {
	lines, err := helpers.ReadFileToLines("inputs/21.txt")
	if err != nil {
		return err
	}
	monkeys := make(map[string]Monkey)
	for _, line := range lines {
		nextMonkey := Monkey{}
		monkeyName := line[0:4]
		nextMonkey.Value, err = strconv.Atoi(line[6:])
		if err != nil {
			nextMonkey.LeftMonkey = line[6:10]
			nextMonkey.RightMonkey = line[13:]
			switch line[11] {
			case '+':
				nextMonkey.Operation = Add
			case '-':
				nextMonkey.Operation = Subtract
			case '*':
				nextMonkey.Operation = Multiply
			case '/':
				nextMonkey.Operation = Divide
			default:
				panic(fmt.Sprintf("Culd not parse %c", line[11]))
			}
		}
		monkeys[monkeyName] = nextMonkey

	}
	fmt.Printf("Part 1: %v\n", part1("root", monkeys))
	fmt.Printf("Part 2: %v\n", part2("root", monkeys))

	return nil
}

func part1(monkeyName string, monkeys map[string]Monkey) int {
	currentMonkey := monkeys[monkeyName]
	if currentMonkey.Value != 0 {
		return currentMonkey.Value
	}
	switch currentMonkey.Operation {
	case Add:
		return part1(currentMonkey.LeftMonkey, monkeys) + part1(currentMonkey.RightMonkey, monkeys)
	case Subtract:
		return part1(currentMonkey.LeftMonkey, monkeys) - part1(currentMonkey.RightMonkey, monkeys)
	case Multiply:
		return part1(currentMonkey.LeftMonkey, monkeys) * part1(currentMonkey.RightMonkey, monkeys)
	case Divide:
		return part1(currentMonkey.LeftMonkey, monkeys) / part1(currentMonkey.RightMonkey, monkeys)
	}
	return 0
}

func part2(monkeyName string, monkeys map[string]Monkey) int {
	if containsHuman(monkeys[monkeyName].LeftMonkey, monkeys) {
		return findHumanValue(monkeys[monkeyName].LeftMonkey, part1(monkeys[monkeyName].RightMonkey, monkeys), monkeys)
	} else {
		return findHumanValue(monkeys[monkeyName].RightMonkey, part1(monkeys[monkeyName].LeftMonkey, monkeys), monkeys)
	}
}

func containsHuman(monkeyName string, monkeys map[string]Monkey) bool {
	if monkeyName == "humn" {
		return true
	} else if monkeys[monkeyName].Value != 0 {
		return false
	} else {
		return containsHuman(monkeys[monkeyName].LeftMonkey, monkeys) || containsHuman(monkeys[monkeyName].RightMonkey, monkeys)
	}
}

func findHumanValue(monkeyName string, desired int, monkeys map[string]Monkey) int {
	if monkeyName == "humn" {
		return desired
	}
	if containsHuman(monkeys[monkeyName].LeftMonkey, monkeys) {
		rightResult := part1(monkeys[monkeyName].RightMonkey, monkeys)
		switch monkeys[monkeyName].Operation {
		case Add:
			return findHumanValue(monkeys[monkeyName].LeftMonkey, desired-rightResult, monkeys)
		case Subtract:
			return findHumanValue(monkeys[monkeyName].LeftMonkey, desired+rightResult, monkeys)
		case Multiply:
			return findHumanValue(monkeys[monkeyName].LeftMonkey, desired/rightResult, monkeys)
		case Divide:
			return findHumanValue(monkeys[monkeyName].LeftMonkey, desired*rightResult, monkeys)
		}
	} else {
		leftResult := part1(monkeys[monkeyName].LeftMonkey, monkeys)
		switch monkeys[monkeyName].Operation {
		case Add:
			return findHumanValue(monkeys[monkeyName].RightMonkey, desired-leftResult, monkeys)
		case Subtract:
			return findHumanValue(monkeys[monkeyName].RightMonkey, leftResult-desired, monkeys)
		case Multiply:
			return findHumanValue(monkeys[monkeyName].RightMonkey, desired/leftResult, monkeys)
		case Divide:
			return findHumanValue(monkeys[monkeyName].RightMonkey, desired*leftResult, monkeys)
		}
	}
	return 0
}
