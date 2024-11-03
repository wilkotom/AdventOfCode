package day15

import (
	"fmt"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

type Scanner struct {
	Location helpers.Coordinate[int64]
	Radius   int64
}

func Day15() {
	lines, err := helpers.ReadFileToLines("inputs/15.txt")

	if err != nil {
		panic(err)
	}

	scanners := make([]Scanner, 0)
	beacons := make(helpers.Set[helpers.Coordinate[int64]])

	for _, line := range lines {
		var scanner helpers.Coordinate[int64]
		var beacon helpers.Coordinate[int64]
		_, err := fmt.Fscanf(strings.NewReader(line), "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d", &scanner.X, &scanner.Y, &beacon.X, &beacon.Y)
		if err != nil {
			panic(err)
		}
		scanners = append(scanners, Scanner{Location: scanner, Radius: scanner.ManhattanDistance(beacon)})
		beacons.Insert(beacon)
	}
	fmt.Println(part1(scanners, beacons, 2000000))
	fmt.Println(part2(scanners, beacons))

}

func part1(scanners []Scanner, beacons helpers.Set[helpers.Coordinate[int64]], yVal int64) int64 {
	var minX, maxX, maxRadius, result int64
	minX = scanners[0].Location.X
	maxX = scanners[0].Location.X
	for _, scanner := range scanners {
		if scanner.Radius > maxRadius {
			maxRadius = scanner.Radius
		}
	}

	for _, scanner := range scanners {
		if scanner.Location.X < minX {
			minX = scanner.Location.X
		} else if scanner.Location.X > maxX {
			maxX = scanner.Location.X
		}
	}
	minX -= maxRadius
	maxX += maxRadius
	for x := minX; x <= maxX; x++ {
		testLocation := helpers.Coordinate[int64]{X: x, Y: yVal}
		if !beacons.Contains(testLocation) {
			for _, scanner := range scanners {
				if scanner.Location.ManhattanDistance(testLocation) <= scanner.Radius {
					result++
					break
				}
			}
		}
	}
	return result
}

func part2(scanners []Scanner, beacons helpers.Set[helpers.Coordinate[int64]]) int64 {

	for _, scanner := range scanners {
	nextPotential:
		for _, potential := range scanner.Location.ManhattanCircle(scanner.Radius + 1) {
			if potential.X >= 0 && potential.X <= 4000000 && potential.Y >= 0 && potential.Y <= 4000000 {
				for _, other := range scanners {
					if other.Location.ManhattanDistance(potential) <= other.Radius {
						continue nextPotential
					}
				}
				return potential.X*4000000 + potential.Y
			}
		}
	}
	return 0

}
