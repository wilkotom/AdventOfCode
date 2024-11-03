package day19

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

type MaterialsBill struct {
	Ore      int
	Clay     int
	Obsidian int
}

type BluePrint struct {
	Ore             MaterialsBill
	Clay            MaterialsBill
	Obsidian        MaterialsBill
	Geode           MaterialsBill
	BluePrintNumber int
}

type ManufacturingState struct {
	OreRobots          int
	ClayRobots         int
	ObsidianRobots     int
	GeodeRobots        int
	MaterialsAvailable MaterialsBill
	GeodesCracked      int
	TimeElapsed        int
}

func Day19() {
	blueprints, err := readInputData("./inputs/19.txt")
	if err != nil {
		panic(err)
	}
	part1total := 0
	for _, blueprint := range blueprints {
		part1total += blueprint.BluePrintNumber * bestGeodesforBluePrint(blueprint, 24)
	}
	fmt.Printf("Part 1: %v\n", part1total)
	part2total := 1
	for _, blueprint := range blueprints[0:3] {
		part2total *= bestGeodesforBluePrint(blueprint, 32)
	}
	fmt.Printf("Part 1: %v\n", part2total)
}

func readInputData(filename string) ([]BluePrint, error) {

	lines, err := helpers.ReadFileToLines(filename)
	if err != nil {
		return nil, err
	}
	startingStates := make([]BluePrint, 0)
	for _, line := range lines {
		startingState := BluePrint{}
		_, err := fmt.Fscanf(strings.NewReader(line), "Blueprint %d: Each ore robot costs %d ore. Each clay robot costs %d ore. Each obsidian robot costs %d ore and %d clay. Each geode robot costs %d ore and %d obsidian.",
			&startingState.BluePrintNumber, &startingState.Ore.Ore, &startingState.Clay.Ore, &startingState.Obsidian.Ore, &startingState.Obsidian.Clay, &startingState.Geode.Ore, &startingState.Geode.Obsidian)
		if err != nil {
			return nil, err
		}
		startingStates = append(startingStates, startingState)
	}
	return startingStates, nil
}

