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


func checkFirstInput(input string) (int, string) {
	

	var firstIndex = -1
	var firstNumStr string

	for _, numStr := range []string{"one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"} {
		index := strings.Index(input, numStr)
		if index != -1 && (firstIndex == -1 || index < firstIndex) {
			firstIndex = index
			firstNumStr = numStr
		}
	}

	if firstIndex != -1 && len(firstNumStr) > 1{
		firstNumStr = numberMap[firstNumStr]
	}

	return firstIndex, firstNumStr
}

func checkLastInput(input string) (int, string) {
	var lastIndex = -1
	var lastNumStr string

	for _, numStr := range []string{"one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"} {
		index := strings.LastIndex(input, numStr)
		if index != -1 && (lastIndex == -1 || index > lastIndex) {
			lastIndex = index
			lastNumStr = numStr
		}
	}

	if lastIndex != -1 && len(lastNumStr) > 1{
		lastNumStr = numberMap[lastNumStr]
	}

	return lastIndex, lastNumStr
}


func getTotalCalib(calibrations []string) (int, error) {
	var total int

	for _, cal := range calibrations {
		cal_start, cal_end := "", ""
		start, end := 0, len(cal)-1
		
		for ; start <= len(cal)-1; start++ {
			var subStrToCheck string
			if len(cal[start:]) >= 5 {
				subStrToCheck = cal[start: start + 5]
			} else {
				subStrToCheck = cal[start:]
			}

			index, result := checkFirstInput(subStrToCheck)
			
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

			index, result := checkLastInput(subStrToCheck)
			
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
