package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

var (
	regA, regB, regC   int
	program            []int
	instructionPointer = 0
)

func main() {
	file, err := os.Open("day_17/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	y := 0
	re := regexp.MustCompile(`\d+`)
	for scanner.Scan() {
		line := scanner.Text()

		switch y {
		case 0:
			regA = readNum(re, line)
			break
		case 1:
			regB = readNum(re, line)
			break
		case 2:
			regC = readNum(re, line)
			break
		default:
			program = readNums(re, line)
		}

		y++
	}

	for instructionPointer < len(program) {
		instruction := program[instructionPointer]
		operand := mapOperand(program[instructionPointer+1])

		switch instruction {
		case 0:
			adv(operand)
			break
		case 1:
			bxl(operand)
			break
		case 2:
			bst(operand)
			break
		case 3:
			jnz(operand)
			break
		case 4:
			bxc(operand)
			break
		case 5:
			out(operand)
			break
		case 6:
			bdv(operand)
			break
		case 7:
			cdv(operand)
			break
		}
	}
}

func readNum(re *regexp.Regexp, line string) int {
	a := re.FindString(line)
	ret, err := strconv.Atoi(a)
	if err != nil {
		panic(err)
	}

	return ret
}

func readNums(re *regexp.Regexp, line string) []int {
	a := re.FindAllString(line, -1)
	ret := make([]int, 0)

	for _, r := range a {
		num, err := strconv.Atoi(r)
		if err != nil {
			panic(err)
		}
		ret = append(ret, num)
	}

	return ret
}

func advanceProgram() {
	instructionPointer += 2
}

func mapOperand(operand int) int {
	if operand <= 3 {
		return operand
	}

	if operand == 4 {
		return regA
	}
	if operand == 5 {
		return regB
	}
	if operand == 6 {
		return regC
	}

	panic("Operand unknown")
}

// opcode 0
func adv(operand int) {
	regA = regA / int(math.Pow(2, float64(operand)))
	advanceProgram()
}

// opcode 1
func bxl(operand int) {
	regB = regB ^ operand
	advanceProgram()
}

// opcode 2
func bst(operand int) {
	regB = operand % 8
	advanceProgram()
}

// opcode 3
func jnz(operand int) {
	if regA == 0 {
		advanceProgram()
		return
	}

	instructionPointer = operand
}

// opcode 4
func bxc(_ int) {
	regB = regB ^ regC
	advanceProgram()
}

// opcode 5
func out(operand int) {
	val := operand % 8
	str := strconv.Itoa(val)
	for _, char := range str {
		fmt.Printf("%s,", string(char))
	}
	advanceProgram()
}

// opcode 6
func bdv(operand int) {
	regB = regA / int(math.Pow(2, float64(operand)))
	advanceProgram()
}

// opcode 7
func cdv(operand int) {
	regC = regA / int(math.Pow(2, float64(operand)))
	advanceProgram()
}
