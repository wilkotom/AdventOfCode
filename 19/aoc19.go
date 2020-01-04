package main
import (
	"github.com/wilkotom/AoC2019/intcode"
	"fmt"
	// "bufio"
	// "os"
	// tm "github.com/buger/goterm"

)
func main() {
	inputFile := "input.txt"
	count :=0 
	// Part 1
	for Y :=0; Y <50; Y++ {
		for X := 0; X <50; X++ {
			outval := getResult(X, Y, inputFile)
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
	// other corner: Y = int((X+100) * 6 / 5

	// Brute force. Slow!
	size := 99
	lastXstart := size
	found := false
	for Y := 0; !found; Y++ {
		last := 0
		// Bit of a kludge here, won't work for inputs where Y = X /n where n > 1 on the left boundary
		// my input has n < 1  
		for X := lastXstart; X < Y ; X++ {
			topRight := getResult(X+size, Y, inputFile)
			bottomLeft := getResult(X, Y+size, inputFile)
			topLeft := getResult(X, Y, inputFile)
			if last != topLeft && last == 1 {
				break
			} else if topLeft == 1 && (topRight == 0){ 
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

func getResult (X, Y int, inputFile string) int {
	intComputer := intcode.StartIntCodeComputer(inputFile)
	intComputer.Input <- X
	intComputer.Input <- Y
	outval := <- intComputer.Output
	return outval
}

