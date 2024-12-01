package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	listLeft := []float64{}
	listRight := []float64{}
	var left float64
	var right float64

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, "   ")
		left, err = strconv.ParseFloat(split[0], 64)
		right, err = strconv.ParseFloat(split[1], 64)

		if err != nil {
			panic(err)
		}

		listLeft = append(listLeft, left)
		listRight = append(listRight, right)
	}

	sort.Float64s(listLeft)
	sort.Float64s(listRight)

	sum := .0
	var i int
	for i, left = range listLeft {
		right = listRight[i]
		dist := math.Abs(left - right)
		sum += dist
	}

	fmt.Printf("Sum: %.0f", sum)
}
