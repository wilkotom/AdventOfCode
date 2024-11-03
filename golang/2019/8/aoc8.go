package main
import(
	"fmt"
	"os"
	"io"
	"math"
)

func main() {
	filename := "image.dat"
	width := 25
	height := 6
	layers := []string{}
	imageFileHandle, _ := os.Open(filename)
	for {
		layer := make([]byte,width*height)

		_, err := imageFileHandle.Read(layer)
		if err != nil {
			if err != io.EOF {
				panic(err)
			} else {
				break
			}
		}  
		layers = append(layers, string(layer))
	}
	partOne(layers)
	partTwo(layers, width, height)
}

func partOne (layers []string){
	//fmt.Println(layers)
	leastZeros := math.MaxInt64
	score := 0
	for _, layer := range layers {
		uniqueChars := make(map[string]int)
		for _, char := range layer {
			uniqueChars[string(char)]++
		}
		if uniqueChars["0"] < leastZeros {
			score = uniqueChars["1"] * uniqueChars["2"]
			leastZeros = uniqueChars["0"]
		}
	}
	fmt.Println(score)
	// part 2
	// 0: black
	// 1: white
	// 2: transparent
}

func partTwo (layers []string, width, height int) {
	final := make([]rune, width*height)
	for i := range final {
		for j := range layers {
			if layers[j][i] == '0' {
				final[i] = ' '
				break
			} else if layers[j][i] == '1' {
				final[i] = '█'
				break
			}
		}
	}
	for i := 0;  i < len(final);  {
		fmt.Println(string(final[i:i+width]))
		i = i + width
	}
}