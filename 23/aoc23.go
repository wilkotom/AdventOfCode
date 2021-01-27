package main

import (
	"fmt"
	"time"

	"github.com/wilkotom/AoC2019/intcode"
)

func main() {
	computers := make(map[int]intcode.Computer)
	for i := 0; i < 50; i++ {
		computers[i] = intcode.StartIntCodeComputer("aoc23.txt")
		computers[i].Input <- i
		computers[i].Input <- -1
	}
	var natX int
	var natY int
	seenNatY := make(map[int]bool)
	for true {
		for i := 0; i < 50; i++ {

			select {
			case dest, ok := <-computers[i].Output:
				if ok {
					x, _ := <-computers[i].Output
					y, _ := <-computers[i].Output

					if dest == 255 {

						natX = x
						natY = y
					} else {
						computers[dest].Input <- x
						computers[dest].Input <- y
					}
				} else {
					fmt.Println("Channel closed!")
					return
				}
			default:
			}

		}
		// wait for all computers to complete operations before checking input / output before checking their input / output buffers. Nasty hack.
		time.Sleep(time.Microsecond * 100)
		totalQueueLen := 0
		for i := 0; i < 50; i++ {
			totalQueueLen = totalQueueLen + len(computers[i].Input) + len(computers[i].Output)
		}
		if totalQueueLen == 0 {
			if seenNatY[natY] {
				fmt.Printf("Seen %d twice \n", natY)
				return
			}
			computers[0].Input <- natX
			computers[0].Input <- natY
			seenNatY[natY] = true
		}
	}
}
