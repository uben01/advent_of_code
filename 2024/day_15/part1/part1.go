package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var size int

func main() {
	file, err := os.Open("day_15/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	m := make([][]rune, 0)
	commands := make([]rune, 0)
	var posX, posY int

	size = 50 // example 10, input 50

	isMap := true

	scanner := bufio.NewScanner(file)
	y := 0
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			isMap = false
			continue
		}

		if isMap {
			pos := strings.IndexRune(line, '@')
			if pos != -1 {
				posY = y
				posX = pos
			}

			m = append(m, []rune(line))
		} else {
			commands = append(commands, []rune(line)...)
		}
		y++
	}

	for _, command := range commands {
		switch command {
		case '<':
			posX, posY = move(m, posX, posY, -1, 0)
		case '>':
			posX, posY = move(m, posX, posY, 1, 0)
		case '^':
			posX, posY = move(m, posX, posY, 0, -1)
		case 'v':
			posX, posY = move(m, posX, posY, 0, 1)
		}
	}

	sum := 0
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			if m[y][x] == 'O' {
				sum += (100 * y) + x
			}
		}
	}

	fmt.Printf("sum: %d", sum)
}

func move(m [][]rune, posX, posY, dX, dY int) (int, int) {
	newX := posX + dX
	newY := posY + dY
	if newX < 0 || newX >= size || posY < 0 || posY >= size {
		return posX, posY
	}
	if m[newY][newX] == '#' {
		return posX, posY
	}

	if m[newY][newX] == 'O' {
		pushX, pushY := move(m, newX, newY, dX, dY)
		if pushX == newX && pushY == newY {
			return posX, posY
		}
	}

	m[newY][newX] = m[posY][posX]
	m[posY][posX] = '.'
	return newX, newY
}

func printMap(m [][]rune) {
	for _, row := range m {
		for _, cell := range row {
			fmt.Print(string(cell))
		}
		fmt.Println()
	}
}
