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


func findNumber(input string, findFromEnd bool) (int, string) {
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

	return index, numStr
}

// Algo: To get the first number start from 0 and to get the last number
// start from the end. 
// Stop early if found a number.
// Kind of like a two pointer algo.
func getTotalCalib(calibrations []string) (int, error) {
	var total int

	for _, cal := range calibrations {
		cal_start, cal_end := "", ""
		start, end := 0, len(cal)-1
		
		// Since the max length of a number string is 5
		// Taking substrings of 5 from both left and right 
		// And checking if they have a number
		for ; start <= len(cal)-1; start++ {
			var subStrToCheck string
			if len(cal[start:]) >= 5 {
				subStrToCheck = cal[start: start + 5]
			} else {
				subStrToCheck = cal[start:]
			}

			index, result := findNumber(subStrToCheck, false)
			
			if index != -1 {
				cal_start = result
				start += index
				break
			}
		}

		for ; end >= start; end-- {
			var subStrToCheck string
			if len(cal[:end+1]) >= 5 {
				subStrToCheck = cal[end-4:end+1]
			} else {
				subStrToCheck = cal[:end+1]
			}

			index, result := findNumber(subStrToCheck, true)
			
			if index != -1 {
				cal_end = result
				break
			}
		}
		final_cal, err := strconv.Atoi(cal_start + cal_end)

		if err != nil {
			return 0, fmt.Errorf("could not convert strings to int: %w", err)
		}

		total += final_cal
	}

	return total, nil
}
