package helpers

import (
	"fmt"
	"go/types"
	"os"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

type Direction int
type Arithmetic interface {
	int | int8 | int16 | int32 | int64 | uint | uint8 | uint16 | uint32 | uint64 | float32 | float64
}

type Set[T comparable] map[T]types.Nil

const (
	NorthWest Direction = iota
	North
	NorthEast
	East
	SouthEast
	South
	SouthWest
	West
)

func (d *Direction) String() string {
	return [...]string{"Northwest", "North", "NorthEast", "East", "Southeast", "South", "SouthWest", "West"}[*d]
}

type Particle[T Arithmetic] struct {
	Coordinate Coordinate[T]
	Direction  Direction
}

type Coordinate[T Arithmetic] struct {
	X T
	Y T
}

type CoordinateList[T Arithmetic] []Coordinate[T]

func (c *Coordinate[T]) String() string {
	return fmt.Sprintf("(%v,%v)", c.X, c.Y)
}

func (c *Coordinate[T]) Add(o Coordinate[T]) Coordinate[T] {
	return Coordinate[T]{
		X: c.X + o.X,
		Y: c.Y + o.Y,
	}
}

func (c *Coordinate[T]) Subtract(o Coordinate[T]) Coordinate[T] {
	return Coordinate[T]{
		X: c.X - o.X,
		Y: c.Y - o.Y,
	}
}

func (c *Coordinate[T]) ManhattanDistance(o Coordinate[T]) T {
	xDistance := c.X - o.X
	if xDistance < 0 {
		xDistance = 0 - xDistance
	}
	yDistance := c.Y - o.Y
	if yDistance < 0 {
		yDistance = 0 - yDistance
	}

	return xDistance + yDistance
}

func (c *Coordinate[T]) Neighbours() CoordinateList[T] {
	return CoordinateList[T]{
		Coordinate[T]{X: c.X - 1, Y: c.Y},
		Coordinate[T]{X: c.X + 1, Y: c.Y},
		Coordinate[T]{X: c.X, Y: c.Y - 1},
		Coordinate[T]{X: c.X, Y: c.Y + 1},
	}
}

func (c *Coordinate[T]) ExtendedNeighbours() CoordinateList[T] {
	return CoordinateList[T]{
		Coordinate[T]{X: c.X - 1, Y: c.Y - 1},
		Coordinate[T]{X: c.X - 1, Y: c.Y},
		Coordinate[T]{X: c.X - 1, Y: c.Y + 1},
		Coordinate[T]{X: c.X, Y: c.Y - 1},
		Coordinate[T]{X: c.X, Y: c.Y + 1},
		Coordinate[T]{X: c.X + 1, Y: c.Y - 1},
		Coordinate[T]{X: c.X + 1, Y: c.Y},
		Coordinate[T]{X: c.X + 1, Y: c.Y + 1},
	}
}

func (c *Coordinate[T]) HexNeighbours() CoordinateList[T] {
	return CoordinateList[T]{
		Coordinate[T]{X: c.X - 2, Y: c.Y},
		Coordinate[T]{X: c.X + 2, Y: c.Y},

		Coordinate[T]{X: c.X + 1, Y: c.Y - 1},
		Coordinate[T]{X: c.X + 1, Y: c.Y + 1},

		Coordinate[T]{X: c.X - 1, Y: c.Y - 1},
		Coordinate[T]{X: c.X - 1, Y: c.Y + 1},
	}
}
func (c *Coordinate[T]) ManhattanCircle(radius T) CoordinateList[T] {
	var circle CoordinateList[T]
	var i T
	for i = 0; i < radius; i++ {
		circle = append(circle, Coordinate[T]{X: c.X - radius + i, Y: c.Y + i})
		circle = append(circle, Coordinate[T]{X: c.X + i, Y: c.Y + radius - i})
		circle = append(circle, Coordinate[T]{X: c.X + radius - i, Y: c.Y - i})
		circle = append(circle, Coordinate[T]{X: c.X - i, Y: c.Y - radius + i})
	}
	return circle
}

func (cl *CoordinateList[T]) Len() int {
	return len(*cl)
}

func (cl *CoordinateList[T]) Contains(loc Coordinate[T]) bool {
	for _, coord := range *cl {
		if coord == loc {
			return true
		}
	}
	return false
}

func (cl *CoordinateList[T]) Less(i, j int) bool {
	if (*cl)[i].Y < (*cl)[j].Y {
		return true
	} else if (*cl)[i].Y > (*cl)[j].Y {
		return false
	}
	return (*cl)[i].X < (*cl)[j].X
}

func (cl *CoordinateList[T]) Swap(i, j int) {
	(*cl)[i], (*cl)[j] = (*cl)[j], (*cl)[i]
}

func Sum[T Arithmetic](arr []T) T {
	var sum T = 0
	for _, val := range arr {
		sum += val
	}
	return sum
}

type Coordinate3D[T Arithmetic] struct {
	X T
	Y T
	Z T
}

type Coordinate3DList[T Arithmetic] []Coordinate3D[T]

func (c *Coordinate3D[T]) String() string {
	return fmt.Sprintf("(%v,%v, %v)", c.X, c.Y, c.Z)
}

func (c *Coordinate3D[T]) Add(o Coordinate3D[T]) Coordinate3D[T] {
	return Coordinate3D[T]{
		X: c.X + o.X,
		Y: c.Y + o.Y,
		Z: c.Z + o.Z,
	}
}

func (c *Coordinate3D[T]) Subtract(o Coordinate3D[T]) Coordinate3D[T] {
	return Coordinate3D[T]{
		X: c.X - o.X,
		Y: c.Y - o.Y,
		Z: c.Z - o.Z,
	}
}

func (c *Coordinate3D[T]) ManhattanDistance(o *Coordinate3D[T]) T {
	xDistance := c.X - o.X
	if xDistance < 0 {
		xDistance = 0 - xDistance
	}
	yDistance := c.Y - o.Y
	if yDistance < 0 {
		yDistance = 0 - yDistance
	}
	zDistance := c.Z - o.Z
	if zDistance < 0 {
		zDistance = 0 - zDistance
	}
	return xDistance + yDistance + zDistance
}

func (c *Coordinate3D[T]) Neighbours() Coordinate3DList[T] {
	return Coordinate3DList[T]{
		Coordinate3D[T]{X: c.X - 1, Y: c.Y, Z: c.Z},
		Coordinate3D[T]{X: c.X + 1, Y: c.Y, Z: c.Z},
		Coordinate3D[T]{X: c.X, Y: c.Y - 1, Z: c.Z},
		Coordinate3D[T]{X: c.X, Y: c.Y + 1, Z: c.Z},
		Coordinate3D[T]{X: c.X, Y: c.Y, Z: c.Z - 1},
		Coordinate3D[T]{X: c.X, Y: c.Y, Z: c.Z + 1},
	}
}

func ReadFileToLines(filename string) ([]string, error) {
	return ReadFileToRecords(filename, "\n")
}

func ReadFileToRecords(filename string, separator string) ([]string, error) {
	fileContent, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	return strings.Split(string(fileContent), separator), nil
}

func ReadFileToString(filename string) (string, error) {
	fileContent, err := os.ReadFile(filename)
	if err != nil {
		return "", err
	}
	return string(fileContent), nil
}

func ReadFileToNumberList[T Arithmetic](filename string) ([]T, error) {
	lines, err := ReadFileToLines(filename)
	if err != nil {
		return nil, err
	}
	numbers := make([]T, len(lines))
	for n, number := range lines {
		parsed, err := strconv.Atoi(number)
		if err != nil {
			return nil, err
		}
		numbers[n] = T(parsed)
	}
	return numbers, nil
}

func ReadFileToNumberGrid(filename string) (map[Coordinate[int]]int, error) {
	lines, err := ReadFileToLines(filename)
	if err != nil {
		return nil, err
	}
	grid := make(map[Coordinate[int]]int)
	for y, line := range lines {
		for x, c := range line {
			grid[Coordinate[int]{x, y}] = int(c) - 48
		}
	}
	return grid, nil
}
func ReadFileToRuneGrid(filename string) (map[Coordinate[int]]rune, error) {
	lines, err := ReadFileToLines(filename)
	if err != nil {
		return nil, err
	}
	grid := make(map[Coordinate[int]]rune)
	for y, line := range lines {
		for x, c := range line {
			grid[Coordinate[int]{x, y}] = c
		}
	}
	return grid, nil
}

func CreateSet[T comparable](iterable []T) Set[T] {
	newSet := make(Set[T])
	for _, item := range iterable {
		newSet.Insert(item)
	}
	return newSet
}

func (s Set[T]) Insert(val T) {
	s[val] = types.Nil{}
}

func (s Set[T]) Remove(val T) {
	delete(s, val)
}

func (s Set[T]) Contains(val T) bool {
	_, ok := s[val]
	return ok
}

func (s Set[T]) Union(other Set[T]) Set[T] {
	union := make(Set[T])
	for key := range s {
		union[key] = types.Nil{}
	}
	for k := range other {
		union[k] = types.Nil{}
	}
	return union
}

func (s *Set[T]) String() string {
	keys := make([]string, 0)
	for key := range *s {
		keys = append(keys, fmt.Sprintf("%v", key))
	}
	return "[" + strings.Join(keys, ",") + "]"
}

func (s Set[T]) Intersection(other Set[T]) Set[T] {
	intersection := make(Set[T])
	if len(s) > len(other) {
		for key := range other {
			if _, ok := s[key]; ok {
				intersection.Insert(key)
			}
		}
	} else {
		for k := range s {
			if _, ok := other[k]; ok {
				intersection.Insert(k)
			}
		}
	}
	return intersection
}

func (s Set[T]) Difference(other Set[T]) Set[T] {
	difference := make(Set[T])
	for item := range s {
		if !other.Contains(item) {
			difference.Insert(item)
		}
	}
	return difference
}

func LetterScore(c uint8) uint8 {
	if c >= 65 && c <= 90 {
		return c - 38
	} else if c >= 97 && c <= 122 {
		return c - 96
	}
	return 0
}

type Stack[T comparable] []T

func (s *Stack[T]) Push(item T) {
	*s = append(*s, item)
}

func (s *Stack[T]) Pop() (T, error) {
	if len(*s) == 0 {
		var empty T
		return empty, fmt.Errorf("attempted to pop empty stack")
	} else {
		index := len(*s) - 1
		element := (*s)[index]
		*s = (*s)[:index]
		return element, nil
	}
}

func Combinations[T comparable](values []T, length int) [][]T {
	// maxLen := Factorial[int](len(values))
	fmt.Println("Max size:", 1500)
	partials := make(chan ([]T), Factorial(len(values)-1))
	combinations := make([][]T, 0, Factorial[int](length))
	for _, start := range values {
		partials <- []T{start}
	}

	for len(partials) > 0 {
		partial := <-partials
		fmt.Println("Partial:", partial)

		for _, val := range values {
			if !slices.Contains(partial, val) {
				nextPartial := make([]T, 0, len(partial)+1)
				nextPartial = append(nextPartial, partial...)
				nextPartial = append(nextPartial, val)
				fmt.Println("Next combination:", nextPartial)
				if len(nextPartial) == length {
					combinations = append(combinations, nextPartial)
				} else {
					partials <- nextPartial
				}
			}
		}
	}
	return combinations
}

func Factorial[T Arithmetic](val T) T {
	if val <= 1 {
		return 1
	}
	return val * Factorial(val-1)
}

type PrioritisedItem[T comparable] struct {
	priority  int
	GameState T
	index     int
}

func NewPrioritisedItem[T comparable](priority int, state T) PrioritisedItem[T] {
	return PrioritisedItem[T]{priority: priority, GameState: state}
}

type MinHeap[T comparable] []*PrioritisedItem[T]

func (h MinHeap[T]) Len() int {
	return len(h)
}

func (h MinHeap[T]) Less(i, j int) bool {
	return h[i].priority < h[j].priority
}

func (h MinHeap[T]) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
	h[i].index = i
	h[j].index = j
}

func (h *MinHeap[T]) Push(x any) {
	n := len(*h)
	item := x.(*PrioritisedItem[T])
	item.index = n
	*h = append(*h, item)
}

func (pq *MinHeap[T]) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}
