package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var (
	blocks       = make(map[rune][]string)
	currentTowel string
)

func main() {
	file, err := os.Open("day_19/input.txt")
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)
	scanner.Scan() // blocks
	line := scanner.Text()
	for _, block := range strings.Split(line, ", ") {
		firstChar := rune(block[0])
		if _, ok := blocks[firstChar]; !ok {
			blocks[firstChar] = make([]string, 0)
		}
		blocks[firstChar] = append(blocks[firstChar], block)
	}

	scanner.Scan() // empty line

	// towels
	towels := make([]string, 0)
	for scanner.Scan() {
		line = scanner.Text()

		if line != "" {
			towels = append(towels, line)
		}
	}
	file.Close()

	count := 0
	for y, towel := range towels {
		currentTowel = towel
		if makeFromBlocks() {
			count++
		}

		fmt.Printf("%d/%d\n", y, len(towels))
	}

	fmt.Printf("count: %d", count)
}

func makeFromBlocks() bool {
	desiredLength := len(currentTowel)

	queue := make([]string, 0)
	queue = append(queue, "")

	var current string
	for len(queue) > 0 {
		current, queue = queue[0], queue[1:]

		nextChar := rune(currentTowel[len(current)])
	outer:
		for _, block := range blocks[nextChar] {
			newTowel := fmt.Sprintf("%s%s", current, block)
			newLength := len(newTowel)

			if newLength > desiredLength {
				continue
			}

			if newLength == desiredLength {
				if newTowel == currentTowel {
					return true
				}
				continue
			}

			if currentTowel[0:newLength] != newTowel {
				continue
			}

			for _, q := range queue {
				if q == newTowel {
					continue outer
				}
			}
			queue = append(queue, newTowel)
		}
	}

	return false
}
