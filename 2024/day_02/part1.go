package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	safeReports := 0

	scanner := bufio.NewScanner(file)
report:
	for scanner.Scan() {
		levels := strings.Split(scanner.Text(), " ")

		var prevLevel *int
		var increasing bool
		for i, level := range levels {
			level, err := strconv.Atoi(level)
			if err != nil {
				panic(err)
			}

			if prevLevel == nil {
				prevLevel = &level
				continue
			}

			if i == 1 {
				diff := math.Abs(float64(*prevLevel - level))
				if diff == 0 || diff > 3 {
					continue report
				}

				increasing = *prevLevel > level
				prevLevel = &level
				continue
			}

			if (*prevLevel > level) != increasing {
				continue report
			}

			diff := math.Abs(float64(*prevLevel - level))
			if diff == 0 || diff > 3 {
				continue report
			}

			prevLevel = &level
		}
		safeReports += 1
	}

	fmt.Printf("Safe report count is: %d", safeReports)
}
