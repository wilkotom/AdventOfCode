package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/wilkotom/AoC2019/intcode"
	// "time"
)

func main() {
	var input string
	reader := bufio.NewReader(os.Stdin)
	intComputer := intcode.StartIntCodeComputer("aoc21.txt")
	outString := ""
	dustCollected := 0
	for {
		out, ok := <-intComputer.Output
		if out < 256 {
			outString = outString + string(out)
		} else {
			dustCollected = out
		}

		if !ok {
			fmt.Println(outString)
			fmt.Println("Dust Collected: ", dustCollected)
			break
		}

		// fmt.Print(string(out))
		if strings.HasSuffix(outString, "instructions:") {
			fmt.Print(outString, " ")
			outString = ""
			instructions := ""
			input, _ = reader.ReadString('\n')
			instructions = instructions + input
			for input != "WALK\n" && input != "RUN\n" {
				input, _ = reader.ReadString('\n')
				instructions = instructions + input
			}

			for _, c := range instructions {
				// fmt.Println(int(c))

				intComputer.Input <- int(c)
			}
		}

	}

}

/*
Part 1:
NOT A J
NOT B T
AND D T
OR T J
NOT C T
AND D T
OR T J
WALK

Part 2:
NOT A J
NOT B T
AND D T
AND H T
OR T J
NOT C T
AND D T
AND H T
OR T J
RUN
*/
