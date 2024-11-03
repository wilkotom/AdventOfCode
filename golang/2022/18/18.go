package day18

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day18() {
	lines, err := helpers.ReadFileToLines("inputs/18.txt")
	if err != nil {
		panic(err)
	}
	cubes := make(helpers.Set[helpers.Coordinate3D[int]])
	for _, line := range lines {
		var x, y, z int
		_, err := fmt.Fscanf(strings.NewReader(line), "%d,%d,%d", &x, &y, &z)
		if err == nil {
			cubes.Insert(helpers.Coordinate3D[int]{X: x, Y: y, Z: z})
		}
	}
	fmt.Println("Part 1: ", part1(&cubes))
	fmt.Println("Part 2: ", part2(cubes))

}

func part1(cubes *helpers.Set[helpers.Coordinate3D[int]]) int {
	total := 0
	for cube := range *cubes {
		hiddenFaces := 0
		for _, neighbour := range cube.Neighbours() {
			if cubes.Contains(neighbour) {
				hiddenFaces++
			}
		}
		total += 6 - hiddenFaces
	}
	return total
}

func part2(cubes helpers.Set[helpers.Coordinate3D[int]]) int {
	bottomLeftFront := helpers.Coordinate3D[int]{X: 65535, Y: 65535, Z: 65535}
	topRightBack := helpers.Coordinate3D[int]{X: 0, Y: 0, Z: 0}
	for cube := range cubes {
		if cube.X < bottomLeftFront.X {
			bottomLeftFront.X = cube.X
		} else if cube.X > topRightBack.X {
			topRightBack.X = cube.X
		}
		if cube.Y < bottomLeftFront.Y {
			bottomLeftFront.Y = cube.Y
		} else if cube.Y > topRightBack.Y {
			topRightBack.Y = cube.Y
		}
		if cube.Z < bottomLeftFront.Z {
			bottomLeftFront.Z = cube.Z
		} else if cube.Z > topRightBack.Z {
			topRightBack.Z = cube.Z
		}
	}

	unvisited := make([]helpers.Coordinate3D[int], 0)
	visited := make(helpers.Set[helpers.Coordinate3D[int]])
	total := 0
	unvisited = append(unvisited, bottomLeftFront.Subtract(helpers.Coordinate3D[int]{X: 1, Y: 1, Z: 1}))
	for len(unvisited) > 0 {
		nextCube := unvisited[0]
		unvisited = unvisited[1:]
		if visited.Contains(nextCube) {
			continue
		}
		visited.Insert(nextCube)
		for _, neighbour := range nextCube.Neighbours() {
			if cubes.Contains(neighbour) {
				total++
			} else if neighbour.X >= (bottomLeftFront.X-1) && neighbour.X <= topRightBack.X+1 &&
				neighbour.Y >= bottomLeftFront.Y-1 && neighbour.Y <= topRightBack.Y+1 &&
				neighbour.Z >= bottomLeftFront.Z-1 && neighbour.Z <= topRightBack.Z+1 && !visited.Contains(neighbour) {
				unvisited = append(unvisited, neighbour)
			}
		}
	}
	return total

}
