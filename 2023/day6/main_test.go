package main

import "testing"

func TestProcessTimesheets(t *testing.T) {
	input := []string{
		"Time:      7  15   30",
		"Distance:  9  40  200",
	}

	count := processtimesheet(input)

	if count != 288 {
		t.Errorf("Expected %d, Got %d", 288, count)
	}
}