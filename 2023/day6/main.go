package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

/*
Time:      7  15   30
Distance:  9  40  200

distance = (time - hold) * hold

(7-0) * 0 = 0
(7-1) * 1 = 6
(7-2) * 2 = 10
(7-3) * 3 = 12
(7-4) * 4 = 12
(7-5) * 5 = 10
(7-6) * 6 = 6
(7-7) * 7 = 0

b = time
x = hold
y = distance

y = (b - x) * x
y = bx - x^2

y > 9

This makes an inverse parabola

y = -x^2 + bx
a = -1
b = time
c = distance
x = hold

So now just use the quadratic formula to find the x's at the distance
and then just subtract the range of x's to get the number of holds
*/

func main() {
	file, err := os.ReadFile("2023/day6/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	part1Result := getHoldsToBeatRecord(strings.Split(string(file), "\n"), true)
	part2Result := getHoldsToBeatRecord(strings.Split(string(file), "\n"), false)

	fmt.Println("The result for part 1: ", part1Result)
	fmt.Println("The result for part 2: ", part2Result)
	
}

type timesheet struct {
	time int
	distance int
}

func strToInt(str string) int {
	num, err := strconv.Atoi(strings.TrimSpace(str))

	if err != nil {
		log.Fatalf("Failed to convert the string to int: %v", err)
	}

	return num
}

func parseTimesheetsWithKerning(timeSheetStr []string) []timesheet {
	timeStrs := strings.Fields(strings.Split(timeSheetStr[0], ":")[1])
	distStrs := strings.Fields(strings.Split(timeSheetStr[1], ":")[1])
	var timeSheets []timesheet

	for i := 0; i <= len(timeStrs) - 1; i++ {
		timeSheets = append(timeSheets, timesheet{
			time: strToInt(timeStrs[i]),
			distance: strToInt(distStrs[i]),
		})
	}

	return timeSheets
}

func parseTimesheeNoKerning(timeSheetStr []string) []timesheet {
	timeStr := strings.Replace(strings.Split(timeSheetStr[0], ":")[1], " ", "", -1)
	distStr := strings.Replace(strings.Split(timeSheetStr[1], ":")[1], " ", "", -1)
	var timesheets []timesheet

	timesheets = append(timesheets, timesheet{
		time: strToInt(timeStr),
		distance: strToInt(distStr),
	})

	return timesheets
}

func getHoldsToBeatRecord(timeSheetStr []string, isKerningPresent bool) int {
	var timeSheets []timesheet
	if isKerningPresent {
		timeSheets = parseTimesheetsWithKerning(timeSheetStr)
	} else {
		timeSheets = parseTimesheeNoKerning(timeSheetStr)
	}

	var result int = 1

	var validHoldCounts []int

	for _, ts := range timeSheets {
		a := float64(1)
		b := -1 * float64(ts.time)
		c := float64(ts.distance)
		holdStart := (-1.0 * b - math.Sqrt(math.Pow(b, 2) - 4.0*a*c))/2*a
		holdEnd := (-1.0 * b + math.Sqrt(math.Pow(b, 2) - 4.0*a*c))/2*a

		holdStart = math.Floor(holdStart + 1)
		holdEnd = math.Ceil(holdEnd - 1)

		validHoldCounts = append(validHoldCounts, int(holdEnd) - int(holdStart) + 1)
	}

	for _, count := range validHoldCounts {
		result *= count
	}

	return result
}
