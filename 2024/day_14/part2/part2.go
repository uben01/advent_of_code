package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type robot struct {
	position coordinate
	velocity coordinate
}

func (r *robot) move(height, width, times int) {
	r.position.y = r.position.y + (r.velocity.y * times)
	r.position.x = r.position.x + (r.velocity.x * times)

	if r.position.x >= width {
		r.position.x = r.position.x % width
	}
	if r.position.y >= height {
		r.position.y = r.position.y % height
	}

	for r.position.x < 0 {
		r.position.x += width
	}
	for r.position.y < 0 {
		r.position.y += height
	}
}

type coordinate struct {
	y, x int
}

func main() {
	file, err := os.Open("day_14/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	robots := make([]*robot, 0)
	width := 101  // example 11, input 101
	height := 103 // example 7, input 103

	scanner := bufio.NewScanner(file)
	re := regexp.MustCompile(`(-?\d+)`)
	for scanner.Scan() {
		line := scanner.Text()
		strs := re.FindAllString(line, 5)

		px := strToInt(strs[0])
		py := strToInt(strs[1])
		vx := strToInt(strs[2])
		vy := strToInt(strs[3])

		robots = append(robots, &robot{position: coordinate{py, px}, velocity: coordinate{vy, vx}})
	}

	// fully arbitrary number of iterations
	for i := 1; i < 10000; i++ {
		for _, r := range robots {
			r.move(height, width, 1)
		}
		shouldStop := checkMap(robots, width, height)
		if shouldStop {
			printMap(robots, width, height)
			fmt.Println("Stopped at", i)
			break
		}
	}
}

func checkMap(robots []*robot, width, height int) bool {
	shouldStop := false
	for y := 0; y < height; y++ {
		nextToEach := 0
		for x := 0; x < width; x++ {
			found := 0
			for _, r := range robots {
				if r.position.y == y && r.position.x == x {
					found++
				}
			}
			if found == 0 {
				nextToEach = 0
			} else {
				nextToEach++
			}

			if nextToEach == 8 { // fully arbitrary number
				shouldStop = true
			}
		}
	}

	return shouldStop
}

func printMap(robots []*robot, width, height int) {
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			found := 0
			for _, r := range robots {
				if r.position.y == y && r.position.x == x {
					found++
				}
			}
			if found == 0 {
				fmt.Print(".")
			} else {
				fmt.Print(found)
			}
		}
		fmt.Println()
	}
}

func strToInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}

	return i
}
