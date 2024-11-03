package day12

import (
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

type nextLocation struct {
	location        helpers.Coordinate[int]
	distanceCovered int
}

func Day12() {
	areaMap, err := helpers.ReadFileToRuneGrid("inputs/12.txt")
	if err != nil {
		panic(err)
	}
	var startPosition, finishPosition helpers.Coordinate[int]
	for loc, label := range areaMap {
		if label == 'S' {
			startPosition = loc
		} else if label == 'E' {
			finishPosition = loc
		}
	}
	areaMap[startPosition] = 'a'
	areaMap[finishPosition] = 'z'
	fmt.Printf("Part 1: %v\n", shortestPath(startPosition, finishPosition, areaMap))
	fmt.Printf("Part 2: %v\n", shortestPathDown(startPosition, finishPosition, areaMap))

}

func shortestPath(startPostion, finishPosition helpers.Coordinate[int], areaMap map[helpers.Coordinate[int]]rune) int {
	visited := make(helpers.Set[helpers.Coordinate[int]])
	nextLocations := make([]nextLocation, 0)
	startingPoint := nextLocation{startPostion, 0}
	nextLocations = append(nextLocations, startingPoint)
	for len(nextLocations) > 0 {
		nextPosition := nextLocations[0]
		nextLocations = nextLocations[1:]

		if !visited.Contains(nextPosition.location) {
			visited.Insert(nextPosition.location)
			if nextPosition.location == finishPosition {
				return nextPosition.distanceCovered
			} else {
				currentHeight := areaMap[nextPosition.location]
				for _, neighbour := range nextPosition.location.Neighbours() {
					neighbourHeight, ok := areaMap[neighbour]
					if ok && neighbourHeight <= currentHeight+1 {
						nextLocations = append(nextLocations, nextLocation{neighbour, nextPosition.distanceCovered + 1})
					}
				}
			}
		}

	}
	return 0
}

func shortestPathDown(startPostion, finishPosition helpers.Coordinate[int], areaMap map[helpers.Coordinate[int]]rune) int {
	visited := make(helpers.Set[helpers.Coordinate[int]])
	nextLocations := make([]nextLocation, 0)
	startingPoint := nextLocation{finishPosition, 0}
	finishHeight := areaMap[startPostion]
	nextLocations = append(nextLocations, startingPoint)
	for len(nextLocations) > 0 {
		nextPosition := nextLocations[0]
		nextLocations = nextLocations[1:]

		if !visited.Contains(nextPosition.location) {
			visited.Insert(nextPosition.location)
			if areaMap[nextPosition.location] == finishHeight {
				return nextPosition.distanceCovered
			} else {
				currentHeight := areaMap[nextPosition.location]
				for _, neighbour := range nextPosition.location.Neighbours() {
					neighbourHeight, ok := areaMap[neighbour]
					if ok && neighbourHeight+1 >= currentHeight {
						nextLocations = append(nextLocations, nextLocation{neighbour, nextPosition.distanceCovered + 1})
					}
				}
			}
		}

	}
	return 0
}
