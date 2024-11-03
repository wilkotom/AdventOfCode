package main
import (
	"github.com/wilkotom/AoC2019/intcode"
	"fmt"
	// "time"
)

type coordinate struct {
	X int
	Y int
}

func main() {
	sectionA()
	sectionB()
}

func sectionA() {
	intComputer := intcode.StartIntCodeComputer("input.txt")

	var lines []string
	var neighbourCount [][]int
	var currentPosition coordinate
	scaffoldLine := ""
	for {
		result, ok := <- intComputer.Output

		if !ok {
			break
		}
		if result == 10 {
			lines = append(lines, scaffoldLine)
			scaffoldLine = ""
		} else {

			scaffoldLine = scaffoldLine + string(result)
			if result != '.' && result != '#' {
				currentPosition.X = len(scaffoldLine) -1
				currentPosition.Y = len(lines)
			}
		}
	}

	for _, line := range lines {
		fmt.Println(line)
	}
	fours := 0
	maxY :=  len(lines)-1 // there's an extra blank line on the end of the string representation of the track
	for Y := 0; Y < maxY; Y++ {
		var nbrLine []int
		maxX := len(lines[Y])

		for X:= 0; X < maxX; X++ {
			neighbours := getNeighbours(X, Y, maxX-1, maxY-1)
			nbrCount := 0
			if lines[Y][X] == '#' {
				fmt.Println(X, Y, neighbours, maxX, maxY )
				for _, nbr := range neighbours {
					fmt.Printf("(%d,%d) - looking at (%d, %d)\n", X, Y, nbr.X, nbr.Y)
					if lines[nbr.Y][nbr.X] == '#' {
						nbrCount++
					}
				}
			}
			if nbrCount == 4{
				fours = fours + (X*Y)
			}
			nbrLine = append(nbrLine, nbrCount)
		}
		neighbourCount = append(neighbourCount, nbrLine)
	}
	// for _, line := range neighbourCount {
	// 	fmt.Println(line)
	// }

	for _, line := range lines {
		fmt.Println(line)

	}
	fmt.Println(fours, currentPosition.X, currentPosition.Y)

	pointing := lines[currentPosition.Y][currentPosition.X]
	steps := 0
	route := ""
	finished := false
	for !finished{
		
		fmt.Print("\033[H\033[2J")
		for _, line := range lines {
			fmt.Println(line)
		}
		fmt.Println(currentPosition)
		fmt.Println(len(lines), len(lines[0]))
		fmt.Println(route)
		fmt.Println(steps)
		// time.Sleep(time.Millisecond * 50)
		// fmt.Println(string(pointing))
		// fmt.Println(lines[currentPosition.Y])
		switch pointing {
		case '^':
			if currentPosition.Y == 0 || lines[currentPosition.Y-1][currentPosition.X] == '.' {
				if steps > 0 {
					route = route + fmt.Sprintf("%d,", steps)
				}
				steps = 0
				if currentPosition.X > 0 && 
						(lines[currentPosition.Y][currentPosition.X-1] == '#' || 
						 lines[currentPosition.Y][currentPosition.X-1] == '>' ||
						 lines[currentPosition.Y][currentPosition.X-1] == '<') {
					pointing = '<'
					route = route + "L,"
					currentPosition.X--
					steps++

				} else if (currentPosition.X+1 == len(lines[0]) -1) || lines[currentPosition.Y][currentPosition.X+1] == '.' {
					// route = route + fmt.Sprintf("%d,", steps)

					finished = true
				} else {
					pointing = '>'
					currentPosition.X++
					steps++
					route = route + "R,"
				}
			} else {
				currentPosition.Y--
				steps ++
			}
		case '>':
			if currentPosition.X == len(lines[0]) -1 || lines[currentPosition.Y][currentPosition.X+1] == '.' {
				if steps > 0 {
					route = route + fmt.Sprintf("%d,", steps)
				}
				steps = 0
				if currentPosition.Y > 0 && 
						(lines[currentPosition.Y-1][currentPosition.X] == '#' || 
						lines[currentPosition.Y-1][currentPosition.X] == '<' || 
						lines[currentPosition.Y-1][currentPosition.X] == '>' ) {
					pointing = '^'
					route = route + "L,"
					currentPosition.Y --
					steps++
				} else if (currentPosition.Y+1 < maxY -1) && lines[currentPosition.Y+1][currentPosition.X] == '.' {
					// route = route + fmt.Sprintf("%d", steps)

					finished = true
				} else {
					pointing = 'v'
					currentPosition.Y ++
					steps++
					route = route + "R,"
				}
			} else {
				currentPosition.X++
				steps ++
			}
		case 'v':
			if currentPosition.Y  == maxY -1 || lines[currentPosition.Y+1][currentPosition.X] == '.'  {
				if steps > 0 {
					route = route + fmt.Sprintf("%d,", steps)
				}

				steps = 0
				if currentPosition.X < len(lines[currentPosition.Y]) && 
						(lines[currentPosition.Y][currentPosition.X+1] == '#' ||
						lines[currentPosition.Y][currentPosition.X+1] == '<' ||
						lines[currentPosition.Y][currentPosition.X+1] == '>'){
					pointing = '>'
					currentPosition.X++
					steps++
					route = route + "L,"
				} else if (currentPosition.X-1 < 0) || lines[currentPosition.Y][currentPosition.X-1] == '.' {
					// route = route + fmt.Sprintf("%d", steps)
					finished = true
				} else {
					pointing = '<'
					currentPosition.X--
					steps++
					route = route + "R,"
				}
			} else {

				currentPosition.Y++
				steps ++
			}
		case '<':
			if currentPosition.X == 0 || lines[currentPosition.Y][currentPosition.X-1] == '.' {
				if steps > 0 {
					route = route + fmt.Sprintf("%d,", steps)
				}
				steps = 0
				if currentPosition.X > 0 &&  
						(lines[currentPosition.Y+1][currentPosition.X] == '#' || 
						lines[currentPosition.Y+1][currentPosition.X] == '^' ||
						lines[currentPosition.Y+1][currentPosition.X] == 'v') {
					pointing = 'v'
					route = route + "L,"
					currentPosition.Y++
					steps++
				} else if (currentPosition.Y-1 < 0) || lines[currentPosition.Y-1][currentPosition.X] == '.' {
					// route = route + fmt.Sprintf("%d", steps)
					finished = true
				} else {
					pointing = '^'
					currentPosition.Y--
					steps++
					route = route + "R,"
				}
			} else {
				currentPosition.X--
				steps ++
			}
		default:
			panic("Can't understand " + string(pointing))
		}
		lines[currentPosition.Y] = replaceAtIndex(lines[currentPosition.Y], fmt.Sprintf("%d", steps)[0], currentPosition.X)

	}
	fmt.Print("\033[H\033[2J")
	for _, line := range lines {
		fmt.Println(line)
	}
	fmt.Println(currentPosition)
	fmt.Println(len(lines), len(lines[0]))
	fmt.Println(route)
	fmt.Println(steps)
}

