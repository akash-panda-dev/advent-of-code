package main

import (
	"os"
	"strings"
	"testing"
)

var grid []string

func init() {
	file, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	grid = strings.Split(string(file), "\n")
}

func BenchmarkProcessMirrorGrid(b *testing.B) {
	for i := 0; i < b.N; i++ {
		processMirrorGrid(grid)
	}
}