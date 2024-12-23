package main

import (
	"bufio"
	"fmt"
	"golang.org/x/exp/slices"
	"os"
	"regexp"
)

func main() {
	file, err := os.Open("day_23/input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	computers := make([]string, 0)
	connections := make([][]string, 0)

	// 3 computers connected to each other
	// using map just to make unique easily
	networks := make(map[string]string)

	re := regexp.MustCompile(`\w+`)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		comps := re.FindAllString(scanner.Text(), 2)
		if !slices.Contains(computers, comps[0]) {
			computers = append(computers, comps[0])
		}
		if !slices.Contains(computers, comps[1]) {
			computers = append(computers, comps[1])
		}
		connections = append(connections, comps)
	}

	for _, startingComputer := range computers {
		// collect connected computers to the starting computer
		connectedComputers := make([]string, 0)
		for _, connection := range connections {
			if c := connectedComputer(startingComputer, connection); c != nil {
				connectedComputers = append(connectedComputers, *c)
			}
		}

		for _, connected := range connectedComputers {
			relevantComputers := make([]string, 0)
			// collect computers connected to the connected computers, but the starting computer
			for _, connection := range connections {
				if c := connectedComputer(connected, connection); c != nil {
					if !isConnectionBetween(startingComputer, connected, connection) {
						relevantComputers = append(relevantComputers, *c)
					}
				}
			}

			// collect relevant computers connected to the starting computer
			for _, relevant := range relevantComputers {
				for _, connection := range connections {
					if isConnectionBetween(startingComputer, relevant, connection) {
						key := makeNetworkFromComputers([]string{startingComputer, connected, relevant})
						networks[key] = key
					}
				}
			}
		}
	}

	count := 0
	for k := range networks {
		if k[0] == 't' || k[2] == 't' || k[4] == 't' {
			count++
		}
	}

	fmt.Printf("Number of networks containing \"t\": %d\n", count)
}

func makeNetworkFromComputers(computers []string) string {
	slices.Sort(computers)
	return fmt.Sprintf("%s%s%s", computers[0], computers[1], computers[2])
}

func connectedComputer(computer string, connection []string) *string {
	if connection[0] == computer {
		return &connection[1]
	}
	if connection[1] == computer {
		return &connection[0]
	}
	return nil
}

func isConnectionBetween(computer1, computer2 string, connection []string) bool {
	conn := connectedComputer(computer1, connection)
	if conn == nil {
		return false
	}
	return *conn == computer2
}
