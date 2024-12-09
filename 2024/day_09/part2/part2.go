package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type fileType struct {
	start, len, id int
}

func main() {
	f, err := os.Open("day_09/input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanRunes)

	files := make([]fileType, 0)

	decoded := make([]string, 0)
	isNum := true
	num := 0
	position := 0
	for scanner.Scan() {
		t := scanner.Text()
		times, err := strconv.Atoi(t)
		if err != nil {
			panic(err)
		}

		str := "."
		if isNum {
			str = strconv.Itoa(num)
			files = append(files, fileType{position, times, num})
			num++
		}

		for range times {
			decoded = append(decoded, str)
		}

		isNum = !isNum
		position += times
	}

outer:
	for i := len(files) - 1; i >= 0; i-- {
		fileToPlace := &files[i]

		space := 0
		for pos, element := range decoded {
			if pos > fileToPlace.start {
				continue outer
			}

			if element != "." {
				space = 0
				continue
			}
			space++

			if space >= fileToPlace.len {
				start := pos - space + 1
				for j := start; j < start+fileToPlace.len; j++ {
					decoded[j] = strconv.Itoa(fileToPlace.id)
				}
				for j := fileToPlace.start; j < fileToPlace.start+fileToPlace.len; j++ {
					decoded[j] = "."
				}
				fileToPlace.start = start
				continue outer
			}
		}
	}

	sum := 0
	for _, e := range files {
		for i := e.start; i < e.start+e.len; i++ {
			sum += i * e.id
		}
	}
	fmt.Printf("sum: %d", sum)
}
