package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"strconv"
)

func main() {
	file, err := os.Open("day_09/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanRunes)

	decoded := make([]string, 0)
	isNum := true
	num := 0
	for scanner.Scan() {
		t := scanner.Text()
		times, err := strconv.Atoi(t)
		if err != nil {
			panic(err)
		}

		str := "."
		if isNum {
			str = strconv.Itoa(num)
			num++
		}
		isNum = !isNum

		for range times {
			decoded = append(decoded, str)
		}

	}

	swapFun := reflect.Swapper(decoded)

outer:
	for i := len(decoded) - 1; i >= 0; i-- {
		if decoded[i] == "." {
			continue
		}

		for j := 0; j < i; j++ {
			if decoded[j] != "." {
				continue
			}

			swapFun(i, j)
			continue outer
		}

		break
	}

	sum := 0
	for i, item := range decoded {
		if item == "." {
			break
		}
		num, err := strconv.Atoi(item)
		if err != nil {
			panic(err)
		}

		sum += i * num
	}

	fmt.Printf("Sum: %d\n", sum)
}
