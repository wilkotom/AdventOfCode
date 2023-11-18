package day16

import (
	"fmt"
	"strings"

	"golang.org/x/exp/slices"

	"github.com/wilkotom/AoC2022/helpers"
)

type Valve struct {
	FlowRate  int
	Distances map[string]int
}

func Day16() {
	valves := parseValveDetails("inputs/16.txt")
	fmt.Println(valves, len(valves))
	valveNames := make([]string, len(valves))
	i := 0
	for valveName := range valves {
		valveNames[i] = valveName
		i++
	}
	fmt.Println(part1(valveNames, valves, 30))
	fmt.Println(part2(valveNames, valves, 26))

}

func parseValveDetails(filename string) map[string]Valve {
	valveDetails, err := helpers.ReadFileToLines(filename)

	if err != nil {
		panic(err)
	}
	valves := make(map[string]Valve)
	for _, valve := range valveDetails {
		splitpoint := strings.Index(valve, ";")
		var valveName string
		var flowRate int
		if _, err := fmt.Fscanf(strings.NewReader(valve[:splitpoint]), "Valve %s has flow rate=%d", &valveName, &flowRate); err != nil {
			panic(err)
		}
		distances := make(map[string]int)
		for _, adjacentValveName := range strings.Split(valve[splitpoint+1:], " ")[5:] {
			distances[adjacentValveName[:2]] = 1
		}
		distances[valveName] = 0
		valves[valveName] = Valve{FlowRate: flowRate,
			Distances: distances}
	}
	for valveName := range valves {
		for dest := range valves {
			if _, ok := valves[valveName].Distances[dest]; !ok {
				valves[valveName].Distances[dest] = 4294967296 // as near infinity as makes no odds?
			}

		}
	}
	// Floyd-Warshall Algorithm.  For each possible intermediate point, examine all start/end pairs.
	// If it's cheaper to go via that point, set that as the cost instead of the direct cost.
	for intermediateDest := range valves {
		for start := range valves {
			for end := range valves {
				if valves[start].Distances[intermediateDest]+valves[intermediateDest].Distances[end] < valves[start].Distances[end] {
					valves[start].Distances[end] = valves[start].Distances[intermediateDest] + valves[intermediateDest].Distances[end]
				}
			}
		}
	}

	for valve, details := range valves {
		if details.FlowRate == 0 && valve != "AA" {
			delete(valves, valve)
		} else {
			for dest := range details.Distances {
				if destDetails, ok := valves[dest]; !ok || destDetails.FlowRate == 0 || dest == valve {
					delete(details.Distances, dest)
				}
			}
		}
	}
	return valves
}

func part1(valveNames []string, valveDetails map[string]Valve, timeLimit int) int {
	existingPath := []string{"AA"}
	paths := make(chan []string, 500000)
	paths <- existingPath
	bestScore := 0
	target := 0
	for len(paths) > 0 {
		path := <-paths
		target--
	nextValveName:
		for _, valveName := range valveNames {
			for _, visited := range path {
				if valveName == visited {
					continue nextValveName
				}
			}
			// create a wholly new slice containing members of the parent
			// Doing otherwise resuts in issues whereby each new path of the same length is
			// overwritten by the next because they're the same address in memory.
			newPath := make([]string, 0, len(path)+1)
			newPath = append(newPath, path...)
			newPath = append(newPath, valveName)
			score := scoreForPath(newPath, valveDetails, timeLimit)
			if score > 0 {
				paths <- newPath
				if score > bestScore {
					bestScore = score
				}
			}
		}
	}
	return bestScore
}

// This gives the right answer but is incredibly slow an inefficient.
func part2(valveNames []string, valveDetails map[string]Valve, timeLimit int) int {
	existingPath := []string{"AA"}
	paths := make(chan []string, 500000)
	pathCache := make(map[string]int)
	paths <- existingPath
	bestScore := 0
	target := 0
	for len(paths) > 0 {
		path := <-paths
		target--
	nextValveName:
		for _, valveName := range valveNames {
			for _, visited := range path {
				if valveName == visited {
					continue nextValveName
				}
			}
			newPath := make([]string, 0, len(path)+1)
			newPath = append(newPath, path...)
			newPath = append(newPath, valveName)
			score := scoreForPath(newPath, valveDetails, timeLimit)
			if score > 0 && len(path) >= len(valveDetails)/2-2 {

				unvisited := make([]string, 0, len(valveNames)-len(newPath))
				for _, valve := range valveNames {
					if !slices.Contains(newPath, valve) {
						unvisited = append(unvisited, valve)
					}
				}
				complementScore := part1(unvisited, valveDetails, timeLimit)
				score = score + complementScore
				if len(newPath) < len(valveDetails)/2 {
					paths <- newPath
				}
				if score > bestScore {
					bestScore = score
				}
			} else if score > 0 && len(path) <= len(valveDetails)/2 {
				cacheKey := strings.Join(newPath, "")
				pathCache[cacheKey] = score
				paths <- newPath

			}
		}
	}
	return bestScore
}

func scoreForPath(path []string, valveDetails map[string]Valve, timeLimit int) int {
	if len(path) == 1 {
		return (timeLimit - 1) * valveDetails[path[0]].FlowRate
	}
	time := 0
	score := 0
	for i, place := range path[1:] {
		details := valveDetails[path[i+1]]
		// fmt.Println(i, details)

		time += valveDetails[path[i]].Distances[place] + 1
		score += (timeLimit - time) * details.FlowRate
		if time > timeLimit {
			return -1
		}
	}
	return score
}
