package day09

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day09() {
	instructions, err := helpers.ReadFileToLines("inputs/09.txt")
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 1: %v\n", solution(&instructions, 2))
	fmt.Printf("Part 2: %v\n", solution(&instructions, 10))

}
func solution(instructions *[]string, ropeLength int) int {
	rope := make([]helpers.Coordinate[int], ropeLength)
	tailVisited := make(helpers.Set[helpers.Coordinate[int]], 0)
	var direction string
	var distance int
	for _, line := range *instructions {

		fmt.Fscanf(strings.NewReader(line), "%s %d", &direction, &distance)
		for i := 0; i < distance; i++ {
			switch direction {
			case "U":
				rope[0] = rope[0].Add(helpers.Coordinate[int]{X: 0, Y: -1})
			case "D":
				rope[0] = rope[0].Add(helpers.Coordinate[int]{X: 0, Y: 1})
			case "L":
				rope[0] = rope[0].Add(helpers.Coordinate[int]{X: -1, Y: 0})
			case "R":
				rope[0] = rope[0].Add(helpers.Coordinate[int]{X: 1, Y: 0})
			default:
				panic("Couldn't understand instruction")
			}
			for j := 1; j < len(rope); j++ {
				neighbours := rope[j-1].ExtendedNeighbours()
				if rope[j] != rope[j-1] && !neighbours.Contains(rope[j]) {
					if rope[j-1].X < rope[j].X {
						rope[j].X -= 1
					} else if rope[j-1].X > rope[j].X {
						rope[j].X += 1
					}
					if rope[j-1].Y < rope[j].Y {
						rope[j].Y -= 1
					} else if rope[j-1].Y > rope[j].Y {
						rope[j].Y += 1
					}
				}
			}
			tailVisited.Insert(rope[ropeLength-1])

		}

	}
	return len(tailVisited)
}
