package main

import (
	"github.com/wilkotom/AoC2019/intcode"
	"fmt"
	"time"
	"math"
)
const (
	NORTH = 1
	SOUTH = 2
	EAST = 3
	WEST = 4
)

type location struct {
	X int
	Y int
}

var origin location

func main() {
	origin = location{0,0}
	filename := "input.txt"
	input := make(chan int, 1)
	output := make(chan int)
	control := make(chan struct{})
	go intcode.ComputeResult(filename, input, output, control)

	facing := 1
	mazeMap := make(map[location]int) 
	currentlocation := location{0,0}
	steps := 0
	topLeft := location{0,0}
	bottomRight := location{0,0}
	maxSteps := 0
	for {
		facing = getNextDirection(mazeMap, currentlocation)
		input <- facing
		result := <-output
		steps++
		if steps > maxSteps {
			maxSteps = steps
		}
		// fmt.Println(result, facing)
		if result == 0 {
			steps --
			switch facing {
			case NORTH:
				mazeMap[location{currentlocation.X, currentlocation.Y-1}] = -1
				if currentlocation.Y -1 < topLeft.Y {
					topLeft.Y = currentlocation.Y-1
				}
			case EAST:
				mazeMap[location{currentlocation.X +1 , currentlocation.Y}]= -1
				if currentlocation.X +1 > bottomRight.X {
					bottomRight.X = currentlocation.X +1
				}
			case SOUTH:
				mazeMap[location{currentlocation.X, currentlocation.Y +1}] = -1
				if currentlocation.Y +1 > bottomRight.Y {
					bottomRight.Y = currentlocation.Y +1
				}
			case WEST:
				mazeMap[location{currentlocation.X-1, currentlocation.Y}] = -1
				if currentlocation.X - 1 < topLeft.X {
					topLeft.X = currentlocation.X-1
				}
			}
			// fmt.Println(mazeMap)

		} else if result == 1 {
			switch facing {
			case NORTH:
				currentlocation.Y--
			case EAST:
				currentlocation.X++
			case SOUTH:
				currentlocation.Y++
			case WEST:
				currentlocation.X--

			}
			if (currentlocation != origin) && (mazeMap[currentlocation] == 0) {
				mazeMap[currentlocation] = steps
			} else {
				steps = mazeMap[currentlocation]
			}
			// fmt.Println("Steps:", steps, "Location: ", currentlocation)
		} else {
			fmt.Println("Steps:", steps)
			origin = currentlocation
			resetStepCounts(mazeMap, currentlocation)
			maxSteps = 0
			switch facing {
			case NORTH:
				currentlocation.Y--
			case EAST:
				currentlocation.X++
			case SOUTH:
				currentlocation.Y++
			case WEST:
				currentlocation.X--

			}
			printMaze(mazeMap, topLeft, bottomRight, currentlocation, facing, maxSteps)
			time.Sleep(time.Second * 3)
			// break

		}
		if currentlocation.X < topLeft.X {
			topLeft.X = currentlocation.X
		}
		if currentlocation.X > bottomRight.X {
			bottomRight.X = currentlocation.X
		}
		if currentlocation.Y < topLeft.Y {
			topLeft.Y = currentlocation.Y
		} 
		if currentlocation.Y > bottomRight.Y {
			bottomRight.Y = currentlocation.Y
		}
		printMaze(mazeMap, topLeft, bottomRight, currentlocation, facing, maxSteps)
	}

}

func printMaze(mazeMap map[location]int, topLeft, bottomRight, current location, facing, maxSteps int) {
	fmt.Print("\033[H\033[2J")
	fmt.Println("Facing: ", facing, " Location: ", current, "Max Steps: ", maxSteps, "Origin", origin)
	fmt.Println(topLeft, bottomRight)

	for y := topLeft.Y; y < bottomRight.Y+1 ; y ++ {

		for x := topLeft.X; x < bottomRight.X+1; x ++ {
			here := location{x,y} 
			if here == current {
				fmt.Print("  o  ")
				continue
			}
			switch mazeMap[location{x,y}] {
			case 0:
				fmt.Print("     ")
			case -1:
				fmt.Print("█████")
			default:
				// fmt.Print(".")
				fmt.Printf(" %3d ", mazeMap[location{x,y}] )
			}
		}
		fmt.Println()
	}
	// fmt.Println(mazeMap)
	time.Sleep(time.Millisecond * 10 )
}


func getNextDirection(mazeMap map[location]int, current location) int {
	scores := make(map[int]int)
	minScore := math.MaxInt16
	bearing := 0
	scores[NORTH] = getPathValue(mazeMap, current.X, current.Y-1) // mazeMap[location{current.X, current.Y-1}]
	scores[SOUTH] = getPathValue(mazeMap, current.X, current.Y+1) // mazeMap[location{current.X, current.Y+1}]
	scores[EAST]  = getPathValue(mazeMap, current.X+1, current.Y) // mazeMap[location{current.X+1, current.Y}]
	scores[WEST]  = getPathValue(mazeMap, current.X-1, current.Y) // mazeMap[location{current.X-1, current.Y}]
	for _, direction := range []int{NORTH, SOUTH, EAST, WEST} {
		if scores[direction] >= 0 && scores[direction] <= minScore {
			bearing = direction
			minScore = scores[direction]
		}
	}
	fmt.Println(scores)
	fmt.Printf("Chose direction %d with score %d\n", bearing, minScore)
	return bearing

}


func getPathValue(mazeMap map[location]int, X, Y int) int {
	if X == origin.X && Y == origin.Y {
		return math.MaxInt16
	}
	return mazeMap[location{X,Y}]
}

func resetStepCounts(mazeMap map[location]int, newOrigin location) {
	for k := range mazeMap {
		if mazeMap[k] > 0 {
			mazeMap[k] = 0
		}
	}
}