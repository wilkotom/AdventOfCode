package main

import (
	"fmt"
)

type coordinate struct {
	X int
	Y int
	Z int
}

type planet struct {
	Position      coordinate
	Velocity      coordinate
	period        coordinate
	Name          string
	StartPos      coordinate
	StartVelocity coordinate
}

func main() {

	planets := []planet{}
	emptyCoordinate := coordinate{0, 0, 0}
	planets = append(planets, planet{coordinate{-9, -1, -1}, emptyCoordinate, emptyCoordinate, "io", coordinate{9, -1, -1}, emptyCoordinate})          //io
	planets = append(planets, planet{coordinate{2, 9, 5}, emptyCoordinate, emptyCoordinate, "Europa", coordinate{2, 9, 5}, emptyCoordinate})           //europa
	planets = append(planets, planet{coordinate{10, 18, -12}, emptyCoordinate, emptyCoordinate, "Ganymede", coordinate{10, 18, -12}, emptyCoordinate}) //ganymede
	planets = append(planets, planet{coordinate{-6, 15, -7}, emptyCoordinate, emptyCoordinate, "Callisto", coordinate{-6, 15, -7}, emptyCoordinate})   //callisto

	// planets = append(planets, planet{coordinate{-1, 0, 2}, emptyCoordinate, emptyCoordinate, "io"})       //io
	// planets = append(planets, planet{coordinate{2, -10, -7}, emptyCoordinate, emptyCoordinate, "Europa"}) //io
	// planets = append(planets, planet{coordinate{4, -8, 8}, emptyCoordinate, emptyCoordinate, "Ganymede"}) //io
	// planets = append(planets, planet{coordinate{3, 5, -1}, emptyCoordinate, emptyCoordinate, "Callisto"}) //io

	// planets = append(planets, planet{position{-8,-10,0},velocity{0,0,0},"io"}) //io
	// planets = append(planets, planet{position{5,5,10},velocity{0,0,0},"Europa"}) //europa
	// planets = append(planets, planet{position{2,-7,3},velocity{0,0,0},"Ganymede"}) //ganymede
	// planets = append(planets, planet{position{9,-8,-3},velocity{0,0,0},"Callisto"}) //callisto
	xPeriod := 0
	yPeriod := 0
	zPeriod := 0
	count := 0
	for xPeriod == 0 || yPeriod == 0 || zPeriod == 0 {
		// if count % 10 == 0 {
		// 	fmt.Println(planets)
		// }
		if count == 1000 {
			total := 0
			for _, planet := range planets {
				pot := abs(planet.Position.X) + abs(planet.Position.Y) + abs(planet.Position.Z)
				kin := abs(planet.Velocity.X) + abs(planet.Velocity.Y) + abs(planet.Velocity.Z)
				total = total + pot*kin
			}
			fmt.Printf("Total energy after 1000 iterations: %d\n", total)

		}

		for i := range planets {
			for j := range planets[i+1:] {
				if planets[i].Position.X > planets[i+j+1].Position.X {
					planets[i].Velocity.X--
					planets[i+j+1].Velocity.X++
				} else if planets[i].Position.X < planets[i+j+1].Position.X {
					planets[i].Velocity.X++
					planets[i+j+1].Velocity.X--
				}

				if planets[i].Position.Y > planets[i+j+1].Position.Y {
					planets[i].Velocity.Y--
					planets[i+j+1].Velocity.Y++
				} else if planets[i].Position.Y < planets[i+j+1].Position.Y {
					planets[i].Velocity.Y++
					planets[i+j+1].Velocity.Y--
				}
				if planets[i].Position.Z > planets[i+j+1].Position.Z {
					planets[i].Velocity.Z--
					planets[i+j+1].Velocity.Z++
				} else if planets[i].Position.Z < planets[i+j+1].Position.Z {
					planets[i].Velocity.Z++
					planets[i+j+1].Velocity.Z--
				}
			}

		}

		for i := range planets {
			planets[i].Position.X = planets[i].Position.X + planets[i].Velocity.X
			planets[i].Position.Y = planets[i].Position.Y + planets[i].Velocity.Y
			planets[i].Position.Z = planets[i].Position.Z + planets[i].Velocity.Z
			if planets[i].Velocity.X == 0 && planets[i].period.X == 0 {
				planets[i].period.X = count * 2
			}
			if planets[i].Velocity.Y == 0 && planets[i].period.Y == 0 {
				planets[i].period.Y = count * 2
			}
			if planets[i].Velocity.Z == 0 && planets[i].period.Z == 0 {
				planets[i].period.Z = count * 2
			}
		}

		if xPeriod == 0 && count > 1 {
			matched := true
			for i := range planets {

				matched = matched && planets[i].StartPos.X == planets[i].Position.X && planets[i].StartVelocity.X == planets[i].Velocity.X
			}
			if matched {
				xPeriod = count
				fmt.Printf("X Period: %d\n", xPeriod)
			}
		}
		if yPeriod == 0 && count > 1 {
			matched := true
			for i := range planets {
				matched = matched && planets[i].StartPos.Y == planets[i].Position.Y && planets[i].StartVelocity.Y == planets[i].Velocity.Y
			}
			if matched {
				yPeriod = count
				fmt.Printf("Y Period: %d\n", yPeriod)
			}
		}
		if zPeriod == 0 && count > 1 {
			matched := true
			for i := range planets {
				matched = matched && planets[i].StartPos.Z == planets[i].Position.Z && planets[i].StartVelocity.Z == planets[i].Velocity.Z
			}
			if matched {
				zPeriod = count
				fmt.Printf("Z Period: %d\n", zPeriod)
			}
		}
		count++

		if count == 1 {
			for i := range planets {
				planets[i].StartPos = planets[i].Position
				planets[i].StartVelocity = planets[i].Velocity
			}
		}

	}

	highest := xPeriod

	if yPeriod > highest {
		highest = yPeriod
	}
	if zPeriod > highest {
		highest = zPeriod
	}

	steps := highest

	for !(steps%xPeriod == 0 && steps%yPeriod == 0 && steps%zPeriod == 0) {
		steps += highest
	}

	fmt.Printf("Number of steps to reach same state: %d\n", steps)

}

func abs(val int) int {
	if val < 0 {
		val = 0 - val
	}
	return val
}
