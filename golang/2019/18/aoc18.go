package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"sort"
	"strings"
	"time"
)

type coordinate struct {
	X int
	Y int
}

type reachable struct {
	distance   int
	behindDoor []string
}

func main() {
	filename := "input.txt"
	part2(filename)
}

func part1(filename string) {

	mazeMap, locations := generateMazeMap(filename)
	distances := make(map[string]map[string]reachable)
	for location := range locations {
		distances[location] = findDestinations(location, mazeMap, locations)
	}

	startingPoint := "0"
	visited := []string{startingPoint}
	unvisited := []string{}
	for place := range distances {
		if !sliceContains(visited, place) {
			unvisited = append(unvisited, place)
		}

	}
	shortestPath := getShortestPath(visited, unvisited, distances, make(map[string]int))
	fmt.Println("Part 1", shortestPath)

}

func part2(filename string) {

	mazeMap, locations := generateMazeMap(filename)
	distances := make(map[string]map[string]reachable)
	for location := range locations {
		distances[location] = findDestinations(location, mazeMap, locations)
	}
	visited := []string{"1", "2", "3", "0"}
	unvisited := []string{}
	for place := range distances {
		if !sliceContains(visited, place) {
			unvisited = append(unvisited, place)
		}

	}
	robotLocation := make(map[string]string)
	for i := 0; i < 4; i++ {
		robotName := fmt.Sprintf("%d", i)
		robotLocation[robotName] = robotName
	}
	sort.Strings(unvisited)
	fmt.Println("Unvisited", unvisited)
	fmt.Println(getShortestPathPart2(visited, unvisited, robotLocation, distances, make(map[string]int)))

}

func getShortestPathPart2(visited []string, unvisited []string, locations map[string]string, distances map[string]map[string]reachable, pathScoreCache map[string]int) int {
	cachingEnabled := true
	shortestMove := 0

	// cache key is location of each robot, plus unvisited locations in sorted order
	cacheKey := ""
	for _, i := range []string{"0", "1", "2", "3"} {
		cacheKey = cacheKey + locations[i]
	}
	cacheKey = cacheKey + ","
	for _, l := range unvisited {

		cacheKey = cacheKey + l

	}
	pathScore, present := pathScoreCache[cacheKey]
	if present && cachingEnabled {
		return pathScore
	}

	// for eatch robot in turn, look at the possible moves it could make
	// return the lowest out of each possible robot's move. Select the lowest of these.

	for robot := range locations {
		for next := range distances[robot] {
			routable := !sliceContains(visited, next)
			for _, barrier := range distances[locations[robot]][next].behindDoor {
				if !sliceContains(visited, strings.ToLower(barrier)) {
					routable = false
				}
			}

			if routable {
				newLocations := make(map[string]string)
				for r, l := range locations {
					newLocations[r] = l
				}
				newLocations[robot] = next
				newVisited := make([]string, len(visited))
				copy(newVisited, visited)
				newVisited = append(newVisited, next)
				newUnVisited := []string{}
				for _, place := range unvisited {
					if place != next && !sliceContains(newVisited, place) {
						newUnVisited = append(newUnVisited, place)
					}
				}
				move := distances[locations[robot]][next].distance + getShortestPathPart2(newVisited, newUnVisited, newLocations, distances, pathScoreCache)
				if move != 0 && (move < shortestMove || shortestMove == 0) {
					shortestMove = move
				}
			}

		}
	}
	pathScoreCache[cacheKey] = shortestMove
	return shortestMove

}

