package main

import (
	"os"
	"strings"
	"testing"
)

var input []string

func init() {
	file, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	input = strings.Split(string(file), "\n\n")
}

func TestProcessPartRatings(t *testing.T) {
	result := processPartRatings(input[0], input[1])

	if result != 0 {
		t.Errorf("Expected 0, got %d", result)
	}
}