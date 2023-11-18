package day14

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day14() {
	instructions, err := helpers.ReadFileToLines("inputs/14.txt")
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 1: %v\n", (part1(buildMap(instructions))))

	fmt.Printf("Part 2: %v\n", (part2(buildMap(instructions))))

}

func part1(cave map[helpers.Coordinate[int]]rune) int {
	grain := helpers.Coordinate[int]{X: 500, Y: 0}
	maxY := 0
	for k := range cave {
		if k.Y > maxY {
			maxY = k.Y
		}
	}
	for grain.Y <= maxY {
		_, ok := cave[helpers.Coordinate[int]{X: grain.X, Y: grain.Y + 1}]
		if !ok {
			grain.Y++
		} else {
			_, ok := cave[helpers.Coordinate[int]{X: grain.X - 1, Y: grain.Y + 1}]
			if !ok {
				grain.X--
				grain.Y++
			} else {
				_, ok := cave[helpers.Coordinate[int]{X: grain.X + 1, Y: grain.Y + 1}]
				if !ok {
					grain.X++
					grain.Y++
				} else {
					cave[grain] = 'o'
					grain = helpers.Coordinate[int]{X: 500, Y: 0}
				}
			}
		}
	}
	result := 0
	for _, v := range cave {
		if v == 'o' {
			result++
		}
	}
	return result
}

func part2(cave map[helpers.Coordinate[int]]rune) int {
	grain := helpers.Coordinate[int]{X: 500, Y: 0}
	maxY := 0
	for k := range cave {
		if k.Y > maxY {
			maxY = k.Y
		}
	}
	for {
		_, ok := cave[helpers.Coordinate[int]{X: grain.X, Y: grain.Y + 1}]
		if !ok {
			grain.Y++
		} else {
			_, ok := cave[helpers.Coordinate[int]{X: grain.X - 1, Y: grain.Y + 1}]
			if !ok {
				grain.X--
				grain.Y++
			} else {
				_, ok := cave[helpers.Coordinate[int]{X: grain.X + 1, Y: grain.Y + 1}]
				if !ok {
					grain.X++
					grain.Y++
				} else {
					cave[grain] = 'o'
					if grain == (helpers.Coordinate[int]{X: 500, Y: 0}) {
						break
					}
					grain = helpers.Coordinate[int]{X: 500, Y: 0}
				}
			}
		}
		if grain.Y == maxY+1 {
			cave[grain] = 'o'
			grain = helpers.Coordinate[int]{X: 500, Y: 0}
		}

	}

	result := 0
	for _, v := range cave {
		if v == 'o' {
			result++
		}
	}
	return result
}

func buildMap(scans []string) map[helpers.Coordinate[int]]rune {
	caveMap := make(map[helpers.Coordinate[int]]rune)

	for _, line := range scans {
		stringCoordinates := strings.Split(line, " -> ")
		coords := make([]helpers.Coordinate[int], 0)
		var x, y int
		for _, strCoord := range stringCoordinates {
			_, err := fmt.Fscanf(strings.NewReader(strCoord), "%d,%d", &x, &y)
			if err != nil {
				panic(err)
			}
			coords = append(coords, helpers.Coordinate[int]{X: x, Y: y})
		}
		start := coords[0]
		for _, next := range coords[1:] {
			if start.X == next.X {
				if start.Y < next.Y {
					for i := start.Y; i <= next.Y; i++ {
						caveMap[helpers.Coordinate[int]{X: start.X, Y: i}] = '#'
					}
				} else {
					for i := next.Y; i <= start.Y; i++ {
						caveMap[helpers.Coordinate[int]{X: start.X, Y: i}] = '#'
					}
				}
			} else {
				if start.X < next.X {
					for i := start.X; i <= next.X; i++ {
						caveMap[helpers.Coordinate[int]{X: i, Y: start.Y}] = '#'
					}
				} else {
					for i := next.X; i <= start.X; i++ {
						caveMap[helpers.Coordinate[int]{X: i, Y: start.Y}] = '#'
					}
				}
			}
			start = next
		}

	}
	return caveMap
}
