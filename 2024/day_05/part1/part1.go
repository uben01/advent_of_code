package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type pair struct {
	before string
	after  string
}

func main() {
	file, err := os.Open("day_05/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	rules := make([]pair, 0)
	sum := 0

	isRule := true
	scanner := bufio.NewScanner(file)
scanner:
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			isRule = false
			continue
		}
		if isRule {
			split := strings.Split(line, "|")
			rules = append(rules, pair{before: split[0], after: split[1]})
		}
		if !isRule {
			split := strings.Split(line, ",")
			alreadyChecked := make([]string, 0)
			var middle int

			for i, str := range split {
				if i == (len(split) / 2) {
					middle, err = strconv.Atoi(str)
					if err != nil {
						panic(err)
					}
				}

				if !checkAgainstAlreadyChecked(rules, alreadyChecked, str) {
					continue scanner
				}

				alreadyChecked = append(alreadyChecked, str)
			}
			sum += middle
		}
	}

	fmt.Printf("Sum: %d", sum)
}

func checkAgainstAlreadyChecked(rules []pair, alreadyChecked []string, current string) bool {
	for _, chk := range alreadyChecked {
		if !checkAgainstRule(rules, chk, current) {
			return false
		}
	}

	return true
}

func checkAgainstRule(rules []pair, first string, second string) bool {
	for _, rule := range rules {
		if rule.after == first && rule.before == second {
			return false
		}
	}

	return true
}
