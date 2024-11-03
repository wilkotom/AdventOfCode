package day20

import (
	"fmt"

	"github.com/wilkotom/AoC2022/helpers"
)

type ListNode struct {
	Value        int
	PreviousItem *ListNode
	NextItem     *ListNode
}

func Day20() error {
	nums, err := helpers.ReadFileToNumberList[int]("inputs/20.txt")
	if err != nil {
		return err
	}

	fmt.Println("Part 1:", partN(nums, 1, 1))
	fmt.Println("Part 2:", partN(nums, 811589153, 10))
	return nil
}

func partN(nums []int, multiplier, iterations int) int {

	nodes := make([](*ListNode), len(nums))
	startingNode := ListNode{Value: nums[0] * multiplier}
	nodes[0] = &startingNode
	lastNode := &startingNode
	for i, n := range nums[1:] {
		newNode := ListNode{Value: n * multiplier, PreviousItem: lastNode}
		nodes[i+1] = &newNode
		(*lastNode).NextItem = &newNode
		lastNode = &newNode
	}
	lastNode.NextItem = &startingNode
	startingNode.PreviousItem = lastNode
	for i := 0; i < iterations; i++ {
		scramble(nodes)
	}
	return answer(nodes)
}

func answer(nodeList [](*ListNode)) int {
	startingPoint := nodeList[0]
	for startingPoint.Value != 0 {
		startingPoint = startingPoint.NextItem
	}
	result := int(0)
	nextPoint := startingPoint
	for i := 0; i < 3001; i++ {
		if i%1000 == 0 {
			result += nextPoint.Value
		}
		nextPoint = nextPoint.NextItem
	}
	return result
}

func scramble(nodeList []*ListNode) {
	nodeCount := len(nodeList)
	for _, node := range nodeList {
		shift := node.Value % int(nodeCount-1)
		if shift == 0 {
			continue
		}
		node.PreviousItem.NextItem = node.NextItem
		node.NextItem.PreviousItem = node.PreviousItem
		if shift < 0 {
			shift += nodeCount - 1
		}
		if shift > 0 {
			nextNode := node.NextItem
			for shift > 0 {
				nextNode = nextNode.NextItem
				shift--
			}
			nextNode.PreviousItem.NextItem = node
			node.PreviousItem = nextNode.PreviousItem
			node.NextItem = nextNode
			nextNode.PreviousItem = node
		}
	}
}
