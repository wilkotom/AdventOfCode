package main

import (
	"github.com/wilkotom/AoC2019/intcode"
	"fmt"
	"time"
	"math"
)
const (
	north = 1
	south = 2
	east = 3
	west = 4
)

type location struct {
	X int
	Y int
}

var origin location

func main() {
	origin = location{0,0}
	filename := "input.txt"
	intComputer := intcode.StartIntCodeComputer(filename)

	facing := 1
	mazeMap := make(map[location]int) 
	currentlocation := location{0,0}
	steps := 0
	topLeft := location{0,0}
	bottomRight := location{0,0}
	maxSteps := 0
	for {
		facing = getNextDirection(mazeMap, currentlocation)
		intComputer.Input <- facing
		result := <-intComputer.Output

		// fmt.Println(result, facing)
		if result == 0 {
			switch facing {
			case north:
				mazeMap[location{currentlocation.X, currentlocation.Y-1}] = -1
				if currentlocation.Y -1 < topLeft.Y {
					topLeft.Y = currentlocation.Y-1
				}
			case east:
				mazeMap[location{currentlocation.X +1 , currentlocation.Y}]= -1
				if currentlocation.X +1 > bottomRight.X {
					bottomRight.X = currentlocation.X +1
				}
			case south:
				mazeMap[location{currentlocation.X, currentlocation.Y +1}] = -1
				if currentlocation.Y +1 > bottomRight.Y {
					bottomRight.Y = currentlocation.Y +1
				}
			case west:
				mazeMap[location{currentlocation.X-1, currentlocation.Y}] = -1
				if currentlocation.X - 1 < topLeft.X {
					topLeft.X = currentlocation.X-1
				}
			}
			// fmt.Println(mazeMap)

		} else if result == 1 || result == 2 {
			steps++
			if steps > maxSteps {
				maxSteps = steps
			}
			if result == 2 {
				origin = currentlocation
				resetStepCounts(mazeMap, currentlocation)
				maxSteps = 0	
			}
			switch facing {
			case north:
				currentlocation.Y--
			case east:
				currentlocation.X++
			case south:
				currentlocation.Y++
			case west:
				currentlocation.X--

			}
			if (currentlocation != origin) && (mazeMap[currentlocation] == 0) {
				mazeMap[currentlocation] = steps
			} else {
				steps = mazeMap[currentlocation]
			}

			// fmt.Println("Steps:", steps, "Location: ", currentlocation)
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
	scores[north] = getPathValue(mazeMap, current.X, current.Y-1) // mazeMap[location{current.X, current.Y-1}]
	scores[south] = getPathValue(mazeMap, current.X, current.Y+1) // mazeMap[location{current.X, current.Y+1}]
	scores[east]  = getPathValue(mazeMap, current.X+1, current.Y) // mazeMap[location{current.X+1, current.Y}]
	scores[west]  = getPathValue(mazeMap, current.X-1, current.Y) // mazeMap[location{current.X-1, current.Y}]
	for _, direction := range []int{north, south, east, west} {
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