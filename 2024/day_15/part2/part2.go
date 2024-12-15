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

	size = 50 * 2 // example 10*2, input 50*2

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
				posX = pos * 2
			}

			m = append(m, make([]rune, 0))
			for _, e := range line {
				switch e {
				case '#':
					m[y] = append(m[y], '#', '#')
				case 'O':
					m[y] = append(m[y], '[', ']')
				case '.':
					m[y] = append(m[y], '.', '.')
				case '@':
					m[y] = append(m[y], '@', '.')
				}
			}
		} else {
			commands = append(commands, []rune(line)...)
		}
		y++
	}

	//printMap(m)
	for _, command := range commands {
		//fmt.Println("================")
		//fmt.Println(string(command))
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
		//printMap(m)
	}

	sum := 0
	for y := 0; y < size/2; y++ {
		for x := 0; x < size; x++ {
			if m[y][x] == '[' {
				sum += (100 * y) + x
			}
		}
	}

	fmt.Printf("sum: %d", sum)
}

func move(m [][]rune, posX, posY, dX, dY int) (int, int) {
	newX := posX + dX
	newY := posY + dY

	if m[newY][newX] == '#' {
		return posX, posY
	}

	if m[newY][newX] == '[' {
		if !checkMove(m, newX, newY, dX, dY) {
			return posX, posY
		}
		if !checkMove(m, newX+1, newY, dX, dY) {
			return posX, posY
		}
		if dY != 0 {
			move(m, newX, newY, dX, dY)
			move(m, newX+1, newY, dX, dY)
		} else {
			move(m, newX, newY, dX, dY)
		}
	} else if m[newY][newX] == ']' {
		if !checkMove(m, newX, newY, dX, dY) {
			return posX, posY
		}
		if !checkMove(m, newX-1, newY, dX, dY) {
			return posX, posY
		}

		if dY != 0 {
			move(m, newX, newY, dX, dY)
			move(m, newX-1, newY, dX, dY)
		} else {
			move(m, newX, newY, dX, dY)
		}
	}

	m[newY][newX] = m[posY][posX]
	m[posY][posX] = '.'
	return newX, newY
}

func checkMove(m [][]rune, posX, posY, dX, dY int) bool {
	newX := posX + dX
	newY := posY + dY
	if m[newY][newX] == '#' {
		return false
	}

	if m[newY][newX] == '.' {
		return true
	}

	if dY != 0 {
		if m[newY][newX] == '[' {
			return checkMove(m, newX, newY, dX, dY) && checkMove(m, newX+1, newY, dX, dY)
		} else if m[newY][newX] == ']' {
			return checkMove(m, newX, newY, dX, dY) && checkMove(m, newX-1, newY, dX, dY)
		}
	} else {
		return checkMove(m, newX, newY, dX, dY)
	}

	return true
}

func printMap(m [][]rune) {
	for i, row := range m {
		fmt.Printf("%d ", i)
		for _, cell := range row {
			fmt.Print(string(cell))
		}
		fmt.Println()
	}
}
