package main
import (
	"fmt"
)

func main() {
	start := 256310
	end := 732736
	validNumbers := 0
	for testnum := start; testnum <= end; testnum++ {
		num := testnum
		var digits []int
		for num != 0 {
			digit := num % 10
			digits = append(digits, digit)
			num = num / 10
		}
		if ascDigits(digits) && noMorethanTwoAdjacent(digits) {
			validNumbers++
		}
	}
	fmt.Printf("%d\n", validNumbers)
}

func adjDigits(num []int) bool {
	// fmt.Println("Num: ",num)
	for pos:=0; pos < len(num) -1; pos++ {
		// fmt.Printf("Pos: %d", pos)
		if num[pos] == num[pos+1] {
			return true

		}
	}
	return false
}

func ascDigits(num []int) bool {
	for pos:=0; pos < len(num) -1; pos++ {
		if num[pos + 1 ] > num[pos] {
			return false
		}
	}
	return true
}

func noMorethanTwoAdjacent (num []int) bool {
	num = append([]int{-1}, num...)
	num = append(num,-1)
	for pos:=0; pos < len(num) -3; pos++ {
		// fmt.Printf("Pos: %d", pos)
		if num[pos] != num[pos+1] && num[pos+1] == num[pos+2] && num[pos+2] != num[pos+3] {
			return true

		}
	}
	return false
}