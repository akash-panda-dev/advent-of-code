package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("2023/day1/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file due to: %v", err)
	}

	calibrations := strings.Split(string(file), "\n")
	result, err := getTotalCalib(calibrations)

	if err != nil {
		log.Fatalln("failed to calculate calibrations: ", err)
	}

	fmt.Println("Final calibration: ", result)
}

var numberMap = map[string]string{
	"one":   "1",
	"two":   "2",
	"three": "3",
	"four":  "4",
	"five":  "5",
	"six":   "6",
	"seven": "7",
	"eight": "8",
	"nine":  "9",
}

func findNumber(input string, findFromEnd bool) string {
	var index = -1
	var numStr string

	numbers := []string{"one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"}

	for _, num := range numbers {
		var idx int
		if findFromEnd {
			idx = strings.LastIndex(input, num)
		} else {
			idx = strings.Index(input, num)
		}

		if idx != -1 && (index == -1 || (findFromEnd && idx > index) || (!findFromEnd && idx < index)) {
			index = idx
			numStr = num
		}
	}

	if index != -1 && len(numStr) > 1 {
		numStr = numberMap[numStr]
	}

	return numStr
}

func getTotalCalib(calibrations []string) (int, error) {
	var total int

	for _, cal := range calibrations {
		cal_start := findNumber(cal, false)
		cal_end := findNumber(cal, true)

		final_cal, err := strconv.Atoi(cal_start + cal_end)

		if err != nil {
			return 0, fmt.Errorf("could not convert strings to int: %w", err)
		}

		total += final_cal
	}

	return total, nil
}
