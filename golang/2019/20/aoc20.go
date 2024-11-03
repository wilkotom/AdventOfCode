package main

import (
	"container/heap"
	"container/list"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type coordinate struct {
	X int
	Y int
}

type distance struct {
	location coordinate
	distance int
}

type unvisitedNode struct {
	tentativeDistance int
	name              string
	index             int
}

type priorityQueue []*unvisitedNode

func (q priorityQueue) Len() int {
	return len(q)
}

func (q priorityQueue) Less(i int, j int) bool {
	return q[i].tentativeDistance < q[j].tentativeDistance
}

func (q priorityQueue) Swap(i int, j int) {
	q[i], q[j] = q[j], q[i]
	q[i].index = i
	q[j].index = j
}

func (q *priorityQueue) Push(x interface{}) {
	n := len(*q)
	node := x.(*unvisitedNode)
	node.index = n
	*q = append(*q, node)
}

func (q *priorityQueue) Pop() interface{} {
	old := *q
	currentLen := len(old)
	val := old[currentLen-1]
	old[currentLen-1] = nil
	val.index = -1
	*q = old[:currentLen-1]
	return val

}

func main() {
	mazeMap, locations := readMaze("input.txt")
	distanceMap := getDistances(mazeMap, locations)
	fmt.Println("Part 1 Distance: ", getShortestDistancePart1(distanceMap, "AAO", "ZZO"))
	fmt.Println("Part 2 Distance: ", getShortestDistancePart2(distanceMap, "AAO", "ZZO"))
}

func getShortestDistancePart2(distanceMap map[string]map[string]int, startPoint string, endPoint string) int {
	startPoint = startPoint + "0"
	endPoint = endPoint + "0"

	tentativeDistance := make(map[string]int)
	visited := make(map[string]bool)

	// Same approach as before, except that we append the current depth level to the node name
	for node := range distanceMap {
		nodeName := fmt.Sprintf("%s0", node)
		tentativeDistance[nodeName] = 9999999999999

	}

	tentativeDistance[startPoint] = 0
	descended := make(map[int]bool)
	descended[0] = true
	var nextNodes priorityQueue
	heap.Init(&nextNodes)
	heap.Push(&nextNodes, &unvisitedNode{tentativeDistance: 0, name: startPoint})

	for nextNodes.Len() > 0 && !visited[endPoint] {
		currentNode := heap.Pop(&nextNodes).(*unvisitedNode)
		currentNodeName := currentNode.name
		currentLevel, _ := strconv.Atoi(currentNodeName[3:])
		if !visited[currentNodeName] {
			for nextNode, nextDist := range distanceMap[currentNodeName[:3]] {
				nextNode = nextNode[:3] + fmt.Sprintf("%d", currentLevel)

				// Moving between levels, increment or decrement the level identifier
				nextLevel := currentLevel
				if currentNodeName[:2] == nextNode[:2] {
					if currentNodeName[2] == 'I' {
						nextLevel++
					} else {
						nextLevel--
					}
					nextNode = nextNode[:3] + fmt.Sprintf("%d", nextLevel)
				}
				// if we've not been to a level before, add tentative distances for all the nodes in it.
				// We can't visit levels below 0.
				if nextLevel > 0 && !descended[nextLevel] {
					for node := range distanceMap {
						nodeName := node + nextNode[3:]
						tentativeDistance[nodeName] = 9999999999999
					}
					descended[nextLevel] = true
				}
				if nextLevel >= 0 {
					if nextDist+tentativeDistance[currentNodeName] < tentativeDistance[nextNode] {
						tentativeDistance[nextNode] = nextDist + tentativeDistance[currentNodeName]
					}
					heap.Push(&nextNodes, &unvisitedNode{tentativeDistance: tentativeDistance[nextNode], name: nextNode})
					visited[currentNodeName] = true

				}

			}
		}
		visited[currentNodeName] = true
	}
	return tentativeDistance[endPoint]
}

func getShortestDistancePart1(distanceMap map[string]map[string]int, startPoint string, endPoint string) int {
	// Djikstra's algorithm.

	tentativeDistance := make(map[string]int)
	visited := make(map[string]bool)
	for node := range distanceMap {
		tentativeDistance[node] = 9999999999999
	}

	tentativeDistance[startPoint] = 0
	var nextNodes priorityQueue
	heap.Init(&nextNodes)
	heap.Push(&nextNodes, &unvisitedNode{tentativeDistance: 0, name: startPoint})
	for nextNodes.Len() > 0 && !visited[endPoint] {
		currentNode := heap.Pop(&nextNodes).(*unvisitedNode)
		currentNodeName := currentNode.name
		for nextNode, nextDist := range distanceMap[currentNodeName] {
			if !visited[nextNode] {
				if tentativeDistance[nextNode] > nextDist+tentativeDistance[currentNodeName] {
					tentativeDistance[nextNode] = nextDist + tentativeDistance[currentNodeName]
				}
				heap.Push(&nextNodes, &unvisitedNode{tentativeDistance: tentativeDistance[nextNode], name: nextNode})

			}
		}
		visited[currentNodeName] = true
	}

	return tentativeDistance[endPoint]
}

func getDistances(mazeMap map[coordinate]string, locations map[string]coordinate) map[string]map[string]int {
	allDistances := make(map[string]map[string]int)
	directions := []coordinate{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}
	for location := range locations {
		distanceMapping := make(map[string]int)
		distances := make(map[coordinate]int)
		evalSquares := list.New()
		evalSquares.PushBack(distance{locations[location], 0})
		for evalSquares.Len() > 0 {
			evalSquare := evalSquares.Front()
			evalSquares.Remove(evalSquare)
			loc := evalSquare.Value.(distance)
			distances[loc.location] = loc.distance
			for _, direction := range directions {
				_, nextSquare := mazeMap[coordinate{loc.location.X + direction.X, loc.location.Y + direction.Y}]
				_, alreadyVisited := distances[coordinate{loc.location.X + direction.X, loc.location.Y + direction.Y}]
				if !alreadyVisited && nextSquare {
					evalSquares.PushBack(distance{coordinate{loc.location.X + direction.X, loc.location.Y + direction.Y}, loc.distance + 1})
				}
			}
		}
		for dest := range distances {
			if len(mazeMap[dest]) == 3 {
				distanceMapping[mazeMap[dest]] = distances[dest]
			}
		}
		if strings.HasSuffix(location, "I") {
			distanceMapping[strings.TrimSuffix(location, "I")+"O"] = 1
		} else {
			distanceMapping[strings.TrimSuffix(location, "O")+"I"] = 1

		}
		delete(distanceMapping, location)
		if location == "AAO" {
			delete(distanceMapping, "AAI")
		} else if location == "ZZO" {
			delete(distanceMapping, "ZZI")
		}

		allDistances[location] = distanceMapping

	}
	return allDistances
}

func renderMaze(mazeMap map[coordinate]string) {
	for y := 2; y < 120; y++ {
		for x := 2; x < 120; x++ {
			square, present := mazeMap[coordinate{x, y}]
			if !present || present && len(square) == 1 && square != "." {
				fmt.Print(("██"))
			} else {
				if square == "." {
					fmt.Print("  ")
				} else {
					fmt.Print(square[:2])
				}
			}
		}
	}
	fmt.Printf("\n")
}

func readMaze(filename string) (map[coordinate]string, map[string]coordinate) {
	inputBytes, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	locations := make(map[string][]coordinate)
	mazeMap := make(map[coordinate]string)
	x := 0
	y := 0

	for _, b := range inputBytes {
		if b == '\n' {
			y++
			x = 0
		} else {
			if b == '.' {
				mazeMap[coordinate{x, y}] = "."
			} else if b != '#' && b != ' ' {
				mazeMap[coordinate{x, y}] = string(b)
				previous := mazeMap[coordinate{x - 1, y}]
				if len(previous) > 0 && previous != "." {
					twoLeft := mazeMap[coordinate{x - 2, y}]
					if twoLeft == "." {
						locations[previous+string(b)] = append(locations[previous+string(b)], coordinate{x - 2, y})
					} else {

						locations[previous+string(b)] = append(locations[previous+string(b)], coordinate{x + 1, y})

					}
				}
				previous = mazeMap[coordinate{x, y - 1}]
				if len(previous) > 0 && previous != "." {
					twoAbove := mazeMap[coordinate{x, y - 2}]
					if twoAbove == "." {
						locations[previous+string(b)] = append(locations[previous+string(b)], coordinate{x, y - 2})
					} else {
						locations[previous+string(b)] = append(locations[previous+string(b)], coordinate{x, y + 1})

					}
				}
			}
			x++
		}
	}
	maxX := 0
	maxY := 0
	for _, coords := range locations {
		for _, loc := range coords {
			if loc.X > maxX {
				maxX = loc.X
			}
			if loc.Y > maxY {
				maxY = loc.Y
			}

		}
	}
	finalLocations := make(map[string]coordinate)
	for location, coords := range locations {
		for _, square := range coords {
			if len(location) == 2 {
				if square.X == maxX || square.X == 2 || square.Y == maxY || square.Y == 2 {
					mazeMap[square] = location + "O"
					finalLocations[location+"O"] = square
				} else {
					mazeMap[square] = location + "I"
					finalLocations[location+"I"] = square
				}
			}
		}
	}
	return mazeMap, finalLocations

}
