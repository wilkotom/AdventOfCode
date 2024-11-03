package day24

import (
	"container/heap"
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

type CompassDirection int

const (
	North CompassDirection = iota
	South
	East
	West
)

type LocationState struct {
	elapsedTime int
	location    helpers.Coordinate[int]
}

func Day24() error {
	input, err := helpers.ReadFileToLines("./inputs/24.txt")
	if err != nil {

		return err
	}
	winds := [4]map[int][]int{
		make(map[int][]int),
		make(map[int][]int),
		make(map[int][]int),
		make(map[int][]int)}
	bottomLeft := helpers.Coordinate[int]{}
	for y, line := range input[1:] {
		if y > bottomLeft.Y {
			bottomLeft.Y = y
		}
		for x, c := range line[1:] {
			if x > bottomLeft.X {
				bottomLeft.X = x
			}
			switch c {
			case '^':
				winds[North][x] = append(winds[North][x], y)
			case 'v':
				winds[South][x] = append(winds[South][x], y)
			case '>':
				winds[East][y] = append(winds[East][y], x)
			case '<':
				winds[West][y] = append(winds[West][y], x)
			}
		}
	}
	bottomLeft.X--
	bottomLeft.Y--
	part1 := crossValley(&winds, helpers.Coordinate[int]{X: 0, Y: -1}, helpers.Coordinate[int]{X: bottomLeft.X, Y: bottomLeft.Y + 1}, bottomLeft, 0)
	returnTrip := crossValley(&winds, helpers.Coordinate[int]{X: bottomLeft.X, Y: bottomLeft.Y + 1}, helpers.Coordinate[int]{X: 0, Y: -1}, bottomLeft, part1)
	part2 := crossValley(&winds, helpers.Coordinate[int]{X: 0, Y: -1}, helpers.Coordinate[int]{X: bottomLeft.X, Y: bottomLeft.Y + 1}, bottomLeft, returnTrip)
	fmt.Printf("Part 1: %v\nPart 2: %v\n", part1, part2)
	return nil
}

func crossValley(winds *[4]map[int][]int, start, end, bottomLeft helpers.Coordinate[int], startTime int) int {

	nextMoves := make(helpers.MinHeap[LocationState], 0)
	heap.Init(&nextMoves)
	startingState := helpers.NewPrioritisedItem(0, LocationState{elapsedTime: startTime, location: start})
	heap.Push(&nextMoves, &startingState)

	seenStates := make(helpers.Set[LocationState])

nextMove:
	for len(nextMoves) > 0 {
		currentState := heap.Pop(&nextMoves).(*helpers.PrioritisedItem[LocationState])
		if seenStates.Contains(currentState.GameState) {
			continue
		}
		seenStates.Insert(currentState.GameState)
		if currentState.GameState.location == end {
			return currentState.GameState.elapsedTime
		}
		if currentState.GameState.location != start && (currentState.GameState.location.X < 0 || currentState.GameState.location.X > bottomLeft.X || currentState.GameState.location.Y < 0 || currentState.GameState.location.Y > bottomLeft.Y) {
			continue
		}

		for _, wind := range winds[South][currentState.GameState.location.X] {
			if ((wind + currentState.GameState.elapsedTime) % (bottomLeft.Y + 1)) == currentState.GameState.location.Y {
				continue nextMove
			}
		}
		for _, wind := range winds[North][currentState.GameState.location.X] {
			if (((wind-currentState.GameState.elapsedTime)%(bottomLeft.Y+1))+bottomLeft.Y+1)%(bottomLeft.Y+1) == currentState.GameState.location.Y {
				continue nextMove
			}
		}
		for _, wind := range winds[East][currentState.GameState.location.Y] {
			if (wind+currentState.GameState.elapsedTime)%(bottomLeft.X+1) == currentState.GameState.location.X {
				continue nextMove
			}
		}
		for _, wind := range winds[West][currentState.GameState.location.Y] {
			if (((wind-currentState.GameState.elapsedTime)%(bottomLeft.X+1))+bottomLeft.X+1)%(bottomLeft.X+1) == currentState.GameState.location.X {
				continue nextMove
			}
		}
		// A* algorithm, weighted toward minimising distance to goal
		for _, neighbour := range currentState.GameState.location.Neighbours() {
			nextMove := helpers.NewPrioritisedItem(currentState.GameState.elapsedTime+1+(2*neighbour.ManhattanDistance(end)), LocationState{elapsedTime: currentState.GameState.elapsedTime + 1, location: neighbour})
			heap.Push(&nextMoves, &nextMove)
		}
		nextMove := helpers.NewPrioritisedItem(currentState.GameState.elapsedTime+1+(2*currentState.GameState.location.ManhattanDistance(end)), LocationState{elapsedTime: currentState.GameState.elapsedTime + 1, location: helpers.Coordinate[int]{X: currentState.GameState.location.X, Y: currentState.GameState.location.Y}})
		heap.Push(&nextMoves, &nextMove)
	}
	return 0
}
