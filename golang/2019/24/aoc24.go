package main

import (
	"fmt"
	"io/ioutil"
)

type coordinate struct {
	X int
	Y int
	Z int
}

func main() {
	grid := generateGrid("input.txt")
	biodiversityMap := make(map[int]bool)
	gridDiversity := getBiodiversity(grid)
	for !biodiversityMap[gridDiversity] {
		biodiversityMap[gridDiversity] = true
		grid = runGeneration(grid, 1)
		gridDiversity = getBiodiversity(grid)
	}
	fmt.Printf("Part 1: Grid Diversity %d seen twice\n", gridDiversity)
	grid = generateGrid("input.txt")

	for i := 0; i < 200; i++ {
		grid = runGeneration(grid, 2)
	}
	fmt.Printf("Part 2 number of bugs: %d\n", len(grid))
}

func printGrid(grid map[coordinate]bool, gridLevel int) {

	for i := 0; i < 25; i++ {
		if i%5 == 0 {
			fmt.Print("\n")
		}
		if grid[coordinate{i % 5, i / 5, gridLevel}] {
			fmt.Print("#")
		} else {

			fmt.Print(" ")
		}
	}
	fmt.Print("\n")

}

func getBiodiversity(grid map[coordinate]bool) int {
	total := 0
	squareScore := 1
	for i := 0; i < 25; i++ {
		if grid[coordinate{i % 5, i / 5, 0}] {
			total = total + squareScore
		}
		squareScore = squareScore * 2
	}
	return total
}

func generateGrid(filename string) map[coordinate]bool {
	inputBytes, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	grid := make(map[coordinate]bool)
	x := 0
	y := 0
	for _, b := range inputBytes {
		if b == '#' {
			grid[coordinate{x, y, 0}] = true
			x++
		} else if b == '\n' {
			y++
			x = 0
		} else {
			x++
		}
	}
	return grid
}

func runGeneration(grid map[coordinate]bool, questionPart int) map[coordinate]bool {
	possibleNodes := getNeighbours(grid, questionPart)
	neighbourCounts := make(map[coordinate]int)
	for node := range possibleNodes {
		count := 0
		var neighbours []coordinate
		if questionPart == 1 {
			neighbours = getPart1Neighbours(node)
		} else {
			neighbours = getPart2Neighbours(node)
		}
		for _, neighbour := range neighbours {
			if possibleNodes[neighbour] {
				count++
			}
		}
		neighbourCounts[node] = count
	}
	nextGrid := make(map[coordinate]bool)
	for square, count := range neighbourCounts {
		if count == 1 {
			nextGrid[square] = true
		} else if !possibleNodes[square] && count == 2 {
			nextGrid[square] = true
		}
	}
	return nextGrid
}

func getNeighbours(grid map[coordinate]bool, questionPart int) map[coordinate]bool {
	neighbours := make(map[coordinate]bool)
	for square := range grid {
		neighbours[square] = grid[square]
		if questionPart == 1 {
			for _, neighbour := range getPart1Neighbours(square) {
				neighbours[neighbour] = grid[neighbour]
			}
		} else {
			for _, neighbour := range getPart2Neighbours(square) {
				neighbours[neighbour] = grid[neighbour]
			}
		}
	}
	return neighbours
}

func getPart1Neighbours(existingNode coordinate) []coordinate {
	neighbourTransformation := []coordinate{{1, 0, 0}, {0, 1, 0}, {-1, 0, 0}, {0, -1, 0}}
	nodesToConsider := []coordinate{}
	for _, translation := range neighbourTransformation {
		newNode := coordinate{existingNode.X + translation.X, existingNode.Y + translation.Y, 0}
		if newNode.X >= 0 && newNode.X <= 4 && newNode.Y >= 0 && newNode.Y <= 4 {
			nodesToConsider = append(nodesToConsider, newNode)
		}
	}
	return nodesToConsider
}

func getPart2Neighbours(existingNode coordinate) []coordinate {
	neighbourTransformation := []coordinate{{1, 0, 0}, {0, 1, 0}, {-1, 0, 0}, {0, -1, 0}}
	nodesToConsider := []coordinate{}
	for _, translation := range neighbourTransformation {
		newNode := coordinate{existingNode.X + translation.X, existingNode.Y + translation.Y, existingNode.Z}
		if newNode.X < 0 {
			nodesToConsider = append(nodesToConsider, coordinate{1, 2, existingNode.Z - 1})
		} else if newNode.X > 4 {
			nodesToConsider = append(nodesToConsider, coordinate{3, 2, existingNode.Z - 1})
		} else if newNode.Y < 0 {
			nodesToConsider = append(nodesToConsider, coordinate{2, 1, existingNode.Z - 1})
		} else if newNode.Y > 4 {
			nodesToConsider = append(nodesToConsider, coordinate{2, 3, existingNode.Z - 1})
		} else if newNode.X == 2 && newNode.Y == 2 {
			if existingNode.X == 2 && existingNode.Y == 1 {
				for i := 0; i < 5; i++ {
					nodesToConsider = append(nodesToConsider, coordinate{i, 0, existingNode.Z + 1})
				}
			} else if existingNode.X == 2 && existingNode.Y == 3 {
				for i := 0; i < 5; i++ {
					nodesToConsider = append(nodesToConsider, coordinate{i, 4, existingNode.Z + 1})
				}
			} else if existingNode.X == 1 && existingNode.Y == 2 {
				for i := 0; i < 5; i++ {
					nodesToConsider = append(nodesToConsider, coordinate{0, i, existingNode.Z + 1})
				}
			} else {
				for i := 0; i < 5; i++ {
					nodesToConsider = append(nodesToConsider, coordinate{4, i, existingNode.Z + 1})
				}
			}

		} else if newNode.X >= 0 && newNode.X <= 4 && newNode.Y >= 0 && newNode.Y <= 4 {
			nodesToConsider = append(nodesToConsider, newNode)
		}
	}
	return nodesToConsider

}
