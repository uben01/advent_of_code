package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type operation string

const (
	AND operation = "AND"
	OR  operation = "OR"
	XOR operation = "XOR"
)

type connection struct {
	Operation operation
	From1     string
	From2     string
	To        string
}

var (
	inputs      = make(map[string]bool)
	connections = make([]connection, 0)
	outputs     = make(map[connection]bool)
)

func main() {
	file, err := os.Open("day_24/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	isReadingInputs := true
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			isReadingInputs = false
			continue
		}

		if isReadingInputs {
			split := strings.Split(line, ": ")
			inputs[split[0]] = split[1] == "1"
		} else {
			split := strings.Split(line, " ")
			to := split[4]
			c := connection{
				Operation: operation(split[1]),
				From1:     split[0],
				From2:     split[2],
				To:        to,
			}
			connections = append(connections, c)

			if strings.HasPrefix(to, "z") {
				outputs[c] = false
			}
		}
	}

	re := regexp.MustCompile(`\d+`)

	calculateOutputs()
	outputList := make([]bool, len(outputs))
	for key, result := range outputs {
		o := re.FindString(key.To)
		oInt, err := strconv.Atoi(o)
		if err != nil {
			panic(err)
		}

		outputList[oInt] = result
	}

	result := 0
	for i, o := range outputList {
		if o {
			result += int(math.Pow(2, float64(i)))
		}
	}

	fmt.Printf("Result: %d\n", result)
}

func calculateOutputs() {
	for o := range outputs {
		from1 := resolveInput(o.From1)
		from2 := resolveInput(o.From2)
		result := makeOperation(o.Operation, from1, from2)
		outputs[o] = result
	}
}

func resolveInput(from string) bool {
	if input, ok := inputs[from]; ok {
		return input
	}
	for _, c := range connections {
		if c.To == from {
			from1 := resolveInput(c.From1)
			from2 := resolveInput(c.From2)
			result := makeOperation(c.Operation, from1, from2)

			inputs[from] = result

			return result
		}
	}
	panic("Invalid input")
}

func makeOperation(op operation, from1, from2 bool) bool {
	switch op {
	case AND:
		return from1 && from2
	case OR:
		return from1 || from2
	case XOR:
		return from1 != from2
	}

	panic("Invalid operation")
}
