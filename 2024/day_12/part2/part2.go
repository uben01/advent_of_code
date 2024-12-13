package main

import (
	"bufio"
	"fmt"
	"golang.org/x/exp/slices"
	"os"
)

type plant struct {
	top, bottom, left, right bool
	visited                  bool
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
			if plants[getKey(y, x)].visited {
				continue
			}
			region, aoi := calculateRegion(plants, y, x, 0, []string{})
			if region == 0 {
				continue
			}
			perimeter := calculatePerimeter(plants, aoi)

			sum += region * perimeter
			fmt.Printf("x: %d, y: %d, region: %d, perimeter: %d\n", x, y, region, perimeter)
		}
	}

	fmt.Printf("cost: %d\n", sum)
}

func calculatePerimeter(plants map[string]*plant, aoi []string) int {
	perimeter := 0

	// horizontal
	for y := 0; y < size; y++ {
		lastTop := true
		lastBottom := true
		for x := 0; x < size; x++ {
			key := getKey(y, x)
			if !slices.Contains(aoi, key) {
				lastTop = true
				lastBottom = true
				continue
			}
			p := plants[key]
			if !p.top && lastTop {
				perimeter++
			}
			if !p.bottom && lastBottom {
				perimeter++
			}
			lastTop = p.top
			lastBottom = p.bottom
		}
	}

	// vertical
	for x := 0; x < size; x++ {
		lastLeft := true
		lastRight := true
		for y := 0; y < size; y++ {
			key := getKey(y, x)
			if !slices.Contains(aoi, key) {
				lastLeft = true
				lastRight = true
				continue
			}
			p := plants[key]
			if !p.left && lastLeft {
				perimeter++
			}
			if !p.right && lastRight {
				perimeter++
			}
			lastLeft = p.left
			lastRight = p.right
		}
	}

	return perimeter
}

func calculateRegion(m map[string]*plant, y, x, region int, keys []string) (int, []string) {
	key := getKey(y, x)
	p := m[key]
	if p.visited {
		return region, keys
	}

	keys = append(keys, key)

	p.visited = true
	region += 1

	if p.top {
		region, keys = calculateRegion(m, y-1, x, region, keys)
	}
	if p.bottom {
		region, keys = calculateRegion(m, y+1, x, region, keys)
	}
	if p.left {
		region, keys = calculateRegion(m, y, x-1, region, keys)
	}
	if p.right {
		region, keys = calculateRegion(m, y, x+1, region, keys)
	}

	return region, keys
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
