package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("day_04/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// example.txt - 10, input.txt - 140
	lineCount := 140

	matrix := make([][]rune, lineCount)
	scanner := bufio.NewScanner(file)
	i := 0
	for scanner.Scan() {
		line := scanner.Text()

		matrix[i] = []rune(line)
		i++
	}

	xmasCount := 0
	for x := 0; x < lineCount; x++ {
		for y := 0; y < lineCount; y++ {
			xs := x + 2
			ys := y + 2
			str := wordFromPointToDirection(matrix, lineCount, x, y, xs, ys)
			if str != nil && match(str) {
				str2 := wordFromPointToDirection(matrix, lineCount, xs, y, x, ys)
				if str2 != nil && match(str2) {
					//fmt.Printf("%d:%d -> %d:%d\n", x, y, xs, y)
					xmasCount++
				}
			}
		}
	}

	fmt.Printf("X-MAS count is: %d", xmasCount)
}

func match(str *string) bool {
	return *str == "MAS" || *str == "SAM"
}

func change(i *int, inc bool, dec bool) {
	if inc {
		*i++
	}
	if dec {
		*i--
	}
}

func wordFromPointToDirection(matrix [][]rune, max, startX, startY, endX, endY int) *string {
	if endX < 0 || endY < 0 || startX >= max || startY >= max || endX >= max || endY >= max || startX < 0 || startY < 0 {
		return nil
	}

	incX := startX < endX
	decX := startX > endX
	incY := startY < endY
	decY := startY > endY

	str := ""
	x := startX
	y := startY
	for x != endX || y != endY {
		str += string(matrix[x][y])
		change(&x, incX, decX)
		change(&y, incY, decY)
	}
	str += string(matrix[x][y])

	return &str
}
