package main

import (
	"bufio"
	"fmt"
	"os"
)

type plant struct {
	top, bottom, left, right bool
	visited                  bool
}

func (p plant) perimeter() int {
	s := 4
	if p.top {
		s--
	}
	if p.bottom {
		s--
	}
	if p.left {
		s--
	}
	if p.right {
		s--
	}

	return s
}

var size int

func main() {
	file, err := os.Open("day_12/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	m := make([][]rune, 0)
	plants := make(map[string]*plant)
	{
		y := 0
		scanner := bufio.NewScanner(file)
		for scanner.Scan() {
			line := scanner.Text()

			m = append(m, []rune(line))
			y++
		}
		size = y
	}

	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			left := isSameAtCoordinate(m, m[y][x], y, x-1)
			right := isSameAtCoordinate(m, m[y][x], y, x+1)
			top := isSameAtCoordinate(m, m[y][x], y-1, x)
			bottom := isSameAtCoordinate(m, m[y][x], y+1, x)

			plants[getKey(y, x)] = &plant{top, bottom, left, right, false}
		}
	}

	sum := 0

	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			region, perimeter := walkFromPoint(plants, y, x, 0, 0)

			sum += region * perimeter
		}
	}

	fmt.Printf("cost: %d\n", sum)
}

func walkFromPoint(m map[string]*plant, y, x, region, perimeter int) (int, int) {
	key := getKey(y, x)
	p := m[key]
	if p.visited {
		return region, perimeter
	}

	region += 1
	perimeter += p.perimeter()
	p.visited = true

	if p.top {
		region, perimeter = walkFromPoint(m, y-1, x, region, perimeter)
	}
	if p.bottom {
		region, perimeter = walkFromPoint(m, y+1, x, region, perimeter)
	}
	if p.left {
		region, perimeter = walkFromPoint(m, y, x-1, region, perimeter)
	}
	if p.right {
		region, perimeter = walkFromPoint(m, y, x+1, region, perimeter)
	}

	return region, perimeter
}

func getKey(y, x int) string {
	return fmt.Sprintf("%d;%d", y, x)
}

func isSameAtCoordinate(m [][]rune, char rune, y, x int) bool {
	if y < 0 || x < 0 || y >= size || x >= size {
		return false
	}

	return m[y][x] == char
}
