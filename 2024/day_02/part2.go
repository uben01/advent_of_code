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
reports:
	for scanner.Scan() {
		levels := strings.Split(scanner.Text(), " ")
		if skipNth(levels, nil) {
			safeReports++
			continue reports
		}

		for i, _ := range levels {
			if skipNth(levels, &i) {
				safeReports++
				continue reports
			}
		}
	}

	fmt.Printf("Safe report count is: %d", safeReports)
}

// skipNth skips the nth element in the levels slice and checks if the remaining elements are valid
func skipNth(levels []string, skip *int) bool {
	var prevLevel *int
	var increasing bool

	zerothSkipped := false
	for i, level := range levels {
		if skip != nil && i == *skip {
			zerothSkipped = true
			continue
		}

		level, err := strconv.Atoi(level)
		if err != nil {
			panic(err)
		}

		if prevLevel == nil {
			prevLevel = &level
			continue
		}

		if (i == 1 && !zerothSkipped) || (i == 2 && zerothSkipped) {
			diff := math.Abs(float64(*prevLevel - level))
			if diff == 0 || diff > 3 {
				return false
			}

			increasing = *prevLevel > level
			prevLevel = &level
			continue
		}

		if (*prevLevel > level) != increasing {
			return false
		}

		diff := math.Abs(float64(*prevLevel - level))
		if diff == 0 || diff > 3 {
			return false
		}

		prevLevel = &level
	}

	return true
}