func bestGeodesforBluePrint(blueprint BluePrint, targetTime int) int {
	seenStates := make(helpers.Set[ManufacturingState])
	bestScore := 0
	maxOreCost := blueprint.Ore.Ore
	if blueprint.Clay.Ore > maxOreCost {
		maxOreCost = blueprint.Clay.Ore
	}
	if blueprint.Obsidian.Ore > maxOreCost {
		maxOreCost = blueprint.Obsidian.Ore
	}
	if blueprint.Geode.Ore > maxOreCost {
		maxOreCost = blueprint.Geode.Ore
	}

	nextStates := []ManufacturingState{{TimeElapsed: 0, OreRobots: 1}}
	mostGeodes := 0
	for len(nextStates) > 0 {
		currentState := nextStates[0]
		nextStates = nextStates[1:]
		// fmt.Println(currentState)
		cacheableState := ManufacturingState{
			MaterialsAvailable: currentState.MaterialsAvailable,
			GeodesCracked:      currentState.GeodesCracked,
			TimeElapsed:        0,
			OreRobots:          currentState.OreRobots,
			ClayRobots:         currentState.ClayRobots,
			ObsidianRobots:     currentState.ObsidianRobots,
			GeodeRobots:        currentState.GeodeRobots,
		}
		if seenStates.Contains(cacheableState) || currentState.GeodesCracked < mostGeodes-1 {
			continue
		}
		if currentState.GeodesCracked > mostGeodes {
			mostGeodes = currentState.GeodesCracked
		}
		if currentState.TimeElapsed == targetTime {
			if bestScore < currentState.GeodesCracked {
				bestScore = currentState.GeodesCracked
			}
			continue
		}
		seenStates.Insert(cacheableState)
		// If we can build a geode robot, we should,
		if currentState.MaterialsAvailable.Ore >= blueprint.Geode.Ore && currentState.MaterialsAvailable.Obsidian >= blueprint.Geode.Obsidian {
			newMaterialsAvailable := MaterialsBill{
				Ore:      currentState.OreRobots + currentState.MaterialsAvailable.Ore - blueprint.Geode.Ore,
				Clay:     currentState.ClayRobots + currentState.MaterialsAvailable.Clay,
				Obsidian: currentState.ObsidianRobots + currentState.MaterialsAvailable.Obsidian - blueprint.Geode.Obsidian,
			}
			nextState := ManufacturingState{
				MaterialsAvailable: newMaterialsAvailable,
				GeodesCracked:      currentState.GeodesCracked + currentState.GeodeRobots,
				TimeElapsed:        currentState.TimeElapsed + 1,
				OreRobots:          currentState.OreRobots,
				ClayRobots:         currentState.ClayRobots,
				ObsidianRobots:     currentState.ObsidianRobots,
				GeodeRobots:        currentState.GeodeRobots + 1,
			}
			nextStates = append(nextStates, nextState)
		} else {
			// Only make an Ore robot if we're making less than the max that can be consumed in one turn
			if currentState.MaterialsAvailable.Ore >= blueprint.Ore.Ore && currentState.OreRobots < maxOreCost {
				newMaterialsAvailable := MaterialsBill{
					Ore:      currentState.OreRobots + currentState.MaterialsAvailable.Ore - blueprint.Ore.Ore,
					Clay:     currentState.ClayRobots + currentState.MaterialsAvailable.Clay,
					Obsidian: currentState.ObsidianRobots + currentState.MaterialsAvailable.Obsidian,
				}
				nextState := ManufacturingState{
					MaterialsAvailable: newMaterialsAvailable,
					GeodesCracked:      currentState.GeodesCracked + currentState.GeodeRobots,
					TimeElapsed:        currentState.TimeElapsed + 1,
					OreRobots:          currentState.OreRobots + 1,
					ClayRobots:         currentState.ClayRobots,
					ObsidianRobots:     currentState.ObsidianRobots,
					GeodeRobots:        currentState.GeodeRobots,
				}
				nextStates = append(nextStates, nextState)

			}

			if currentState.MaterialsAvailable.Ore >= blueprint.Clay.Ore && currentState.ClayRobots < blueprint.Obsidian.Clay {
				newMaterialsAvailable := MaterialsBill{
					Ore:      currentState.OreRobots + currentState.MaterialsAvailable.Ore - blueprint.Clay.Ore,
					Clay:     currentState.ClayRobots + currentState.MaterialsAvailable.Clay,
					Obsidian: currentState.ObsidianRobots + currentState.MaterialsAvailable.Obsidian,
				}
				nextState := ManufacturingState{
					MaterialsAvailable: newMaterialsAvailable,
					GeodesCracked:      currentState.GeodesCracked + currentState.GeodeRobots,
					TimeElapsed:        currentState.TimeElapsed + 1,
					OreRobots:          currentState.OreRobots,
					ClayRobots:         currentState.ClayRobots + 1,
					ObsidianRobots:     currentState.ObsidianRobots,
					GeodeRobots:        currentState.GeodeRobots,
				}
				nextStates = append(nextStates, nextState)

			}

			if currentState.MaterialsAvailable.Ore >= blueprint.Obsidian.Ore && currentState.MaterialsAvailable.Clay >= blueprint.Obsidian.Clay {
				newMaterialsAvailable := MaterialsBill{
					Ore:      currentState.OreRobots + currentState.MaterialsAvailable.Ore - blueprint.Obsidian.Ore,
					Clay:     currentState.ClayRobots + currentState.MaterialsAvailable.Clay - blueprint.Obsidian.Clay,
					Obsidian: currentState.ObsidianRobots + currentState.MaterialsAvailable.Obsidian,
				}
				nextState := ManufacturingState{
					MaterialsAvailable: newMaterialsAvailable,
					GeodesCracked:      currentState.GeodesCracked + currentState.GeodeRobots,
					TimeElapsed:        currentState.TimeElapsed + 1,
					OreRobots:          currentState.OreRobots,
					ClayRobots:         currentState.ClayRobots,
					ObsidianRobots:     currentState.ObsidianRobots + 1,
					GeodeRobots:        currentState.GeodeRobots,
				}
				nextStates = append(nextStates, nextState)

			}
			newMaterialsAvailable := MaterialsBill{
				Ore:      currentState.OreRobots + currentState.MaterialsAvailable.Ore,
				Clay:     currentState.ClayRobots + currentState.MaterialsAvailable.Clay,
				Obsidian: currentState.ObsidianRobots + currentState.MaterialsAvailable.Obsidian,
			}

			nextState := ManufacturingState{
				MaterialsAvailable: newMaterialsAvailable,
				GeodesCracked:      currentState.GeodesCracked + currentState.GeodeRobots,
				TimeElapsed:        currentState.TimeElapsed + 1,
				OreRobots:          currentState.OreRobots,
				ClayRobots:         currentState.ClayRobots,
				ObsidianRobots:     currentState.ObsidianRobots,
				GeodeRobots:        currentState.GeodeRobots,
			}
			nextStates = append(nextStates, nextState)
		}
	}
	fmt.Println(bestScore)
	return bestScore
}
