package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	file, err := os.Open("day_03/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	full := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)
	parts := regexp.MustCompile(`\d{1,3}`)

	sum := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		muls := full.FindAllString(line, -1)
		for _, mul := range muls {
			nums := parts.FindAllString(mul, 2)

			var first, second int
			first, err = strconv.Atoi(nums[0])
			second, err = strconv.Atoi(nums[1])

			sum += first * second
		}
	}

	fmt.Printf("Sum: %d", sum)
}
