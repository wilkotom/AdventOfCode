package main
import (
    "github.com/wilkotom/AoC2019/intcode"
	"fmt"
	"time"

)

type coordinates struct {
	X int
	Y int
}

func main() {
	grid := make(map[[2]int]int)
	blockCount := 0
	ball := coordinates{0,0}
	bat := coordinates{0,0}
	intComputer := intcode.StartIntCodeComputer("input.txt")

	maxX := 0
	maxY := 0
	for {
		printGrid(grid,maxX,maxY)
		X := <- intComputer.Output
		Y := <- intComputer.Output
		tileID, ok := <- intComputer.Output
		grid[[2]int{X,Y}] = tileID
		if maxX < X {
			maxX = X
		}
		if maxY < Y {
			maxY = Y
		}
		if !ok {
			break
		}
		switch tileID {
		case 2:
			blockCount++
		case 3:
			bat.X = X
		case 4:
			ball.X = X
			ball.Y = Y
			if bat.X < ball.X {
				intComputer.Input <- 1
			} else if bat.X > ball.X {
				intComputer.Input <- -1
			} else if bat.X == ball.X {
				intComputer.Input <- 0
			}
		}
	}
	fmt.Println("Score: ",grid[[2]int{-1,0}])
	fmt.Println("Blocks:", blockCount)


}

// func computeResult (inputMap map[int]int, input, output chan int, control chan struct{}, id int) {
// 	argLength := map[int]int{
// 		1: 3,
// 		2: 3,
// 		3: 1,
// 		4: 1,
// 		5: 3,
// 		6: 3,
// 		7: 3,
// 		8: 3,
// 		9: 1,
// 		99: 0,
// 	  }

// 	finished := false
// 	pos := 0
// 	relativeBase := 0
// 	for !finished {
// 		instruction := inputMap[pos] % 100
// 		if instruction == 99 {
// 			close(control)
// 			close(output)
// 			return 
// 		}
// 		parameters := make(map[int]int)
// 		positionals := inputMap[pos] / 100
// 		offset := []int{0,0,0}
// 		for argCount := 0; argCount < argLength[instruction]; argCount++ {

// 			switch positionals % 10 {
// 				case 0:
// 					parameters[argCount] = inputMap[inputMap[pos+argCount+1]]
// 				case 1:
// 					parameters[argCount] = inputMap[pos+argCount+1]
// 				case 2:
// 					parameters[argCount] =  inputMap[relativeBase + inputMap[pos+argCount+1]]
// 					offset[argCount] = relativeBase
// 				default:
// 					panic("wtf")
// 			}
// 			positionals = positionals / 10
// 		}
// 		switch instruction {
// 			case 1:
// 				inputMap[inputMap[pos+3] + offset[2]] = parameters[0]  + parameters[1]
// 				pos = pos + 4

// 			case 2:
// 				inputMap[inputMap[pos+3] + offset[2]] = parameters[0] * parameters[1]
// 				pos = pos + 4

// 			case 3:
// 				inputMap[inputMap[pos+1] + offset[0]] = <- input
// 				pos = pos + 2

// 			case 4:
// 				output <- parameters[0]
// 				pos = pos + 2

// 			case 5: 
// 				if parameters[0] != 0 {
// 					pos = parameters[1]
// 				} else {
// 					pos = pos + 3
// 				}
// 			case 6:
// 				if parameters[0] == 0 {
// 					pos = parameters[1]
// 				} else {
// 					pos = pos + 3
// 				}
				
// 			case 7:
// 				if parameters[0] < parameters[1] {
// 					inputMap[inputMap[pos+3] + offset[2]] = 1
// 				} else {
// 					inputMap[inputMap[pos+3] + offset[2]] = 0
// 				}
// 				pos = pos + 4

// 			case 8:
// 				if parameters[0] == parameters[1] {

// 					inputMap[inputMap[pos+3] + offset[2]] = 1
// 				} else {

// 					inputMap[inputMap[pos+3] + offset[2]] = 0
// 				}
// 				pos = pos + 4
// 			case 9:
// 				relativeBase = relativeBase + parameters[0]
// 				pos = pos + 2
// 			default:
// 				panic (fmt.Sprintf("Hit unknown instruction: %d", instruction))
// 		}
// 	}
// 	return 
// }

func printGrid(grid map[[2]int]int, maxX, maxY int) {
	fmt.Print("\033[H\033[2J")
	fmt.Println("Score:", grid[ [2]int{-1,0} ])
	for Y := 0; Y < maxY+1; Y++ {
		for X := 0; X < maxX+1; X++ {
			switch grid[ [2]int{X,Y}] {
			case 0:
				fmt.Print(" ")
			case 1:
				fmt.Print("X")
			case 2:
				fmt.Print("0")
			case 3: 
				fmt.Print("-")
			case 4:
				fmt.Print(".")
			}
			// fmt.Print(grid[ [2]int{X,Y} ])
		}
		fmt.Println()
	}
	time.Sleep(time.Millisecond * 10 )
}