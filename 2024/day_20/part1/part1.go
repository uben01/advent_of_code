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
	size       int
	start, end coord
	minGain    = 100 // 1 = example, 100 = input

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

		indexE := strings.IndexRune(line, 'E')
		if indexE != -1 {
			end = coord{y, indexE, 0}
		}

		m = append(m, []rune(line))
		y++
	}
	size = y

	costs := bfs(m)

	count := cheat(costs)

	fmt.Printf("possible cheats: %d", count)
}

func cheat(costs [][]int) int {
	count := 0
	for y := 1; y < size-1; y++ {
		for x := 1; x < size-1; x++ {
			// starting point is y,x
			startCost := costs[y][x]

			if startCost == math.MaxInt {
				continue
			}

			// horizontal 1 or 2
			for x2 := x + 2; x2 <= x+3; x2++ {
				if costs[y][x2-1] != math.MaxInt {
					break
				}
				if isOutOfMap(y, x2) {
					break
				}
				endCost := costs[y][x2]
				if endCost == math.MaxInt {
					continue
				}
				if absDiff(startCost, endCost) > minGain {
					count++
				}
			}

			// vertical 1 or 2
			for y2 := y + 2; y2 <= y+3; y2++ {
				if costs[y2-1][x] != math.MaxInt {
					break
				}

				if isOutOfMap(y2, x) {
					break
				}
				endCost := costs[y2][x]
				if endCost == math.MaxInt {
					continue
				}

				if absDiff(startCost, endCost) > minGain {
					count++
				}
			}
		}
	}

	return count
}

func absDiff(a, b int) int {
	return int(math.Abs(float64(a - b)))
}

// out if y and x ALWAYS >= 0
func isOutOfMap(y, x int) bool {
	return y >= size || x >= size
}

func bfs(m [][]rune) [][]int {
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

		stepDijkstra(m, costs, &queue, pos)
	}

	return costs
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

	return
}
