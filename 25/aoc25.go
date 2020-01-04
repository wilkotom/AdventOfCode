package main
import (
	"github.com/wilkotom/AoC2019/intcode"
	"fmt"
	"bufio"
	"os"
	"strings"
	// "time"
)


func main() {
	var input string
	reader := bufio.NewReader(os.Stdin)
	intComputer := intcode.StartIntCodeComputer("aoc25.txt")
	outString := ""
	for {
		out, ok := <- intComputer.Output
		outString = outString + string(out)
		// fmt.Print(string(out))
		if strings.HasSuffix(outString, "Command?") {
			fmt.Print(outString," ")
			outString = ""
			input, _  = reader.ReadString('\n')
			for _, c := range input {
				// fmt.Println(int(c))

				intComputer.Input <-int(c)
			}
		}
		if !ok {
			fmt.Println(outString)
			break
		}

	}

	
}
