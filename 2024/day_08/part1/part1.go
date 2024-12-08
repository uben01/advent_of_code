package main

import (
	"bufio"
	"fmt"
	"os"
)

type location struct {
	x, y int
}

func sub(l1, l2 location) location {
	return location{
		l1.x - (l2.x - l1.x),
		l1.y - (l2.y - l1.y),
	}
}

func main() {
	file, err := os.Open("day_08/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	m := make(map[rune][]location)
	antinodes := make([][]rune, 0)

	y := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		antinodes = append(antinodes, make([]rune, 0))
		for x, char := range line {
			antinodes[y] = append(antinodes[y], '.')
			if char == '.' {
				continue
			}

			l := location{x, y}

			if _, ok := m[char]; !ok {
				m[char] = make([]location, 0)
			}
			m[char] = append(m[char], l)
		}

		y++
	}
	size := y

	for _, list := range m {
		for i1, e1 := range list {
			for i2, e2 := range list {
				if i2 == i1 {
					continue
				}

				diff := sub(e1, e2)
				if diff.x < 0 || diff.x >= size || diff.y < 0 || diff.y >= size {
					continue
				}

				antinodes[diff.y][diff.x] = '#'
			}
		}
	}

	sum := 0
	for _, line := range antinodes {
		for _, char := range line {
			if char == '#' {
				sum++
			}
		}
	}

	fmt.Printf("Sum: %d", sum)
}
