package day08

import (
	"fmt"
	"log"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day08() {
	grid, err := helpers.ReadFileToNumberGrid("inputs/08.txt")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%v\n", part1(grid))
	fmt.Printf("%v\n", part2(grid))

}

func part1(grid map[helpers.Coordinate[int]]int) int {
	total := 0
	for k := range grid {
		if isVisible(k, grid) {
			total += 1
		}
	}

	return total
}

func part2(grid map[helpers.Coordinate[int]]int) int {
	bestScore := 0
	for k := range grid {
		treeScore := treeScore(k, grid)
		if treeScore > bestScore {
			bestScore = treeScore
		}
	}

	return bestScore
}

func exploreDirection(loc, direction helpers.Coordinate[int], grid map[helpers.Coordinate[int]]int) bool {
	height := grid[loc]
	target := loc.Add(direction)
	for {
		targetHeight, ok := grid[target]
		if !ok {
			return true
		}
		if targetHeight >= height {
			return false
		}
		target = target.Add(direction)
	}
}

func isVisible(loc helpers.Coordinate[int], grid map[helpers.Coordinate[int]]int) bool {

	return exploreDirection(loc, helpers.Coordinate[int]{X: 0, Y: -1}, grid) ||
		exploreDirection(loc, helpers.Coordinate[int]{X: 0, Y: 1}, grid) ||
		exploreDirection(loc, helpers.Coordinate[int]{X: 1, Y: 0}, grid) ||
		exploreDirection(loc, helpers.Coordinate[int]{X: -1, Y: 0}, grid)

}

func treeScore(loc helpers.Coordinate[int], grid map[helpers.Coordinate[int]]int) int {
	return visibleTreesInDirection(loc, helpers.Coordinate[int]{X: 0, Y: -1}, grid) *
		visibleTreesInDirection(loc, helpers.Coordinate[int]{X: 0, Y: 1}, grid) *
		visibleTreesInDirection(loc, helpers.Coordinate[int]{X: 1, Y: 0}, grid) *
		visibleTreesInDirection(loc, helpers.Coordinate[int]{X: -1, Y: 0}, grid)

}

func visibleTreesInDirection(loc, direction helpers.Coordinate[int], grid map[helpers.Coordinate[int]]int) int {
	height := grid[loc]
	target := loc.Add(direction)
	total := 0
	for {
		targetHeight, ok := grid[target]
		if !ok {
			return total
		}
		if targetHeight >= height {
			return total + 1
		}
		target = target.Add(direction)
		total += 1
	}
}
