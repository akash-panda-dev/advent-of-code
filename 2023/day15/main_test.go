package main

import "testing"

func TestHashSeq(t *testing.T) {
	t.Run("should return the correct hash", func(t *testing.T) {
		seq := "qp=3"
		expected := 1
		got := hashSeq(seq)

		if got != expected {
			t.Errorf("Expected %v but got %v", expected, got)
		}
	})
}
