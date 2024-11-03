package day22

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

type CompassDirection int
type Turn int
type Square int
type Instruction struct {
	Steps int
	Turn  Turn
}

const (
	East CompassDirection = iota
	South
	West
	North
	Open Square = iota
	Solid
	Right Turn = iota
	Left
)

type StartAndEnd struct {
	Start int
	End   int
}

func Day22() error {
	data, err := helpers.ReadFileToRecords("./inputs/22.txt", "\n\n")
	if err != nil {
		return err
	}
	arena := make(map[helpers.Coordinate[int]]Square)
	for y, line := range strings.Split(data[0], "\n") {
		for x, c := range line {
			switch c {
			case '#':
				arena[helpers.Coordinate[int]{X: x, Y: y}] = Solid
			case '.':
				arena[helpers.Coordinate[int]{X: x, Y: y}] = Open
			}
		}
	}

	directions := make([]Instruction, 0)

	steps := 0
	for _, c := range data[1] {
		if c >= '0' && c <= '9' {
			steps = steps*10 + int(c-'0')
		} else if c == 'R' {
			directions = append(directions, Instruction{Steps: steps, Turn: Right})
			steps = 0
		} else if c == 'L' {
			directions = append(directions, Instruction{Steps: steps, Turn: Left})
			steps = 0
		}
	}
	directions = append(directions, Instruction{Steps: steps, Turn: Right})
	fmt.Printf("Part 1: %v\n", part1(directions, arena))
	fmt.Printf("Part 2: %v\n", part2(directions, arena))

	return nil
}

func part1(directions []Instruction, arena map[helpers.Coordinate[int]]Square) int {
	rows := make(map[int]StartAndEnd)
	cols := make(map[int]StartAndEnd)
	for coord := range arena {
		rowDimensions, ok := rows[coord.Y]
		if !ok {
			rows[coord.Y] = StartAndEnd{Start: coord.X, End: coord.X}
		} else if rowDimensions.Start > coord.X {
			rows[coord.Y] = StartAndEnd{Start: coord.X, End: rowDimensions.End}
		} else if rowDimensions.End < coord.X {
			rows[coord.Y] = StartAndEnd{Start: rowDimensions.Start, End: coord.X}
		}
		colDimensions, ok := cols[coord.X]
		if !ok {
			cols[coord.X] = StartAndEnd{Start: coord.Y, End: coord.Y}
		} else if colDimensions.Start > coord.Y {
			cols[coord.X] = StartAndEnd{Start: coord.Y, End: colDimensions.End}
		} else if colDimensions.End < coord.Y {
			cols[coord.X] = StartAndEnd{Start: colDimensions.Start, End: coord.Y}
		}

	}
	currentLocation := helpers.Coordinate[int]{X: rows[0].Start, Y: 0}
	currentFacing := East
	for _, instruction := range directions {
		nextSquare := currentLocation
		steps := instruction.Steps
		for steps > 0 {
			switch currentFacing {
			case North:
				nextSquare.Y -= 1
				if _, ok := arena[nextSquare]; !ok {
					nextSquare.Y = cols[nextSquare.X].End
				}
			case South:
				nextSquare.Y += 1
				if _, ok := arena[nextSquare]; !ok {
					nextSquare.Y = cols[nextSquare.X].Start
				}
			case East:
				nextSquare.X += 1
				if _, ok := arena[nextSquare]; !ok {
					nextSquare.X = rows[nextSquare.Y].Start
				}
			case West:
				nextSquare.X -= 1
				if _, ok := arena[nextSquare]; !ok {
					nextSquare.X = rows[nextSquare.Y].End
				}
			}
			if arena[nextSquare] == Solid {
				break
			}

			currentLocation = nextSquare
			steps--
		}
		switch currentFacing {
		case North:
			if instruction.Turn == Right {
				currentFacing = East
			} else {
				currentFacing = West
			}
		case East:
			if instruction.Turn == Right {
				currentFacing = South
			} else {
				currentFacing = North
			}
		case South:
			if instruction.Turn == Right {
				currentFacing = West
			} else {
				currentFacing = East
			}
		case West:
			if instruction.Turn == Right {
				currentFacing = North
			} else {
				currentFacing = South
			}
		}
	}
	return ((currentLocation.Y + 1) * 1000) + (currentLocation.X+1)*4 + (int(currentFacing)+3)%4
}

