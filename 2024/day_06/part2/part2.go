package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("day_06/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// example: 10, input 130
	size := 130
	m := make([][]rune, size)

	var startX, startY int

	{
		i := 0
		scanner := bufio.NewScanner(file)
		for scanner.Scan() {
			line := scanner.Text()

			x := strings.Index(line, "^")
			if x != -1 {
				startX = x
				startY = i
			}

			m[i] = []rune(line)
			i++
		}
	}

	possibleNewObs := 0
	for i := range m {
		for j := range m[i] {
			if m[i][j] == '#' || (i == startY && j == startX) {
				continue
			}

			m[i][j] = '#'

			newObsIsGood := stepUntilOut(m, size, startY, startX)
			if newObsIsGood {
				possibleNewObs++
			}
			m[i][j] = '.'
		}
	}

	fmt.Printf("Possible new obstructions: %d \n", possibleNewObs)
}

func stepUntilOut(m [][]rune, size, x, y int) bool {
	dx := -1
	dy := 0

	visited := make(map[string]bool)

	var out bool
	for {
		out, x, y = stepToDirection(m, size, x, y, dx, dy)
		if out {
			return false
		}

		h := hash(x, y, dx, dy)
		if visited[h] {
			return true
		}
		visited[h] = true

		dx, dy = newDirection(dx, dy)
	}
}

func hash(x, y, dx, dy int) string {
	return fmt.Sprintf("%d;%d;%d;%d", x, y, dx, dy)
}

func newDirection(dx, dy int) (int, int) {
	if dx == -1 {
		return 0, 1
	}
	if dy == 1 {
		return 1, 0
	}
	if dx == 1 {
		return 0, -1
	}
	return -1, 0
}

func stepToDirection(m [][]rune, size, x, y, dx, dy int) (bool, int, int) {
	for {
		if x+dx < 0 || x+dx >= size || y+dy < 0 || y+dy >= size {
			return true, x, y
		}

		if m[x+dx][y+dy] == '#' {
			return false, x, y
		}

		x += dx
		y += dy
	}
}
