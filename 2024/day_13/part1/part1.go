package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

type coordinate struct {
	x, y int
	cost int
}

func (c coordinate) push(x, y, cost int) (int, int, int) {
	return x + c.x, y + c.y, cost + c.cost
}

func (c coordinate) unpush(x, y, cost int) (int, int, int) {
	return x - c.x, y - c.y, cost - c.cost
}

func (c coordinate) isOver(x, y int) bool {
	return x > c.x || y > c.y
}

func (c coordinate) isEqual(x, y int) bool {
	return x == c.x && y == c.y
}

func main() {
	file, err := os.Open("day_13/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	c := 0
	var a string
	var b string
	var prize string
	scanner := bufio.NewScanner(file)

	totalCost := 0
	for scanner.Scan() {
		line := scanner.Text()

		if c%4 == 0 {
			a = line
		}
		if c%4 == 1 {
			b = line
		}
		if c%4 == 2 {
			prize = line
		}
		if c%4 == 3 {
			aC := getCoordinates(a, 'A')
			bC := getCoordinates(b, 'B')
			prizeC := getCoordinates(prize, 'P')

			subresult := calculate(aC, bC, prizeC)
			if subresult != nil {
				totalCost += *subresult
			}
		}
		c++
	}

	fmt.Printf("cost: %d", totalCost)
}

func calculate(a, b, p coordinate) *int {
	costA := calculateCostWithPreferredButton(a, b, p)
	costB := calculateCostWithPreferredButton(b, a, p)

	if costA == math.MaxInt && costB == math.MaxInt {
		return nil
	}

	result := int(math.Min(float64(costA), float64(costB)))
	return &result
}

func calculateCostWithPreferredButton(a, b, p coordinate) int {
	x := 0
	y := 0
	cost := 0

	aPush := 0
	bPush := 0
	for {
		if p.isEqual(x, y) {
			return cost
		}
		x, y, cost = a.push(x, y, cost)
		aPush++

		if p.isOver(x, y) {
			break
		}
	}

	for {
		x, y, cost = a.unpush(x, y, cost)
		aPush--

		if aPush < 0 {
			return math.MaxInt
		}

		for !p.isEqual(x, y) && !p.isOver(x, y) {
			x, y, cost = b.push(x, y, cost)
			bPush++
		}

		if p.isEqual(x, y) {
			return cost
		}
	}
}

func getCoordinates(str string, id rune) coordinate {
	r := regexp.MustCompile(`\d+`)
	c := r.FindAllString(str, -1)

	x, err := strconv.Atoi(c[0])
	if err != nil {
		panic(err)
	}
	y, err := strconv.Atoi(c[1])
	if err != nil {
		panic(err)
	}

	cost := 0
	if id == 'A' {
		cost = 3
	}
	if id == 'B' {
		cost = 1
	}

	return coordinate{x, y, cost}
}
