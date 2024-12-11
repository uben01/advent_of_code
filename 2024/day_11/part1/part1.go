package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("day_11/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	list := make([]int, 0)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanWords)
	for scanner.Scan() {
		word := scanner.Text()

		num, err := strconv.Atoi(word)
		if err != nil {
			panic(err)
		}

		list = append(list, num)
	}

	for i := 0; i < 25; i++ {
		newList := make([]int, 0)
		for _, e := range list {
			newList = append(newList, blink(e)...)
		}
		list = newList
	}

	fmt.Printf("count: %d\n", len(list))
}

func blink(num int) []int {
	if num == 0 {
		return []int{1}
	}
	str := strconv.Itoa(num)
	if len(str)%2 == 0 {
		left := str[0 : len(str)/2]
		right := str[len(str)/2:]

		l, err := strconv.Atoi(left)
		if err != nil {
			panic(err)
		}
		r, err := strconv.Atoi(right)
		if err != nil {
			panic(err)
		}

		return []int{l, r}
	}

	return []int{num * 2024}

}
