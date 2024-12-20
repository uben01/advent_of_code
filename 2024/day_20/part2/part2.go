package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type coord struct {
	y, x int
	cost int
}

func (c coord) move(d coord) coord {
	return coord{c.y + d.y, c.x + d.x, c.cost + 1}
}

var (
	size    int
	start   coord
	minGain = 100 // 50 = example, 100 = input
	count   = 0

	directions = []coord{
		{0, 1, 1},  // right
		{0, -1, 1}, // left
		{1, 0, 1},  // up
		{-1, 0, 1}, // down
	}
)

func main() {
	file, err := os.Open("day_20/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	m := make([][]rune, 0)

	y := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		indexS := strings.IndexRune(line, 'S')
		if indexS != -1 {
			start = coord{y, indexS, 0}
		}

		m = append(m, []rune(line))
		y++
	}
	size = y

	costs := bfsToEnd(m)
	cheat(m, costs)

	fmt.Printf("sum: %d\n", count)
}

func cheat(m [][]rune, costs [][]int) {
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			if m[y][x] == '#' {
				continue
			}
			inDistance(m, costs, coord{y, x, 0})
		}
	}
}

func inDistance(m [][]rune, costs [][]int, pos coord) {
	nowCost := costs[pos.y][pos.x]
	for i := -20; i <= 20; i++ {
		for j := -20; j <= 20; j++ {
			absI := int(math.Abs(float64(i)))
			absJ := int(math.Abs(float64(j)))
			if absI+absJ > 20 {
				continue
			}

			posY := pos.y + i
			posX := pos.x + j
			if posY < 0 || posY >= size || posX < 0 || posX >= size {
				continue
			}
			if m[posY][posX] == '#' {
				continue
			}

			gain := (costs[posY][posX] - nowCost) - absI - absJ
			if gain >= minGain {
				count++
			}
		}
	}
}

func bfsToEnd(m [][]rune) [][]int {
	costs := make([][]int, size)
	for i := 0; i < size; i++ {
		costs[i] = make([]int, size)
		for j := 0; j < size; j++ {
			costs[i][j] = math.MaxInt
		}
	}

	queue := make([]coord, 0)
	queue = append(queue, start)

	var pos coord
	for len(queue) > 0 {
		pos, queue = queue[0], queue[1:]

		stepBFS(m, costs, &queue, pos)
	}

	return costs
}

func stepBFS(m [][]rune, cost [][]int, queue *[]coord, pos coord) bool {
	if pos.x < 0 || pos.y < 0 || pos.x >= size || pos.y >= size {
		return false
	}
	if m[pos.y][pos.x] == '#' {
		return false
	}
	if cost[pos.y][pos.x] < pos.cost {
		return false
	}

	cost[pos.y][pos.x] = pos.cost

outer:
	for _, direction := range directions {
		newPos := pos.move(direction)
		for _, q := range *queue {
			if q == newPos {
				continue outer
			}
		}
		*queue = append(*queue, newPos)
	}

	return true
}
