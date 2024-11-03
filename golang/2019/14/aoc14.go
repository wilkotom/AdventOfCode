package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type reagent struct {
	name     string
	quantity int
}

type reaction struct {
	quantity int
	name     string
	reagents []reagent
}

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)
	results := make(map[string]reaction)
	requirements := make(map[string]int)
	satisfied := make(map[string]int)

	for scanner.Scan() {
		reactionParts := strings.Split(scanner.Text(), " => ")
		result := strings.Split(reactionParts[1], " ")
		reagents := strings.Split(reactionParts[0], ", ")
		parsedReagents := []reagent{}
		for _, ingredient := range reagents {
			recipe := strings.Split(ingredient, " ")
			quantity, _ := strconv.Atoi(recipe[0])

			parsedReagents = append(parsedReagents, reagent{recipe[1], quantity})
		}
		resultQuantity, _ := strconv.Atoi(result[0])
		results[result[1]] = reaction{resultQuantity, result[1], parsedReagents}
		requirements[result[1]] = 0
		satisfied[result[1]] = 0

	}
	var oreUsed int
	quantity := 1
	oreUsed, satisfied = makeFuel(satisfied, results, quantity)

	fmt.Printf("Part 1: Need %d Ore to make %d unit of fuel\n", oreUsed, quantity)
	target := 1000000000000
	startingPoint := target / oreUsed
	increment := target / 10000000

	for oreUsed < target {
		for ingredient := range satisfied {
			satisfied[ingredient] = 0
		}
		oreUsed, _ = makeFuel(satisfied, results, startingPoint)
		if oreUsed > target && increment != 1 {
			startingPoint = startingPoint - increment
			oreUsed, _ = makeFuel(satisfied, results, startingPoint-increment)
			increment = increment / 10
		} else {
			startingPoint += increment
		}
	}
	oreUsed, _ = makeFuel(satisfied, results, startingPoint-2)
	fmt.Printf("Part 2: Made %d fuel with %d ore\n", startingPoint-2, oreUsed)

}

func makeFuel(satisfied map[string]int, reactions map[string]reaction, fuelRequired int) (int, map[string]int) {
	requirements := make(map[string]int)
	requirements["FUEL"] = fuelRequired
	oreRequired := 0
	reagentsRequired := true
	for reagentsRequired {
		reagentsRequired = false
		for result := range requirements {
			if requirements[result] > satisfied[result] {
				needed := requirements[result] - satisfied[result]
				numReactions := int(math.Ceil(float64(needed) / float64(reactions[result].quantity)))
				for n := range reactions[result].reagents {
					if reactions[result].reagents[n].name == "ORE" {
						oreRequired = oreRequired + (reactions[result].reagents[n].quantity * numReactions)
						reagentsRequired = true
					} else {
						reagentsRequired = true
						requirements[reactions[result].reagents[n].name] = requirements[reactions[result].reagents[n].name] + (reactions[result].reagents[n].quantity * numReactions)
					}
				}
				satisfied[result] = satisfied[result] + (reactions[result].quantity * numReactions)
			}
		}
	}
	satisfied["ORE"] = satisfied["ORE"] - oreRequired

	for ingredient := range requirements {
		satisfied[ingredient] = satisfied[ingredient] - requirements[ingredient]
		requirements[ingredient] = 0
	}
	return oreRequired, satisfied

}
