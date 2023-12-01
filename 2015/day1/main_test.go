package main

import "testing"

func TestPart1(t *testing.T) {
	t.Run("Test with valid instructions", func(t *testing.T) {
		t.Parallel()
		result := part1("(())")

		if result != 0 {
			t.Errorf("Expected %d, Got %d", 0, result)
		}
	})
}

func TestPart2(t *testing.T) {
	t.Run("Test with valid instructions", func(t *testing.T) {
		t.Parallel()
		result := part2("()())")

		if result != 5 {
			t.Errorf("Expected %d, Got %d", 5, result)
		}
	})
}

func BenchmarkPart1(b *testing.B) {
	b.Run("Part1 benchmarking", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			part1("((()()))()(()()(())())")
		}
	})
}
