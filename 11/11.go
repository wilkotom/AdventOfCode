package eleven

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

type Monkey struct {
	Id               int
	Items            []int64
	Transformation   func(int64) int64
	TestDivisor      int64
	TrueDestination  int
	FalseDestination int
	InspectionCount  int
}

func main() {
	monkeys := readMonkeysFromFile("11/input.txt")
	fmt.Printf("Part 1: %v\n", part1(monkeys))
	monkeys = readMonkeysFromFile("11/input.txt")
	fmt.Printf("Part 1: %v\n", part2(monkeys))
}

func part1(monkeys []*Monkey) int {
	for i := 0; i < 20; i++ {
		for _, monkey := range monkeys {
			for _, item := range monkey.Items {
				worryLevel := monkey.Transformation(item)
				worryLevel /= 3
				if worryLevel%monkey.TestDivisor == 0 {
					monkeys[monkey.TrueDestination].Items = append(monkeys[monkey.TrueDestination].Items, worryLevel)
				} else {
					monkeys[monkey.FalseDestination].Items = append(monkeys[monkey.FalseDestination].Items, worryLevel)
				}
			}
			monkey.InspectionCount += len(monkey.Items)
			monkey.Items = make([]int64, 0)
		}
	}
	var max, second int
	for _, monkey := range monkeys {
		if monkey.InspectionCount > max {
			second = max
			max = monkey.InspectionCount
		} else if monkey.InspectionCount > second {
			second = monkey.InspectionCount
		}
	}
	return max * second
}
func part2(monkeys []*Monkey) int {

	commonMonkeyFactor := int64(1)

	for _, monkey := range monkeys {
		commonMonkeyFactor *= monkey.TestDivisor
	}

	for i := 0; i < 10000; i++ {
		for _, monkey := range monkeys {
			for _, item := range monkey.Items {
				worryLevel := monkey.Transformation(item)
				worryLevel %= commonMonkeyFactor
				if worryLevel%monkey.TestDivisor == 0 {
					monkeys[monkey.TrueDestination].Items = append(monkeys[monkey.TrueDestination].Items, worryLevel)
				} else {
					monkeys[monkey.FalseDestination].Items = append(monkeys[monkey.FalseDestination].Items, worryLevel)
				}
			}
			monkey.InspectionCount += len(monkey.Items)
			monkey.Items = make([]int64, 0)
		}
	}
	var max, second int
	for _, monkey := range monkeys {
		if monkey.InspectionCount > max {
			second = max
			max = monkey.InspectionCount
		} else if monkey.InspectionCount > second {
			second = monkey.InspectionCount
		}
	}
	return max * second
}

func readMonkeysFromFile(filename string) []*Monkey {
	monkey_descriptions, err := helpers.ReadFileToRecords(filename, "\n\n")
	if err != nil {
		panic(err)
	}
	var truedestination, falsedestination int
	var transformation func(int64) int64
	var testdivisor int64
	var monkeys []*Monkey

	for id, monkey_description := range monkey_descriptions {
		var items_list []int64

		lines := strings.Split(monkey_description, "\n")
		for _, item := range strings.Split(lines[1][18:], ", ") {
			val, err := strconv.ParseInt(item, 10, 64)
			if err == nil {
				items_list = append(items_list, val)
			} else {
				panic(err)
			}

		}
		if lines[2][23] == '*' {
			if lines[2][25:] == "old" {
				transformation = func(a int64) int64 { return a * a }
			} else {
				val, err := strconv.ParseInt(lines[2][25:], 10, 64)
				if err != nil {
					panic(err)
				} else {
					transformation = func(a int64) int64 { return val * a }
				}
			}
		} else {
			val, err := strconv.ParseInt(lines[2][25:], 10, 64)
			if err != nil {
				panic(err)
			} else {
				transformation = func(a int64) int64 { return val + a }
			}
		}
		testdivisor, err = strconv.ParseInt(lines[3][21:], 10, 64)
		if err != nil {
			panic(err)
		}
		truedestination, err = strconv.Atoi(lines[4][29:])
		if err != nil {
			panic(err)
		}
		falsedestination, err = strconv.Atoi(lines[5][30:])
		if err != nil {
			panic(err)
		}
		monkeys = append(monkeys, &Monkey{id, items_list, transformation, testdivisor, truedestination, falsedestination, 0})
	}
	return monkeys
}
