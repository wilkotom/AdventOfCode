package main
import (
	"strings"
	"strconv"
	"fmt"
	"io/ioutil"
	// "bufio"
	// "os"
	// tm "github.com/buger/goterm"

)
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
	count :=0 
	// Part 1
	for Y :=0; Y <50; Y++ {
		for X := 0; X <50; X++ {

			instructions :=  make(map[int]int)
			for k, v := range inputNumbers {
				instructions[k] = v
			}
			outval := getResult(X, Y, instructions)
			fmt.Print(outval)
			if outval == 1{
				count++;
			}
		}
		fmt.Println()
	}
	fmt.Println("Part 1 result: ", count)

	// left edge: X*3 >= Y *2
	// right edge: 6*X <= 5Y
	// find the lowest X, Y such that (X*3 >= (Y+100) *2) and (X+100)* 6 <= Y *5
	// one corner: X >= ((Y +100)*2)/3
	// other corner: Y = int((X+100) * float646 / 5

	// Brute force. Slow!
	size := 99
	lastXstart := size
	found := false
	for Y := 0; !found; Y++ {
		last := 0
		// Bit of a kludge here, won't work for inputs where Y = X /n where n > 1 on the left boundary
		// my input has n < 1  
		for X := lastXstart; X < Y ; X++ {
			topRight := getResult(X+size, Y, inputNumbers)
			bottomLeft := getResult(X, Y+size, inputNumbers)
			topLeft := getResult(X, Y, inputNumbers)
			if last != topLeft && last == 1 {
				break
			} else if last != topLeft && last == 0 {
				lastXstart = X
			}
			if topRight == 1 && bottomLeft == 1 {
				fmt.Println(X*10000 + Y)
				found = true
			}
			last = topLeft

		}
	}




}

func getResult (X, Y int, inputNumbers map[int]int) int {

	instructions :=  make(map[int]int)
	for k, v := range inputNumbers {
		instructions[k] = v
	}
	input := make(chan int)
	output := make(chan int)
	control := make(chan struct{})


	go computeResult(instructions, input, output, control)

	input <- X
	input <- Y
	outval := <- output
	return outval


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
