package day13

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

const (
	Less = iota
	Equal
	Greater
)

type packet interface {
	isSingleValue() bool
}

type singleValue int
type valueList []packet

func (v singleValue) isSingleValue() bool {
	return true
}

func (v valueList) isSingleValue() bool {
	return false
}

func Day13() {
	packet_pairs, err := helpers.ReadFileToRecords("inputs/13.txt", "\n\n")

	if err != nil {
		panic(err)
	}
	total := 0
	lowerDivider := parsePacket("[[2]]")
	upperDivider := parsePacket("[[6]]")
	belowLower := 0
	belowUpper := 0
	for n, pair := range packet_pairs {
		packets := strings.Split(pair, "\n")
		leftPacket := parsePacket(packets[0])
		rightPacket := parsePacket(packets[1])
		if ordering(leftPacket, lowerDivider) == Less {
			belowLower++
		}
		if ordering(leftPacket, upperDivider) == Less {
			belowUpper++
		}

		if ordering(rightPacket, lowerDivider) == Less {
			belowLower++
		}
		if ordering(rightPacket, upperDivider) == Less {
			belowUpper++
		}

		if ordering(leftPacket, rightPacket) == Less {
			total += n + 1
		}
	}
	fmt.Printf("Part 1: %v\n", total)
	fmt.Printf("Part 2: %v\n", (belowLower+1)*(belowUpper+2))

}

func parsePacket(packetText string) valueList {
	packetText = packetText[1 : len(packetText)-1]

	items := make([]string, 0)
	startPosition := 0
	endPosition := 1
	var parsedPacket valueList

	for endPosition < len(packetText) {
		if packetText[startPosition] == '[' {
			level := 0
			level++
			for level > 0 {

				if packetText[endPosition] == '[' {
					level++
				} else if packetText[endPosition] == ']' {
					level--
				}
				endPosition++
			}
			if endPosition > len(packetText) {
				endPosition--
			}
			items = append(items, packetText[startPosition:endPosition])
		} else {
			for endPosition < len(packetText) && packetText[endPosition] != ',' {
				endPosition++
			}

			if endPosition == len(packetText) {
				items = append(items, packetText[startPosition:])
			} else {
				items = append(items, packetText[startPosition:endPosition])
			}
		}
		startPosition = endPosition + 1
		endPosition = startPosition + 1
	}
	if startPosition < len(packetText) {
		items = append(items, packetText[startPosition:])
	}

	for _, item := range items {
		if item[0] == '[' {
			subPacket := parsePacket(item)
			parsedPacket = append(parsedPacket, subPacket)
		} else {
			value, _ := strconv.Atoi(item)
			parsedPacket = append(parsedPacket, singleValue(value))
		}
	}
	return parsedPacket
}

func ordering(left, right packet) int {
	if left.isSingleValue() && right.isSingleValue() {
		// fmt.Printf("Single Values %v, %v\n", left, right)

		if left.(singleValue) < right.(singleValue) {
			return Less
		} else if left.(singleValue) == right.(singleValue) {
			return Equal
		} else {
			return Greater
		}
	} else if left.isSingleValue() {
		newLeft := make(valueList, 0)
		newLeft = append(newLeft, left)
		return ordering(newLeft, right)
	} else if right.isSingleValue() {
		newRight := make(valueList, 0)
		newRight = append(newRight, right)
		return ordering(left, newRight)
	} else {
		for len(left.(valueList)) > 0 && len(right.(valueList)) > 0 {
			nextLeft := left.(valueList)[0]
			left = left.(valueList)[1:]
			nextRight := right.(valueList)[0]
			right = right.(valueList)[1:]
			order := ordering(nextLeft, nextRight)
			if order != Equal {
				return order
			}
		}
		if len(left.(valueList)) == 0 && len(right.(valueList)) > 0 {
			return Less
		} else if len(left.(valueList)) > 0 && len(right.(valueList)) == 0 {
			return Greater
		} else {
			return Equal
		}

	}
}
