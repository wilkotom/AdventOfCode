package day23

import (
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day23() error {
	input, err := helpers.ReadFileToLines("./inputs/23.txt")
	if err != nil {

		return err
	}
	boardState := make(helpers.Set[helpers.Coordinate[int]])
	for y, line := range input {
		for x, c := range line {
			if c == '#' {
				boardState.Insert(helpers.Coordinate[int]{X: x, Y: y})
			}
		}
	}
	turn := 0
	for {
		nextBoardState := iterateBoard(boardState, turn)
		turn++
		if turn == 10 {
			fmt.Printf("Part 1: %v\n", part1(nextBoardState))
		}
		if len(nextBoardState.Difference(boardState)) == 0 {
			fmt.Printf("Part 2: %v\n", turn)
			break
		}
		boardState = nextBoardState
	}
	return nil
}
func part1(board helpers.Set[helpers.Coordinate[int]]) int {
	minX := 0
	minY := 0
	maxX := 0
	maxY := 0
	for elf := range board {
		if elf.X < minX {
			minX = elf.X
		}
		if elf.X > maxX {
			maxX = elf.X
		}
		if elf.Y < minY {
			minY = elf.Y
		}
		if elf.Y > maxY {
			maxY = elf.Y
		}
	}
	return ((maxX - minX + 1) * (maxY - minY + 1)) - len(board)
}

func iterateBoard(board helpers.Set[helpers.Coordinate[int]], turn int) helpers.Set[helpers.Coordinate[int]] {

	directions := [4][3]helpers.Coordinate[int]{
		{{X: -1, Y: -1}, {X: 0, Y: -1}, {X: 1, Y: -1}},
		{{X: -1, Y: 1}, {X: 0, Y: 1}, {X: 1, Y: 1}},
		{{X: -1, Y: -1}, {X: -1, Y: 0}, {X: -1, Y: 1}},
		{{X: 1, Y: -1}, {X: 1, Y: 0}, {X: 1, Y: 1}}}

	targetSquaresCount := make(map[helpers.Coordinate[int]]int)
	nextSquare := make(map[helpers.Coordinate[int]]helpers.Coordinate[int])

	for elf := range board {
		mustMove := false
		nextSquare[elf] = elf
		for _, square := range elf.ExtendedNeighbours() {
			if board.Contains(square) {
				mustMove = true
				break
			}
		}
		if mustMove {
			for i := 0; i < 4; i++ {
				direction := (i + turn) % 4
				canChooseDirection := true
				for _, square := range directions[direction] {
					if board.Contains(elf.Add(square)) {
						canChooseDirection = false
						break
					}
				}
				if canChooseDirection {
					nextSquare[elf] = elf.Add(directions[direction][1])
					targetSquaresCount[elf.Add(directions[direction][1])]++
					break
				}
			}

		} else {
			targetSquaresCount[elf]++
		}
	}
	nextBoard := make(helpers.Set[helpers.Coordinate[int]])
	for current, next := range nextSquare {
		if targetSquaresCount[next] == 1 {
			nextBoard.Insert(next)
		} else {
			nextBoard.Insert(current)
		}
	}
	return nextBoard

}
