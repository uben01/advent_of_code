package main

import (
	"bufio"
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

	listLeft := []int{}
	listRight := []int{}
	var left int
	var right int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, "   ")
		left, err = strconv.Atoi(split[0])
		right, err = strconv.Atoi(split[1])

		if err != nil {
			panic(err)
		}

		listLeft = append(listLeft, left)
		listRight = append(listRight, right)
	}

	sort.Ints(listLeft)
	sort.Ints(listRight)

	sum := 0
	for i, left := range listLeft {
		right := listRight[i]
		dist := math.Abs(float64(left - right))
		sum += int(dist)
	}

	println("Total dist: ", sum)
}
