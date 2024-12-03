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

	full := regexp.MustCompile(`(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))`)
	mulParts := regexp.MustCompile(`\d{1,3}`)
	do := regexp.MustCompile(`do\(\)`)
	dont := regexp.MustCompile(`don't\(\)`)

	sum := 0
	shouldDo := true

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		expressions := full.FindAllString(line, -1)
		for _, exp := range expressions {
			if do.MatchString(exp) {
				shouldDo = true
				continue
			}
			if dont.MatchString(exp) {
				shouldDo = false
				continue
			}
			if !shouldDo {
				continue
			}

			nums := mulParts.FindAllString(exp, 2)

			var first, second int
			first, err = strconv.Atoi(nums[0])
			second, err = strconv.Atoi(nums[1])

			sum += first * second
		}
	}

	fmt.Printf("Sum: %d", sum)
}
