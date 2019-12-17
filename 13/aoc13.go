package main
import (
	"strings"
	"strconv"
	"fmt"
	"io/ioutil"
	// "bufio"
	// "os"
	// tm "github.com/buger/goterm"
	"time"

)

type coordinates struct {
	X int
	Y int
}

func main() {
	joystick := 0
	grid := make(map[[2]int]int)
	inputBytes, err := ioutil.ReadFile("input.txt")
    if err != nil {
        panic(err)
	}
	blockCount := 0
	inputString := string(inputBytes)
	inputNumbers := make(map[int]int)
	for i, asciiNum := range strings.Split(inputString, ",") {
		number, _ := strconv.Atoi(asciiNum)
		inputNumbers[i] = number
	} 
	maxY := 0
	maxX := 0
	ball := coordinates{0,0}
	bat := coordinates{0,0}
	output := make(chan int)
	control := make(chan struct{})

	go computeResult(inputNumbers, &joystick, output, control, 0)
	for {
		X := <-output
		Y := <-output
		tileID, ok := <-output
		if !ok {
			break
		}
		if maxY < Y {
			maxY = Y
		}
		if maxX < X {
			maxX = X
		}
		grid[[2]int{X,Y}] = tileID
		if X == -1 {
			break
		}
		switch tileID {

			case 2:
				blockCount++
			case 3:
				bat.X = X
				bat.Y = Y
			case 4:
				ball.X = X
				ball.Y = Y
		}

	}


	fmt.Println(blockCount)
	score := 0
	for {
		printGrid(grid,maxX,maxY)
		fmt.Println("Score: ", score)
		fmt.Println("Joystick: ", joystick)
		X := <-output
		Y := <-output
		tileID, ok := <-output
		if X == -1 && Y == 0 {
			score = tileID
		} 
		grid[[2]int{X,Y}] = tileID
		if !ok {
			fmt.Println()
			break
		}

		fmt.Println(X,Y, tileID)
		switch tileID {
			case 3:
				bat.X = X
			case 4:
				ball.X = X
				ball.Y = Y
		}
		if bat.X < ball.X {
			joystick = 1
		} else if bat.X > ball.X {
			joystick = -1
		} else if bat.X == ball.X {
			joystick = 0
		}

	
		fmt.Println("Bat: ", bat)
		fmt.Println("Ball: ", ball)
		fmt.Println("--------------------------------------")
	
	}
	fmt.Println(grid[[2]int{-1,0}])

	fmt.Println(score)
	fmt.Println("Bat: ", bat)
	fmt.Println("Ball: ", ball)
	fmt.Println("--------------------------------------")
}

func computeResult (inputMap map[int]int, input *int, output chan int, control chan struct{}, id int) {
	argLength := map[int]int{
		1: 3,
		2: 3,
		3: 1,
		4: 1,
		5: 3,
		6: 3,
		7: 3,
		8: 3,
		9: 1,
		99: 0,
	  }

	finished := false
	pos := 0
	relativeBase := 0
	for !finished {
		instruction := inputMap[pos] % 100
		if instruction == 99 {
			close(control)
			close(output)
			return 
		}
		parameters := make(map[int]int)
		positionals := inputMap[pos] / 100
		offset := []int{0,0,0}
		for argCount := 0; argCount < argLength[instruction]; argCount++ {

			switch positionals % 10 {
				case 0:
					parameters[argCount] = inputMap[inputMap[pos+argCount+1]]
				case 1:
					parameters[argCount] = inputMap[pos+argCount+1]
				case 2:
					parameters[argCount] =  inputMap[relativeBase + inputMap[pos+argCount+1]]
					offset[argCount] = relativeBase
				default:
					panic("wtf")
			}
			positionals = positionals / 10
		}
		switch instruction {
			case 1:
				inputMap[inputMap[pos+3] + offset[2]] = parameters[0]  + parameters[1]
				pos = pos + 4

			case 2:
				inputMap[inputMap[pos+3] + offset[2]] = parameters[0] * parameters[1]
				pos = pos + 4

			case 3:
				inputMap[inputMap[pos+1] + offset[0]] = *input
				pos = pos + 2

			case 4:
				output <- parameters[0]
				pos = pos + 2

			case 5: 
				if parameters[0] != 0 {
					pos = parameters[1]
				} else {
					pos = pos + 3
				}
			case 6:
				if parameters[0] == 0 {
					pos = parameters[1]
				} else {
					pos = pos + 3
				}
				
			case 7:
				if parameters[0] < parameters[1] {
					inputMap[inputMap[pos+3] + offset[2]] = 1
				} else {
					inputMap[inputMap[pos+3] + offset[2]] = 0
				}
				pos = pos + 4

			case 8:
				if parameters[0] == parameters[1] {

					inputMap[inputMap[pos+3] + offset[2]] = 1
				} else {

					inputMap[inputMap[pos+3] + offset[2]] = 0
				}
				pos = pos + 4
			case 9:
				relativeBase = relativeBase + parameters[0]
				pos = pos + 2
			default:
				panic (fmt.Sprintf("Hit unknown instruction: %d", instruction))
		}
	}
	return 
}

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
	time.Sleep(time.Millisecond * 30 )
}