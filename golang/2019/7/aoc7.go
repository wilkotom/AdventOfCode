package main
import (
	"strings"
	"strconv"
	"fmt"
	// "bufio"
	// "os"
)

func main() {
	inputString := "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
	inputNumbers := []int{}
	for _, asciiNum := range strings.Split(inputString, ",") {
		number, _ := strconv.Atoi(asciiNum)
		inputNumbers = append(inputNumbers,number)
	} 
	stages := []int{5,6,7,8,9}
	permutations := generateInputPermutations(stages)
	bestResult := 0
	bestPermutation := make([]int, len(stages))

	for _, permutation := range permutations {
		inputs := make([]chan int, len(stages))
		controls := make([]chan struct{}, len(stages))
		for i := range inputs {
			inputs[i] = make(chan int)
			controls[i] = make(chan struct{})
		 }
		for i := 0; i < len(inputs) ; i++ {
			localInstrCopy := make([]int, len(inputNumbers))
			copy(localInstrCopy, inputNumbers)
			destination := i +1
			if destination == len(inputs) {
				destination = 0
			}
			go computeResult(localInstrCopy, inputs[i], inputs[destination], controls[i], i)
			inputs[i] <- permutation[i]
		}
		inputs[0] <- 0
		<- controls[0] // if the first processor has closed its control channel, it will no longer consume input. What we have is the final answer
		lastResult := <- inputs[0]
		
		if lastResult > bestResult {
			bestResult = lastResult
			copy(bestPermutation, permutation)
		}
	}
	fmt.Println(bestPermutation, bestResult)
}

func computeResult (inputSlice []int, input, output chan int, control chan struct{}, id int) {
	finished := false
	pos := 0
	for !finished {
		// inputReader := bufio.NewReader(os.Stdin)
		instruction := inputSlice[pos] % 100
		// This is up here just in case 99 is the last instruction in the set - fetching args will give an out-of-bounds error
		if instruction == 99 {
			close(control)
			return 
		}
		parameters := make([]int,3)
		positionals := inputSlice[pos] / 100	
		if positionals % 10 == 0 {
			parameters[0] = inputSlice[inputSlice[pos+1]]
		} else {
			parameters[0] = inputSlice[pos+1]
		}
		// In the case of 2 and 3-argument commands I'm attempting to resolve all args as pointers even if we'll ignore the pointer
		// and use a literal anyway. Occasionally these premature resolutions point outside the program. The size check causes these 
		// bad lookups to be skipped. At this point I don't know how many args I actually need.
		// Will probably need to refactor this to deal with arbitrary numbers of args
		if (positionals /10 )% 10 == 0 && inputSlice[pos+2] < len(inputSlice) { 
			parameters[1] = inputSlice[inputSlice[pos+2]]
		} else {
			parameters[1] = inputSlice[pos+2]
		}
		if (positionals / 100 )%10 == 0 && pos+3 < len(inputSlice) && inputSlice[pos+3] < len(inputSlice) {
			parameters[2] = inputSlice[inputSlice[pos+3]]		
		} else if pos +3 >= len(inputSlice) { // Kludge to get around a one argument opcode being immediately before a 99 at the end of the file.
			parameters[2] = 0
		} else {
			parameters[2] = inputSlice[pos+3]
		}

		switch instruction {
			case 1:
				inputSlice[inputSlice[pos+3]] = parameters[0]  + parameters[1]
				pos = pos + 4

			case 2:
				inputSlice[inputSlice[pos+3]] = parameters[0] * parameters[1]
				pos = pos + 4

			case 3:

				inputSlice[inputSlice[pos+1]] = <-input
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
					inputSlice[inputSlice[pos+3]] = 1
				} else {
					inputSlice[inputSlice[pos+3]] = 0
				}
				pos = pos + 4

			case 8:
				if parameters[0] == parameters[1] {
					inputSlice[inputSlice[pos+3]] = 1
				} else {
					inputSlice[inputSlice[pos+3]] = 0
				}
				pos = pos + 4

			default:
				panic (fmt.Sprintf("Hit unknown instruction: %d", instruction))
		}
	}
	return 
}


func generateInputPermutations(inputs []int) [][]int {
	outputs := [][]int{}
	values := make([]int, len(inputs))
	copy(values, inputs)
	if len(values) == 1 {
		return [][]int{values}
	}
	for i := 0; i < len(values); i++{
		firstPart := make([]int, len(values[:i]))
		secondPart := make([]int, len(values[i+1:]))
		copy(firstPart, values[:i])
		copy(secondPart, values[i+1:])
		for _, perm := range generateInputPermutations(append(secondPart, firstPart...)) {
			outputs = append(outputs, append([]int{values[i]}, perm...))
		}
	}
	
	return outputs
}
