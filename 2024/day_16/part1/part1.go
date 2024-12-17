package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

var (
	size int
)

type reindeer struct {
	y, x             int
	facingY, facingX int
}

func move(r reindeer, dy, dx int) reindeer {
	r2 := r
	r2.y += dy
	r2.x += dx

	return r2
}

func turn(r reindeer, turnRight bool) reindeer {
	r2 := r
	if turnRight {
		if r.facingX == 1 {
			r2.facingX = 0
			r2.facingY = -1
		} else if r.facingX == -1 {
			r2.facingX = 0
			r2.facingY = 1
		} else if r.facingY == 1 {
			r2.facingY = 0
			r2.facingX = 1
		} else {
			r2.facingY = 0
			r2.facingX = -1
		}
	} else {
		if r.facingX == 1 {
			r2.facingX = 0
			r2.facingY = 1
		} else if r.facingX == -1 {
			r2.facingX = 0
			r2.facingY = -1
		} else if r.facingY == 1 {
			r2.facingY = 0
			r2.facingX = -1
		} else {
			r2.facingY = 0
			r2.facingX = 1
		}
	}

	return r2
}

func main() {
	file, err := os.Open("day_16/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	m := make([][]rune, 0)
	var r reindeer

	scanner := bufio.NewScanner(file)
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		m = append(m, []rune(line))

		pos := strings.IndexRune(line, 'S')
		if pos != -1 {
			r = reindeer{y, pos, 0, 1}
		}

		y++
	}
	size = y

	visited := make([][]int, size)
	for i := range visited {
		visited[i] = make([]int, size)
		for j := range visited[i] {
			visited[i][j] = math.MaxInt
		}
	}
	minDist := dfs(m, visited, r, 0, 0, true)
	fmt.Printf("Minimum distance: %d\n", minDist)
}

func dfs(m [][]rune, visited [][]int, r reindeer, cost int, turns int, moved bool) int {
	if m[r.y][r.x] == '#' {
		return math.MaxInt
	}

	if m[r.y][r.x] == 'E' {
		return realCost(cost, turns)
	}

	if moved {
		if visited[r.y][r.x] <= realCost(cost, turns) {
			return math.MaxInt
		}
		visited[r.y][r.x] = realCost(cost, turns)
	}

	a := dfs(m, visited, move(r, r.facingY, r.facingX), cost+1, turns, true)
	if moved {
		b := dfs(m, visited, turn(r, true), cost, turns+1, false)
		c := dfs(m, visited, turn(r, false), cost, turns+1, false)

		return min(a, b, c)
	}

	return a
}

func realCost(cost, turns int) int {
	return cost + (turns * 1000)
}
