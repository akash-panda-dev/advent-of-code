package main

import (
	"flag"
	"fmt"
	"log"
	"os"
)

func main() {
	part := flag.Int("part", 1, "Part to execute")
	flag.Parse()

	instructions, err := os.ReadFile("2015/day1/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file: %v", err)
	}

	switch *part {
	case 1:
		result := part1(string(instructions))
		fmt.Printf("Santa is on floor: %d", result)
	case 2:
		result := part2(string(instructions))
		fmt.Printf("Santa has reached the basement at position: %d", result)
	}
	
}

func part1(instructions string) int {
	var floor int

	for _, rune := range instructions {
		switch rune {
		case '(':
			floor += 1
		case ')':
			floor -= 1
		}
	}

	return floor
}

func part2(instructions string) int {
	var floor int
	basementFloor := -1

	for position, rune := range instructions {
		switch rune {
		case '(':
			floor += 1
		case ')':
			floor -= 1
		}

		if floor == basementFloor {
			return position + 1
		}
	}

	panic("Santa never reaches the basement. Kids there will not get their presents sadly :(")
}
