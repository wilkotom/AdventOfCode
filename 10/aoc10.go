package main
import (
	"strings"
	"fmt"
	"io/ioutil"
	"math"
	"sort"
)

type asteroidLocation struct {
	X int
	Y int
	distance float64

}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
    if err != nil {
        panic(err)
	}
	inputLines := strings.Split(string(inputBytes), "\n")
	asteroidMap := make([][]int, len(inputLines))

	for i, line := range inputLines{
		asteroidMap[i] = make([]int, len(inputLines[i]))
		for j, c := range line {
			if c == 35 {
				asteroidMap[i][j] = 1
			}
		}
	}
	// fmt.Println(asteroidMap)
	mostAsteroids := 0
	coordinates := [2]int{-1,-1}
	for i := range inputLines{
		for j := range inputLines[i] {
			if asteroidMap[i][j] == 1 {
				asteroidsVisible := getVisibleAsteroidCount(i, j, asteroidMap)
				if len(asteroidsVisible) > mostAsteroids {
					mostAsteroids = len(asteroidsVisible)
					coordinates = [2]int{j,i}
				}
			}
		}

	}
	fmt.Println(mostAsteroids, coordinates)
	asteroids := getVisibleAsteroidCount(coordinates[0], coordinates[1], asteroidMap)
	for _, bearing := range asteroids {
		sort.Slice(bearing, func(i, j int) bool {
			return bearing[i].distance < bearing[j].distance
		  })
	}
	angles := make([]float64, 0, len(asteroids))
	for angle := range asteroids {
		angles = append(angles, angle)
	}
	sort.Float64s(angles)
	count := 0
	for  count < 41 {
		for _, angle := range angles {
			fmt.Println(count, angle, asteroids[angle])
			if len(asteroids[angle]) > 0 {
				count++
				zapped := asteroids[angle][0]
				asteroidMap[zapped.X][zapped.Y] = count
				asteroids[angle] = asteroids[angle][1:]
				if count == 41 {
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
	for i := range asteroidMap{
		for j := range asteroidMap[i] {
			if !(i == astY && j == astX) && asteroidMap[j][i] == 1 {
				diffY := float64(astY - i)
				diffX := float64(astX - j)
				distance := math.Sqrt(diffX * diffX + diffY* diffY )
				location := asteroidLocation{j,i, distance}
				vector := math.Atan2(-diffY,diffX)
				if vector < 0 {
					vector = vector + 2 * math.Pi
				}
				vector = vector + 0.5 * math.Pi
				// fmt.Println("Can see asteroid at ", i, j, math.Atan2(diffY, diffX))
				vectors[vector] = append(vectors[vector],location)
			}
		}
	}
	return vectors
}