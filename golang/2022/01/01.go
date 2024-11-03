package day01

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day01() {
	file, _ := os.Open("inputs/01.txt")

	defer file.Close()
	calorieCarriers := make([]int, 0)
	scanner := bufio.NewScanner(file)
	total := 0
	for scanner.Scan() {
		calories, _ := strconv.Atoi(scanner.Text())
		total += calories
		if calories == 0 {
			calorieCarriers = append(calorieCarriers, total)
			total = 0
		}
	}
	calorieCarriers = append(calorieCarriers, total)
	sort.Ints(calorieCarriers)
	fmt.Printf("Part 1: %v\n", calorieCarriers[len(calorieCarriers)-1])
	fmt.Printf("Part 2: %v\n", helpers.Sum(calorieCarriers[len(calorieCarriers)-3:]))

}