func getShortestPath(visited []string, unvisited []string, distances map[string]map[string]reachable, pathScoreCache map[string]int) int {
	if len(unvisited) == 1 {
		return distances[visited[len(visited)-1]][unvisited[0]].distance
	}

	var shortestPath int

	for _, nextDest := range unvisited {
		nextUnvisited := []string{}
		cacheKey := nextDest
		for _, place := range unvisited {
			if place != nextDest {
				nextUnvisited = append(nextUnvisited, place)
				cacheKey += place
			}
		}
		haveKeys := true
		for _, door := range distances[visited[len(visited)-1]][nextDest].behindDoor {
			if sliceContains(nextUnvisited, strings.ToLower(door)) {
				haveKeys = false
			}
		}
		if haveKeys {
			newVisited := make([]string, len(visited))
			copy(newVisited, visited)
			newVisited = append(newVisited, nextDest)
			cachedVal, present := pathScoreCache[cacheKey]
			pathLen := 0
			if present {
				pathLen = distances[visited[len(visited)-1]][nextDest].distance + cachedVal
			} else {
				cacheValue := getShortestPath(newVisited, nextUnvisited, distances, pathScoreCache)
				pathScoreCache[cacheKey] = cacheValue
				pathLen = distances[visited[len(visited)-1]][nextDest].distance + cacheValue
			}
			if pathLen < shortestPath || shortestPath == 0 {
				shortestPath = pathLen
			}
		}

	}
	return shortestPath
}

func renderMaze(mazeMap map[coordinate]string, youAreHere coordinate, bottomRight coordinate) {
	time.Sleep(time.Millisecond * 220)
	fmt.Print("\033[H\033[2J")
	for y := 0; y <= bottomRight.Y; y++ {
		for x := 0; x <= bottomRight.X; x++ {
			location := coordinate{x, y}
			if youAreHere == location {
				fmt.Print("@")
			} else {
				fmt.Printf("%s", mazeMap[coordinate{x, y}])
			}
		}
		fmt.Print("\n")

	}
}

func findDestinations(startingPoint string, mazeMap map[coordinate]string, startingPoints map[string]coordinate) map[string]reachable {

	distances := make(map[coordinate]reachable)
	results := make(map[string]reachable)
	findDistRecursive(mazeMap, distances, []string{}, startingPoints[startingPoint], 0)

	for k, v := range startingPoints {
		if distances[v].distance > 0 {
			results[k] = distances[v]
		}
	}
	return results
}

func findDistRecursive(mazeMap map[coordinate]string, distances map[coordinate]reachable, doorsPassed []string, location coordinate, distance int) {
	current, present := mazeMap[location]
	_, visited := distances[location]
	if visited || !present || current == "#" {
		return
	}
	newDoorsPassed := make([]string, len(doorsPassed))
	copy(newDoorsPassed, doorsPassed)
	isDoor, _ := regexp.MatchString("[A-Z]", current)
	if isDoor {
		newDoorsPassed = append(newDoorsPassed, current)
	}
	neighbours := []coordinate{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}

	for _, neighbour := range neighbours {
		neighbourDistance, mapped := distances[coordinate{location.X + neighbour.X, location.Y + neighbour.Y}]
		if mapped {
			if (neighbourDistance.distance + 1) < distance {
				distance = neighbourDistance.distance + 1
			}
		}

	}
	distances[location] = reachable{distance, newDoorsPassed}
	for _, neighbour := range neighbours {

		findDistRecursive(mazeMap, distances, newDoorsPassed, coordinate{location.X + neighbour.X, location.Y + neighbour.Y}, distance+1)

	}
}

func generateMazeMap(filename string) (map[coordinate]string, map[string]coordinate) {
	inputBytes, err := ioutil.ReadFile(filename)
	atCount := 0
	if err != nil {
		panic(err)
	}
	locations := make(map[string]coordinate)
	mazeMap := make(map[coordinate]string)
	x := 0
	y := 0

	for _, b := range inputBytes {
		if b == '\n' {
			y++
			x = 0
		} else if b == '@' {
			locations[fmt.Sprintf("%d", atCount)] = coordinate{x, y}
			atCount++
		} else if rune(b) >= 97 && rune(b) <= 122 {
			locations[string(b)] = coordinate{x, y}
		}
		mazeMap[coordinate{x, y}] = string(b)
		x++
	}

	return mazeMap, locations

}

func sliceContains(target []string, searchFor string) bool {
	for _, potentialMatch := range target {
		if potentialMatch == searchFor {
			return true
		}
	}
	return false
}
