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

type times struct {
	s string
	t int
}

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
	for _, towel := range towels {
		currentTowel = towel
		currentCount := makeFromBlocks()
		count += currentCount
	}

	fmt.Printf("count: %d", count)
}

func makeFromBlocks() int {
	possibleOptions := 0

	desiredLength := len(currentTowel)

	queue := make([]times, 0)
	queue = append(queue, times{"", 1})

	var current times
	for len(queue) > 0 {
		current, queue = queue[0], queue[1:]

		nextChar := rune(currentTowel[len(current.s)])
	outer:
		for _, block := range blocks[nextChar] {
			newTowel := fmt.Sprintf("%s%s", current.s, block)
			newLength := len(newTowel)

			if newLength > desiredLength {
				continue
			}

			if newLength == desiredLength {
				if newTowel == currentTowel {
					possibleOptions += current.t
				}
				continue
			}

			if currentTowel[0:newLength] != newTowel {
				continue
			}

			for i := 0; i < len(queue); i++ {
				if queue[i].s == newTowel {
					queue[i].t += current.t
					continue outer
				}
			}
			queue = append(queue, times{newTowel, current.t})
		}
	}

	return possibleOptions
}
