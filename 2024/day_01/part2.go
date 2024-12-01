package main

import (
	"bufio"
	"os"
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

	sum := 0
	for _, left := range listLeft {
		counter := 0
		for _, right := range listRight {
			if left == right {
				counter++
			}
		}
		sum += left * counter
	}

	println("Total dist: ", sum)
}
