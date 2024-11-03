package main
import (
	"fmt"
	"os"
	"bufio"
	"strings"
)

func main() {
	filename := "orbits.txt"
	orbits := getOrbits(filename) 
	distances := getDistances(orbits)
	total := 0
	for _, dist := range distances {
		total += dist
	}
	fmt.Println(total)
	fmt.Println(getOrbitalTransfers("YOU", "SAN", orbits))
}

func getOrbits(filename string) map[string]string {
	file, _ := os.Open(filename)
	defer file.Close()

	orbits := make(map[string]string)
	scanner := bufio.NewScanner(file)
    for scanner.Scan() {
		points := strings.Split(scanner.Text(),")")
		orbits[points[1]] = points[0]
		
	}
	return orbits
}

func getDistances(orbits map[string]string) map[string]int {
	distances := make(map[string]int)
	distances["COM"] = 0
	for planet := range orbits {
		getDistance(planet, orbits, distances)
	}
	return distances
}

func getDistance(planet string, orbits map[string]string, distances map[string]int) int {
	var distance int
	if computed, present := distances[planet]; present {
		distance = computed
	} else if parentDist, present := distances[orbits[planet]]; present {
		distance = 1 + parentDist
		distances[planet] = distance
	} else {
		distance = 1 + getDistance(orbits[planet], orbits, distances)
		distances[planet] = distance
	}
	return distance
}

func getPath(startpoint string, orbits  map[string]string) []string {
	path := []string{}
	for orbits[startpoint] != "COM" {
		path = append([]string{orbits[startpoint]}, path...)
		startpoint = orbits[startpoint] 
	}
	return path
}

func getOrbitalTransfers(point1 string, point2 string, orbits map[string]string) int {
	point1Path := getPath(point1, orbits)
	point2Path := getPath(point2, orbits)
	for i :=0; i<len(point1Path); i++ { // This will crash if YOU and SAN don't orbit the same COM
		if point1Path[i] != point2Path[i] {
			return len(point1Path[i:]) + len(point2Path[i:])
		}
	}
	return -1
}