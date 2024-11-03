package main
import (
	"strings"
	"strconv"
	"fmt"
	"bufio"
	"os"
)

func main() {
	inputString := "3,225,1,225,6,6,1100,1,238,225,104,0,1101,86,8,225,1101,82,69,225,101,36,65,224,1001,224,-106,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,102,52,148,224,101,-1144,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1102,70,45,225,1002,143,48,224,1001,224,-1344,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,1101,69,75,225,1001,18,85,224,1001,224,-154,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1101,15,59,225,1102,67,42,224,101,-2814,224,224,4,224,1002,223,8,223,101,3,224,224,1,223,224,223,1101,28,63,225,1101,45,22,225,1101,90,16,225,2,152,92,224,1001,224,-1200,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,1101,45,28,224,1001,224,-73,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1,14,118,224,101,-67,224,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,677,224,102,2,223,223,1005,224,329,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,344,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,359,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,374,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,389,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,1108,226,677,224,102,2,223,223,1006,224,434,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1108,226,226,224,1002,223,2,223,1005,224,479,1001,223,1,223,1007,226,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1007,226,226,224,102,2,223,223,1005,224,509,101,1,223,223,107,677,226,224,1002,223,2,223,1006,224,524,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,539,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,554,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,569,101,1,223,223,108,677,226,224,1002,223,2,223,1006,224,584,101,1,223,223,108,226,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1107,226,226,224,102,2,223,223,1006,224,614,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,629,1001,223,1,223,107,226,226,224,102,2,223,223,1005,224,644,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,659,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,674,101,1,223,223,4,223,99,226"
	//inputString := "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
	//inputString := "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
	inputNumbers := []int{}
	for _, asciiNum := range strings.Split(inputString, ",") {
		number, _ := strconv.Atoi(asciiNum)
		inputNumbers = append(inputNumbers,number)
	}
	computeResult(inputNumbers)
}

func computeResult (inputSlice []int) int {
	finished := false
	pos := 0
	for !finished {
		inputReader := bufio.NewReader(os.Stdin)
		instruction := inputSlice[pos] % 100
		// This is up here just in case 99 is the last instruction in the set - fetching args will give an out-of-bounds error
		if instruction == 99 {
			return inputSlice[0]
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
		if (positionals / 100 )%10 == 0 && inputSlice[pos+3] < len(inputSlice) {
			parameters[2] = inputSlice[inputSlice[pos+3]]		
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
				fmt.Print("Input Requested: ")
				text, _ := inputReader.ReadString('\n')
				num, err := strconv.Atoi(strings.TrimSpace(text))
				if err != nil {
					panic(err)
				}
				inputSlice[inputSlice[pos+1]] = num
				pos = pos + 2

			case 4:
				fmt.Printf("Output: %d\n", parameters[0])
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
	return inputSlice[0]
}