func sectionB() {
	fmt.Println("Section B")
	intComputer := intcode.StartIntCodeComputer("partb.txt")
	solution := "A,A,B,C,B,C,B,C,A,C\nR,6,L,8,R,8\nR,4,R,6,R,6,R,4,R,4\nL,8,R,6,L,10,L,10\nn\n"

	for _, entry := range solution{
		intComputer.Input <- int(entry)
		fmt.Println(rune(entry))
	}

	for {
		out, ok := <- intComputer.Output
		if out > 128 {
			fmt.Println("Dust:", out)
		} else {
			fmt.Print(string(out))
			if out == 0 || !ok {
				break
			}
		}
	}


}

func getNeighbours(X, Y, maxX, maxY int) []coordinate {
	if X == 0 && Y == 0 {
		return []coordinate{{X+1, Y}, {X, Y+1}}
	} else if X == 0 {
		return []coordinate{{X+1, Y},  {X, Y-1}, {X, Y+1}}
	} else if Y == 0 {
		return []coordinate{{X+1, Y},  {X-1, Y}, {X, Y+1}}
	} else if X == maxX && Y == maxX {
		return []coordinate{{X-1, Y}, {X, Y-1}}
	} else if X == maxX  {
		return []coordinate{{X-1, Y}, {X, Y-1}, {X, Y+1}}
	} else if Y == maxY {
		return []coordinate{{X-1, Y}, {X+1, Y}, {X, Y-1}}
	} 
	return []coordinate{{X-1, Y}, {X+1, Y}, {X, Y-1}, {X, Y+1}}
}

func replaceAtIndex(in string, r byte, i int) string {
    out := []byte(in)
    out[i] = r
    return string(out)
}

/*
A,A,B,C,B,C,B,C,A,C

A:R,6,L,8,R,8 
B: R,4,R,6,R,6,R,4,R,4
C: L,8,R,6,L,10,L,10
*/