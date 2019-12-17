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
	Position coordinate
	Velocity coordinate
	period coordinate
	Name string
}

func main(){

	planets := []planet{}
	emptyCoordinate := coordinate{0,0,0}
	planets = append(planets, planet{coordinate{-9,-1,-1},emptyCoordinate, emptyCoordinate, "io"}) //io
	planets = append(planets, planet{coordinate{2,9,5},emptyCoordinate, emptyCoordinate, "Europa"}) //europa
	planets = append(planets, planet{coordinate{10,18,-12},emptyCoordinate, emptyCoordinate, "Ganymede"}) //ganymede
	planets = append(planets, planet{coordinate{-6,15,-7},emptyCoordinate, emptyCoordinate, "Callisto"}) //callisto

	// planets = append(planets, planet{position{-1,0,2},velocity{0,0,0},"io"}) //io
	// planets = append(planets, planet{position{2,-10,-7},velocity{0,0,0},"Europa"}) //io
	// planets = append(planets, planet{position{4,-8,8},velocity{0,0,0},"Ganymede"}) //io
	// planets = append(planets, planet{position{3,5,-1},velocity{0,0,0},"Callisto"}) //io

	// planets = append(planets, planet{position{-8,-10,0},velocity{0,0,0},"io"}) //io
	// planets = append(planets, planet{position{5,5,10},velocity{0,0,0},"Europa"}) //europa
	// planets = append(planets, planet{position{2,-7,3},velocity{0,0,0},"Ganymede"}) //ganymede
	// planets = append(planets, planet{position{9,-8,-3},velocity{0,0,0},"Callisto"}) //callisto

	for count := 0; count < 1000000; count++ {
		// if count % 10 == 0 {
		// 	fmt.Println(planets)
		// }
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
	}
	total := 0
	for _, planet := range planets {
		fmt.Println(planet)
		pot := abs(planet.Position.X) + abs(planet.Position.Y) + abs(planet.Position.Z)
		kin := abs(planet.Velocity.X) + abs(planet.Velocity.Y) + abs(planet.Velocity.Z)
		fmt.Println(pot, kin, pot*kin)
		total = total + pot * kin
		fmt.Printf("%s, period X:%d Y:%d Z:%d,\n", planet.Name, planet.period.X, planet.period.Y, planet.period.Z)
	}
	fmt.Println(total)

}

func abs(val int) int {
	if val < 0 {
		val = 0 - val
	}
	return val
}