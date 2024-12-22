package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("day_22/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	sum := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		sum += calculate(num)
	}

	fmt.Printf("final numers sum is: %d", sum)
}

func calculate(num int) int {
	for i := 0; i < 2000; i++ {
		n := num * 64
		num = mix(num, n)
		num = prune(num)

		n = num / 32
		num = mix(num, n)
		num = prune(num)

		n = num * 2048
		num = mix(num, n)
		num = prune(num)
	}
	return num
}

func prune(num int) int {
	return num % 16777216
}

func mix(secretNum, num int) int {
	return secretNum ^ num
}
