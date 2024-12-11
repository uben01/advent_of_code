package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var cache map[string]int

func main() {
	// Done from input to 40 txt the first n iterations
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

	cache = make(map[string]int)

	sum := 0
	for _, e := range list {
		sum += blink(e, 75)
	}

	fmt.Printf("count: %d\n", sum)
}

func blink(num, times int) int {
	key := fmt.Sprintf("%d;%d", num, times)
	if count, ok := cache[key]; ok {
		return count
	}

	var ret int
	var str string
	defer (func() {
		cache[key] = ret
	})()

	if times == 0 {
		ret = 1
		goto end
	}

	if num == 0 {
		ret = blink(1, times-1)
		goto end
	}
	str = strconv.Itoa(num)
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
		ret = blink(l, times-1) + blink(r, times-1)
		goto end
	}

	ret = blink(2024*num, times-1)

end:
	return ret
}
