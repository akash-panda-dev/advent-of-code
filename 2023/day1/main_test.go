package main

import (
	"testing"
)

func TestGetTotalCalib(t *testing.T) {
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
		"7abcdefgh121",
	}

	result, err := getTotalCalib(calibrations)

	if err != nil {
		t.Error("Failed test")
	}

	if result != 514 {
		t.Errorf("Expected %d, Got %d", 514, result)
	}
}

func BenchmarkGetTotalCalib(b *testing.B) {
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

	for i := 0; i < b.N; i++ {
		getTotalCalib(calibrations)
	}
}

func TestFindNumber(t *testing.T) {
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
		result := findNumber(cal, true)

		if result != expectedOutput[i] {
			t.Errorf("Expected %v, Got %v", expectedOutput[i], result)
		}
	}
}
