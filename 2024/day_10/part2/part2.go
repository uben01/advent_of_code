package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type location struct {
	y, x int
}

func main() {
	file, err := os.Open("day_10/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	size := 55
	m := make([][]int, size)

	start := make([]location, 0)

	y := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		m[y] = make([]int, size)
		for x, char := range line {
			s := string(char)
			num, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			m[y][x] = num

			if num == 0 {
				start = append(start, location{y, x})
			}
		}

		y++
	}

	sum := 0
	for _, s := range start {
		sum += findTopHeadsFromStart(m, s)
	}

	fmt.Printf("sum: %d", sum)
}

func findTopHeadsFromStart(m [][]int, start location) int {
	dxs := []int{1, 0, -1, 0}
	dys := []int{0, 1, 0, -1}

	tops := make(map[location]int)
	for i := 0; i < 4; i++ {
		step(m, start, tops, dxs[i], dys[i])
	}

	sum := 0
	for _, c := range tops {
		sum += c
	}

	return sum
}

func step(m [][]int, pos location, tops map[location]int, dx, dy int) {
	size := len(m)
	newX := pos.x + dx
	newY := pos.y + dy

	if newX < 0 || newX >= size || newY < 0 || newY >= size {
		return
	}
	oldValue := m[pos.y][pos.x]
	newValue := m[newY][newX]

	if newValue-1 != oldValue {
		return
	}
	newLoc := location{newY, newX}
	if newValue == 9 {
		if _, ok := tops[newLoc]; !ok {
			tops[newLoc] = 0
		}
		tops[newLoc]++
	}

	dxs := []int{1, 0, -1, 0}
	dys := []int{0, 1, 0, -1}
	for i := 0; i < 4; i++ {
		step(m, newLoc, tops, dxs[i], dys[i])
	}
}
