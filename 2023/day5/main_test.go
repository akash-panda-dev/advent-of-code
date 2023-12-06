package main

import (
	"os"
	"strings"
	"testing"
)

func TestProcessAlmanac(m *testing.T) {
	file, err := os.ReadFile("./input_test.txt")

	if err != nil {
		m.Fatalf("Failed to read the file, %v", err)
	}

	processAlmanac(strings.Split(string(file), "\n"))
}
