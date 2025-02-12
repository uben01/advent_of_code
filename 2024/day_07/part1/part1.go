package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("day_07/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	sum := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		nums := strings.Split(line, " ")
		res, err := strconv.Atoi(strings.Replace(nums[0], ":", "", 1))
		if err != nil {
			panic(err)
		}

		list := make([]int, 0)
		for _, num := range nums[1:] {
			n, err := strconv.Atoi(num)
			if err != nil {
				panic(err)
			}
			list = append(list, n)
		}

		if valid(res, list) {
			sum += res
		}
	}

	fmt.Printf("sum: %d", sum)
}

func valid(result int, numbers []int) bool {
	if mul(result, numbers, 1) {
		return true
	}

	if add(result, numbers, 0) {
		return true
	}

	return false
}

func add(result int, numbers []int, current int) bool {
	if len(numbers) == 0 {
		return result == current
	}

	tmp := numbers[0] + current
	numbers = numbers[1:]

	if mul(result, numbers, tmp) {
		return true
	}

	if add(result, numbers, tmp) {
		return true
	}

	return false
}

func mul(result int, numbers []int, current int) bool {
	if len(numbers) == 0 {
		return result == current
	}

	tmp := numbers[0] * current
	numbers = numbers[1:]

	if mul(result, numbers, tmp) {
		return true
	}

	if add(result, numbers, tmp) {
		return true
	}

	return false
}
