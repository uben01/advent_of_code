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
	regA, regB, regC   = 0, 0, 0
	program            []int
	instructionPointer = 0
	programAsString    string
	programOutput      string
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

		if y == 4 {
			program = readNums(re, line)
			for _, p := range program {
				programAsString = fmt.Sprintf("%s%d,", programAsString, p)
			}
		}

		y++
	}

	// 8^15 >= 1 is needed to reach the length
	pows := []int{
		0, // 8^0
		0, // 8^1
		0, // 8^2
		0, // 8^3
		0, // 8^4
		0, // 8^5
		0, // 8^6
		0, // 8^7
		0, // 8^8
		0, // 8^9
		0, // 8^10
		0, // 8^11
		0, // 8^12
		0, // 8^13
		0, // 8^14
		0, // 8^15
	}

	canary(pows)

	y = 0
	for i, p := range pows {
		y += p * int(math.Pow(8, float64(i)))
	}
	for {
		regA = y
		regB = 0
		regC = 0
		programOutput = ""
		instructionPointer = 0

		runProgram()

		//fmt.Printf("%s\n", programOutput)

		if programOutput == programAsString {
			fmt.Printf("A initialized as: %d", y)
			return
		} else if y%10000000 == 0 {
			fmt.Println(y)
		}

		y++
	}
}

func matchingEndN(n int) bool {
	k := len(programOutput) - n

	return programOutput[k:] == programAsString[k:]
}

// pows is an array of 15 zeros initially
func canary(pows []int) {
	matchingN := 0
	for i := 15; i > 2; i-- {
		for {
			pows[i]++

			y := 0
			for i, p := range pows {
				y += p * int(math.Pow(8, float64(i)))
			}

			regA = y
			regB = 0
			regC = 0
			programOutput = ""
			instructionPointer = 0

			runProgram()

			if matchingEndN(matchingN) {
				pows[i]--
				matchingN += 2
				break
			}
		}

	}
}

func runProgram() {
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
		programOutput = fmt.Sprintf("%s%s,", programOutput, string(char))
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
