package main
import (
	"strings"
	"strconv"
	"fmt"
	"io/ioutil"

)

type coordinate struct {
	X int
	Y int
}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
    if err != nil {
        panic(err)
	}
	inputString := string(inputBytes)
	inputNumbers := make(map[int]int)
	for i, asciiNum := range strings.Split(inputString, ",") {
		number, _ := strconv.Atoi(asciiNum)
		inputNumbers[i] = number
	} 

	input := make(chan int)
	output := make(chan int)
	control := make(chan struct{})

	go computeResult(inputNumbers, input, output, control)
	var lines []string
	var neighbourCount [][]int
	var currentPostion coordinate
	scaffoldLine := ""
	for {
		result, ok := <- output

		if !ok {
			break
		}
		if result == 10 {
			lines = append(lines, scaffoldLine)
			scaffoldLine = ""
		} else {

			scaffoldLine = scaffoldLine + string(result)
			if result != '.' && result != '#' {
				currentPostion.X = len(scaffoldLine) -1
				currentPostion.Y = len(lines) -1
			}
		}
	}

	for _, line := range lines {
		fmt.Println(line)
	}
	fours := 0
	maxY :=  len(lines)-1 // there's an extra blank line on the end of the string representation of the track
	for Y := 0; Y < maxY; Y++ {
		var nbrLine []int
		maxX := len(lines[Y])

		for X:= 0; X < maxX; X++ {
			neighbours := getNeighbours(X, Y, maxX-1, maxY-1)
			nbrCount := 0
			if lines[Y][X] == '#' {
				fmt.Println(X, Y, neighbours, maxX, maxY )
				for _, nbr := range neighbours {
					fmt.Printf("(%d,%d) - looking at (%d, %d)\n", X, Y, nbr.X, nbr.Y)
					if lines[nbr.Y][nbr.X] == '#' {
						nbrCount++
					}
				}
			}
			if nbrCount == 4{
				fours = fours + (X*Y)
			}
			nbrLine = append(nbrLine, nbrCount)
		}
		neighbourCount = append(neighbourCount, nbrLine)
	}
	for _, line := range neighbourCount {
		fmt.Println(line)
	}
		for _, line := range lines {
		fmt.Println(line)
	}
	fmt.Println(fours, currentPostion.X, currentPostion.Y)

	var nextSquare coordinate 
	for {
		switch lines[currentPostion.Y][currentPostion.X] {
		case '^':
			nextSquare = coordinate{currentPostion.X,currentPostion.Y-1}
		case '>':
			nextSquare = coordinate{currentPostion.X+1,currentPostion.Y}
		case 'v':
			nextSquare = coordinate{currentPostion.X,currentPostion.Y+1}
		case '<':
			nextSquare = coordinate{currentPostion.X-1,currentPostion.Y}
		}
		if lines[nextSquare.Y][nextSquare.X] != '#' {
			
		}
	}

}

func getNeighbours(X, Y, maxX, maxY int) []coordinate {
	if X == 0 && Y == 0 {
		return []coordinate{{X+1, Y}, {X, Y+1}}
	} else if X == 0 {
		return []coordinate{{X+1, Y},  {X, Y-1}, {X, Y+1}}
	} else if Y == 0 {
		return []coordinate{{X+1, Y},  {X-1, Y}, {X, Y+1}}
	} else if X == maxX && Y == maxX {
		return []coordinate{{X-1, Y}, {X, Y-1}}
	} else if X == maxX  {
		return []coordinate{{X-1, Y}, {X, Y-1}, {X, Y+1}}
	} else if Y == maxY {
		return []coordinate{{X-1, Y}, {X+1, Y}, {X, Y-1}}
	} 
	return []coordinate{{X-1, Y}, {X+1, Y}, {X, Y-1}, {X, Y+1}}
}

func computeResult (inputMap map[int]int, input, output chan int, control chan struct{}) {
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
				inputMap[inputMap[pos+1] + offset[0]] = <-input
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