func part2(directions []Instruction, arena map[helpers.Coordinate[int]]Square) int {
	/*
	   AB
	   C
	  ED
	  F
	*/
	startX := 65535
	for coord := range arena {
		if coord.X < startX && coord.Y == 0 {
			startX = coord.X
		}
	}
	currentLocation := helpers.Coordinate[int]{X: startX, Y: 0}
	currentFacing := East
	for _, instruction := range directions {
		nextSquare := currentLocation
		nextFacing := currentFacing
		steps := instruction.Steps
		for steps > 0 {
			switch currentFacing {
			case North:
				nextSquare.Y -= 1
				if _, ok := arena[nextSquare]; !ok {
					if nextSquare.X < 50 {
						// E to C
						nextSquare.Y = nextSquare.X + 50
						nextSquare.X = 50
						nextFacing = East
					} else if nextSquare.X < 100 {
						// A to F
						nextSquare.Y = nextSquare.X + 100
						nextSquare.X = 0
						nextFacing = East
					} else {
						// B to F
						nextSquare.X = nextSquare.X - 100
						nextSquare.Y = 199
						nextFacing = North
					}
				}
			case South:
				nextSquare.Y += 1
				if _, ok := arena[nextSquare]; !ok {
					if nextSquare.X < 50 {
						// F to B
						nextSquare.X = nextSquare.X + 100
						nextSquare.Y = 0
						nextFacing = South
					} else if nextSquare.X < 100 {
						// D to F
						nextSquare.Y = nextSquare.X + 100
						nextSquare.X = 49
						nextFacing = West
					} else {
						// B to C
						nextSquare.Y = nextSquare.X - 50
						nextSquare.X = 99
						nextFacing = West
					}
				}
			case East:
				nextSquare.X += 1
				if _, ok := arena[nextSquare]; !ok {
					if nextSquare.Y < 50 {
						// B to D
						nextSquare.X = 99
						nextSquare.Y = 149 - nextSquare.Y
						nextFacing = West
					} else if nextSquare.Y < 100 {
						// C to B
						nextSquare.X = nextSquare.Y + 50
						nextSquare.Y = 49
						nextFacing = North
					} else if nextSquare.Y < 150 {
						// D to B
						nextSquare.X = 149
						nextSquare.Y = 149 - nextSquare.Y
						nextFacing = West
					} else {
						nextSquare.X = nextSquare.Y - 100
						nextSquare.Y = 149
						nextFacing = North
					}
				}
			case West:
				nextSquare.X -= 1
				if _, ok := arena[nextSquare]; !ok {
					if nextSquare.Y < 50 {
						// A to E
						nextSquare.X = 0
						nextSquare.Y = 149 - nextSquare.Y
						nextFacing = East
					} else if nextSquare.Y < 100 {
						// C to E
						nextSquare.X = nextSquare.Y - 50
						nextSquare.Y = 100
						nextFacing = South
					} else if nextSquare.Y < 150 {
						// E to A
						nextSquare.X = 50
						nextSquare.Y = 149 - nextSquare.Y
						nextFacing = East
					} else {
						// F to A
						nextSquare.X = nextSquare.Y - 100
						nextSquare.Y = 0
						nextFacing = South
					}
				}
			}
			if arena[nextSquare] == Solid {
				break
			}

			currentLocation = nextSquare
			currentFacing = nextFacing
			steps--
		}
		switch currentFacing {
		case North:
			if instruction.Turn == Right {
				currentFacing = East
			} else {
				currentFacing = West
			}
		case East:
			if instruction.Turn == Right {
				currentFacing = South
			} else {
				currentFacing = North
			}
		case South:
			if instruction.Turn == Right {
				currentFacing = West
			} else {
				currentFacing = East
			}
		case West:
			if instruction.Turn == Right {
				currentFacing = North
			} else {
				currentFacing = South
			}
		}
	}
	return ((currentLocation.Y + 1) * 1000) + (currentLocation.X+1)*4 + (int(currentFacing)+3)%4
}
