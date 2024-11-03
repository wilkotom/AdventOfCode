package main
import (
	"strconv"
	"fmt"
	"os"
	"bufio"
)

func main() {
	filename := "weights.txt"
	file, _ := os.Open(filename)

	defer file.Close()
	total := 0
	scanner := bufio.NewScanner(file)
    for scanner.Scan() {
		mass, _ := strconv.Atoi(scanner.Text())
		total = total + fuel(mass)
	}
	fmt.Println(total)
}

func fuel ( mass int ) int {
	fm := (mass / 3) - 2 
	required := 0
	if fm > 0 {
		required = fm + fuel(fm)
	}
	return required
}