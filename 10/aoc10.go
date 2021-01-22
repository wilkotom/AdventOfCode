package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"sort"
	"strings"
)

type asteroidLocation struct {
	coordinate [2]int
	distance   float64
}

func msain() {
	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	inputLines := strings.Split(string(inputBytes), "\n")
	asteroidMap := make([][]int, len(inputLines))

	for i, line := range inputLines {
		asteroidMap[i] = make([]int, len(inputLines[i]))
		for j, c := range line {
			if c == 35 {
				asteroidMap[i][j] = 1
			}
		}
	}
	// fmt.Println(asteroidMap)
	mostAsteroids := make(map[float64][]asteroidLocation)
	coordinates := [2]int{-1, -1}
	for i := range inputLines {
		for j := range inputLines[i] {
			if asteroidMap[j][i] == 1 {
				asteroidsVisible := getVisibleAsteroidCount(j, i, asteroidMap)
				if len(asteroidsVisible) > len(mostAsteroids) {
					mostAsteroids = asteroidsVisible
					coordinates = [2]int{j, i}
				}
			}
		}

	}
	fmt.Println("Most visible Asteroids at:", coordinates, "There are ", len(mostAsteroids), " asteroids visible")
	// asteroids := getVisibleAsteroidCount(coordinates[0], coordinates[1], asteroidMap)
	for _, bearing := range mostAsteroids {
		sort.Slice(bearing, func(i, j int) bool {
			return bearing[i].distance < bearing[j].distance
		})
	}
	angles := make([]float64, 0, len(mostAsteroids))
	for angle := range mostAsteroids {
		angles = append(angles, angle)
	}
	sort.Float64s(angles)
	count := 0
	for count <= 200 {
		for _, angle := range angles {
			fmt.Println(count, angle, mostAsteroids[angle])
			if len(mostAsteroids[angle]) > 0 {
				count++
				zapped := mostAsteroids[angle][0]
				asteroidMap[zapped.coordinate[0]][zapped.coordinate[1]] = count
				mostAsteroids[angle] = mostAsteroids[angle][1:]
				if count == 200 {
					fmt.Println(zapped)
					fmt.Println(asteroidMap)
					break
				}
			}
		}
	}
}

func getVisibleAsteroidCount(astX, astY int, asteroidMap [][]int) map[float64][]asteroidLocation {
	vectors := make(map[float64][]asteroidLocation)
	for y := range asteroidMap {
		for x := range asteroidMap[y] {
			if !(x == astX && y == astY) && asteroidMap[x][y] == 1 {
				diffX := float64(x - astX)
				diffY := float64(y - astY)
				distance := math.Sqrt(diffX*diffX + diffY*diffY)
				location := asteroidLocation{[2]int{x, y}, distance}
				vector := math.Atan2(diffX, diffY) + math.Pi
				if vector >= 2*math.Pi {
					vector = vector - 2*math.Pi
				}
				// if vector <= math.Pi && vector >= 0 {
				// 	vector = math.Abs(vector - math.Pi/2)
				// } else if vector < 0 {
				// 	vector = math.Abs(vector) + math.Pi/2
				// } else {
				// 	vector = math.Pi*1.5 - vector
				// }

				// 	vector = vector + 2*math.Pi
				// } else if vector >= 2*math.Pi {
				// 	vector = 2*math.Pi - vector
				// }
				// vector = 2*math.Pi - vector
				// if vector >= 2*math.Pi {
				// 	vector = 2*math.Pi - vector
				// }
				vectors[vector] = append(vectors[vector], location)
			}
		}
	}
	return vectors
}
