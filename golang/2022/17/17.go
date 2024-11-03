package day17

import (
	"errors"
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

var rocks = [5][]helpers.Coordinate[int]{
	{{X: 2, Y: 0}, {X: 3, Y: 0}, {X: 4, Y: 0}, {X: 5, Y: 0}},               // horizontal line
	{{X: 3, Y: 0}, {X: 2, Y: 1}, {X: 3, Y: 1}, {X: 4, Y: 1}, {X: 3, Y: 2}}, // cross
	{{X: 2, Y: 0}, {X: 3, Y: 0}, {X: 4, Y: 0}, {X: 4, Y: 1}, {X: 4, Y: 2}}, // L
	{{X: 2, Y: 0}, {X: 2, Y: 1}, {X: 2, Y: 2}, {X: 2, Y: 3}},               // vertical line
	{{X: 2, Y: 0}, {X: 3, Y: 0}, {X: 2, Y: 1}, {X: 3, Y: 1}},               // square
}

func Day17() {

	directions, err := helpers.ReadFileToString("inputs/17.txt")
	if err != nil {
		panic(err)
	}
	//directions := ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
	part1, part2, _ := part1(&directions, 1000000000000)
	fmt.Printf("Part 1: %v\n", part1)
	fmt.Printf("Part 2: %v\n", part2)

}

func part1(directions *string, target int) (int, int, error) {
	directionPointer := 0
	directionString := *directions
	shapePointer := 0
	playingField := make(helpers.Set[helpers.Coordinate[int]])
	maxY := -1
	part1Answer := 0
	firstLine := -1
	firstLineShapePointer := -1
	firstLineDirectionPointer := -1
	var cycleRocks, cycleTime int
	for shapePointer < target {
		shape := make([]helpers.Coordinate[int], len(rocks[shapePointer%len(rocks)]))
		for i := range shape {
			shape[i] = rocks[shapePointer%len(rocks)][i].Add(helpers.Coordinate[int]{X: 0, Y: maxY + 4})
		}

		stuck := false
		var delta helpers.Coordinate[int]
		for !stuck {
			switch directionString[directionPointer%len(directionString)] {
			case '<':
				delta = helpers.Coordinate[int]{X: -1, Y: 0}
			case '>':
				delta = helpers.Coordinate[int]{X: 1, Y: 0}
			default:
				return 0, 0, errors.New("unexpected char in input")
			}

			directionPointer++
			horizontalTranslation := make([]helpers.Coordinate[int], 0, len(shape))
			canMoveHorizontally := true
			for _, point := range shape {
				newPoint := point.Add(delta)
				if newPoint.X < 0 || newPoint.X > 6 || playingField.Contains(newPoint) {
					canMoveHorizontally = false
					break
				}
				horizontalTranslation = append(horizontalTranslation, newPoint)
			}
			if canMoveHorizontally {
				shape = horizontalTranslation
			}

			verticalTranslation := make([]helpers.Coordinate[int], 0, len(shape))
			for _, point := range shape {
				newPoint := point.Add(helpers.Coordinate[int]{X: 0, Y: -1})
				if playingField.Contains(newPoint) || newPoint.Y < 0 {
					stuck = true
					break
				}
				verticalTranslation = append(verticalTranslation, newPoint)
			}
			if stuck {
				for _, point := range shape {
					playingField.Insert(point)
					if point.Y > maxY {
						maxY = point.Y
					}
				}
				shapePointer++
			} else {
				shape = verticalTranslation
			}

		}

		if shapePointer == 2022 {
			part1Answer = maxY + 1
		} else if shapePointer > 2022 {
			lineFound := true
			for x := 0; x < 7; x++ {
				if !playingField.Contains(helpers.Coordinate[int]{X: x, Y: maxY}) {
					lineFound = false
					break
				}
			}
			if lineFound && firstLine == -1 {
				firstLine = maxY
				firstLineShapePointer = shapePointer
				firstLineDirectionPointer = directionPointer
			} else if lineFound && shapePointer%len(rocks) == firstLineShapePointer%len(rocks) &&
				directionPointer%len(directionString) == firstLineDirectionPointer%len(directionString) {
				secondLine := maxY
				cycleTime = shapePointer - firstLineShapePointer
				cycleRocks = (secondLine - firstLine) * ((1000000000000 - firstLineShapePointer) / cycleTime)
				remainingCyles := (1000000000000 - firstLineShapePointer) % cycleTime
				_, startAndEnd, _ := part1(directions, firstLineShapePointer+remainingCyles)
				return part1Answer, startAndEnd + cycleRocks + 1, nil
			}

		}

	}
	return 0, cycleRocks + maxY, nil
}
