package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type coordinate struct {
	x, y float64
	cost float64
}

func main() {
	// you have to add a new line to the end of the file in order to be correct
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

	var totalCost float64 = 0
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

			result := calculateCostWithEquations(aC, bC, prizeC)
			subTotal := result
			totalCost += subTotal
		}
		c++
	}

	fmt.Printf("cost: %d", int(totalCost))
}

func calculateCostWithEquations(a, b, p coordinate) float64 {
	A := ((p.x * b.y) - (p.y * b.x)) / (a.x*b.y - a.y*b.x)
	B := ((a.x * p.y) - (a.y * p.x)) / (a.x*b.y - a.y*b.x)

	if !isWholeNumber(A) || !isWholeNumber(B) {
		return 0
	}

	if (a.x*A)+(b.x*B) == p.x && (a.y*A)+(b.y*B) == p.y {
		return A*a.cost + B*b.cost
	}

	return 0
}

func isWholeNumber(n float64) bool {
	return n == float64(int(n))
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
	if id == 'P' {
		x += 10000000000000
		y += 10000000000000
	}

	return coordinate{float64(x), float64(y), float64(cost)}
}
