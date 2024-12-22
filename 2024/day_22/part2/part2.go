package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

var m = make(map[string]int)

func main() {
	file, err := os.Open("day_22/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}

		calculate(num)
	}

	maxVal := math.MinInt
	for _, value := range m {
		if value > maxVal {
			maxVal = value
		}
	}

	fmt.Printf("most bananas to get: %d\n", maxVal)
}

func hash(arr []int) string {
	str := ""
	for _, e := range arr {
		str = fmt.Sprintf("%s;%d", str, e)
	}

	return str
}

func calculate(num int) {
	prevDiffs := make([]int, 0, 4)
	var maxDiffs [][]int

	var maxCost = math.MinInt
	var prevCost = cost(num)

	var mTemp = make(map[string]int)

	for i := 0; i < 2000; i++ {
		num = step(num)

		currentCost := cost(num)

		diff := currentCost - prevCost
		if len(prevDiffs) == 4 {
			prevDiffs = prevDiffs[1:]
		}
		prevDiffs = append(prevDiffs, diff)

		if currentCost == maxCost {
			diffs := make([]int, len(prevDiffs))
			copy(diffs, prevDiffs)
			maxDiffs = append(maxDiffs, diffs)
		}

		if currentCost > maxCost {
			maxCost = currentCost
		}
		prevCost = currentCost

		if len(prevDiffs) == 4 {
			h := hash(prevDiffs)
			if _, ok := mTemp[h]; !ok {
				mTemp[h] = currentCost
			} // only the first occurrence is interesting
		}
	}

	for key, value := range mTemp {
		if _, ok := m[key]; !ok {
			m[key] = 0
		}
		m[key] += value
	}

}

func step(num int) int {
	n := num * 64
	num = mix(num, n)
	num = prune(num)

	n = num / 32
	num = mix(num, n)
	num = prune(num)

	n = num * 2048
	num = mix(num, n)
	num = prune(num)

	return num
}

func prune(num int) int {
	return num % 16777216
}

func mix(secretNum, num int) int {
	return secretNum ^ num
}

func cost(num int) int {
	s := fmt.Sprint(num)
	s = s[len(s)-1:]
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}

	return i
}
