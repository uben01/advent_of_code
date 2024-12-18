package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

type coord struct {
	y, x int
	cost int
}

func (c coord) move(d coord) coord {
	return coord{c.y + d.y, c.x + d.x, c.cost + 1}
}

var (
	size        = 71   // example 7, input 71
	numOfBlocks = 1024 // example 12, input 1024

	directions = []coord{
		{0, 1, 1},  // right
		{0, -1, 1}, // left
		{1, 0, 1},  // up
		{-1, 0, 1}, // down
	}
)

func main() {
	file, err := os.Open("day_18/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	corruptedBlocks := make([]coord, 0)

	re := regexp.MustCompile(`\d+`)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		corruptedBlocks = append(corruptedBlocks, readCoords(re, line))
	}

	m := make([][]rune, size)
	for i := 0; i < size; i++ {
		m[i] = make([]rune, size)
		for j := 0; j < size; j++ {
			m[i][j] = '.'
		}
	}

	fallBlocks(m, corruptedBlocks[0:numOfBlocks])

	bfs(m, coord{0, 0, 0})
}

func bfs(m [][]rune, pos coord) {
	costs := make([][]int, size)
	for i := 0; i < size; i++ {
		costs[i] = make([]int, size)
		for j := 0; j < size; j++ {
			costs[i][j] = math.MaxInt
		}
	}
	costs[0][0] = 0

	queue := make([]coord, 0)
	queue = append(queue, pos)
	for len(queue) > 0 {
		pos, queue = queue[0], queue[1:]

		stepDijkstra(m, costs, &queue, pos)
	}

	fmt.Printf("cost to reach end: %d", costs[size-1][size-1])
}

func stepDijkstra(m [][]rune, cost [][]int, queue *[]coord, pos coord) {
	if pos.x < 0 || pos.y < 0 || pos.x >= size || pos.y >= size {
		return
	}
	if m[pos.y][pos.x] == '#' {
		return
	}
	if cost[pos.y][pos.x] < pos.cost {
		return
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
}

func fallBlocks(m [][]rune, blocks []coord) {
	for _, b := range blocks {
		m[b.y][b.x] = '#'
	}
}

func readCoords(re *regexp.Regexp, line string) coord {
	strs := re.FindAllString(line, 2)

	y, err := strconv.Atoi(strs[1])
	if err != nil {
		panic(err)
	}

	x, err := strconv.Atoi(strs[0])
	if err != nil {
		panic(err)
	}

	return coord{y, x, -1}
}
