package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	calibrations := []string{
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
		"eighthree",
		"sevenine",
	}

	result, err := part2(calibrations)
	
	if err != nil{
		t.Error("Failed test")
	}

	if result != 443 {
		t.Errorf("Expected %d, Got %d", 443, result)
	}
}

func TestCheckLastInput(t *testing.T) {
	calibrations := []string{
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
		"fivekltdkmm3rdmdnm32nineddsfdzpks",
	}

	expectedOutput := []string{
		"9",
		"3",
		"3",
		"4",
		"2",
		"4",
		"6",
		"9",
	}

	for i, cal := range calibrations {
		_, result := checkLastInput(cal)

		if result != expectedOutput[i] {
			t.Errorf("Expected %v, Got %v", expectedOutput[i], result)
		}
	}
}
