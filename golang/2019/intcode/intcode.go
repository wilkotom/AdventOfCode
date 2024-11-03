package intcode

import (
	"fmt"
	"strings"
	"io/ioutil"
	"strconv"
)

// Computer describes the available channels for communicating with an intcode computer
type Computer struct {
	filename string;
	Input chan int;
	Output chan int;
	Control chan struct{};
}

// StartIntCodeComputer starts an Intcode computer using the supplied filename for its instructions
func StartIntCodeComputer(filename string) Computer {
	Input :=  make(chan int,1024)
	Output := make(chan int,1024)
	Control := make(chan struct{})
	computer := Computer{filename, Input, Output, Control}
	go computer.ComputeResult()
	return computer

}

func readInstructions (filename string) map[int]int {
	inputBytes, err := ioutil.ReadFile(filename)
    if err != nil {
        panic(err)
	}
	inputString := string(inputBytes)
	inputNumbers := make(map[int]int)
	for i, asciiNum := range strings.Split(inputString, ",") {
		number, _ := strconv.Atoi(asciiNum)
		inputNumbers[i] = number
	} 
	return inputNumbers
}

// ComputeResult creates an intcode computer running the instructions in the input file
// input, output and "control" (has it finished) are handled by way of channels
func (computer *Computer) ComputeResult () {

	inputMap := readInstructions(computer.filename)
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
			close(computer.Control)
			close(computer.Output)
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
				inputMap[inputMap[pos+1] + offset[0]] = <- computer.Input
				pos = pos + 2

			case 4:
				computer.Output <- parameters[0]
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
