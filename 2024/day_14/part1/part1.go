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

	for _, r := range robots {
		r.move(height, width, 100)
	}

	score := scoreQuadrants(robots, width, height)
	fmt.Printf("Score: %d\n", score)
}

func scoreQuadrants(robots []*robot, width, height int) int {
	quadrantScores := make([]int, 4)

	for _, r := range robots {
		q := quadrant(r.position.y, r.position.x, height, width)
		if q == -1 {
			continue
		}
		quadrantScores[q]++
	}

	sum := 1
	for _, score := range quadrantScores {
		sum *= score
	}

	return sum
}

func quadrant(y, x, height, width int) int {
	if y == (height-1)/2 {
		return -1
	}
	if x == (width-1)/2 {
		return -1
	}

	if y < height/2 && x < width/2 {
		return 0
	} else if y < height/2 && x > width/2 {
		return 1
	} else if y > height/2 && x < width/2 {
		return 2
	} else {
		return 3
	}
}

func strToInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}

	return i
}
