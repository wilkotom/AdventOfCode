package day07

import (
	"fmt"
	"log"
	"strings"

	"github.com/wilkotom/AoC2022/helpers"
)

func Day07() {
	lines, err := helpers.ReadFileToLines("inputs/07.txt")
	if err != nil {
		log.Fatal(err)
	}
	instructions := make(helpers.Stack[string], 0)
	for i := len(lines) - 1; i >= 0; i-- {
		instructions.Push(lines[i])
	}
	_, err = instructions.Pop()
	if err != nil {
		panic("Something weird happened")
	}
	tree := traverse(&instructions, "/")
	fmt.Printf("Part 1: %v\n", tree.part1())
	needed := 30000000 - (70000000 - tree.totalSize())
	fmt.Printf("Part 2: %v\n", tree.part2(needed))

}

func traverse(stack *helpers.Stack[string], name string) Directory {
	currentDir := Directory{
		Name:    name,
		Files:   make([]File, 0),
		Subdirs: make([]Directory, 0),
	}
	for {
		instr, err := stack.Pop()
		if err != nil || instr == "$ cd .." {
			return currentDir
		} else {
			var (
				name string
				size int64
			)
			reader := strings.NewReader(instr)
			_, err = fmt.Fscanf(reader, "$ cd %s", &name)
			if err == nil {
				currentDir.Subdirs = append(currentDir.Subdirs, traverse(stack, name))
			} else {
				_, err = fmt.Fscanf(reader, "%d %s", &size, &name)
				if err == nil {
					currentDir.Files = append(currentDir.Files, File{name, size})
				}
			}

		}

	}

}

type File struct {
	Name string
	Size int64
}

type Directory struct {
	Files   []File
	Name    string
	Subdirs []Directory
}

func (d *Directory) totalSize() int64 {
	total := int64(0)
	for _, file := range d.Files {
		total += file.Size
	}

	for _, dir := range d.Subdirs {
		total += dir.totalSize()
	}

	return total
}

func (d *Directory) part1() int64 {
	total := int64(0)
	if d.totalSize() <= 100000 {
		total += d.totalSize()
	}
	for _, s := range d.Subdirs {
		total += s.part1()
	}

	return total
}

func (d *Directory) part2(needed int64) int64 {
	size := d.totalSize()
	if size < needed {
		return 0
	}
	for _, s := range d.Subdirs {
		candidate := s.part2(needed)
		if candidate > 0 && candidate < size {
			size = candidate
		}
	}

	return size
}
