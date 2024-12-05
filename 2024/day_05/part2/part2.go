package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
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
	incorrectLines := make([]string, 0)

	isRule := true
	scanner := bufio.NewScanner(file)
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

			for _, str := range split {
				if !checkAgainstAlreadyChecked(rules, alreadyChecked, str) {
					incorrectLines = append(incorrectLines, line)
					break
				}

				alreadyChecked = append(alreadyChecked, str)
			}
		}
	}

	sum := 0
	for _, line := range incorrectLines {
		split := strings.Split(line, ",")
		swapFun := reflect.Swapper(split)

		swapViolatingNums(rules, split, swapFun)

		var num int
		num, err = strconv.Atoi(split[len(split)/2])
		if err != nil {
			panic(err)
		}

		sum += num
	}

	fmt.Printf("sum: %d", sum)
}

func swapViolatingNums(rules []pair, split []string, swapFun func(int, int)) {
outer:
	for {
		for i, first := range split {
			for j, second := range split {
				if i >= j {
					continue
				}

				if !checkAgainstRule(rules, first, second) {
					swapFun(i, j)
					continue outer
				}
			}
		}
		break outer
	}
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
