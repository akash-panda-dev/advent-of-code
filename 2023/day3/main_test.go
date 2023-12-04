package main

import "testing"

func TestProcessSchemas(t *testing.T) {
	schematics := Schematics{
		schemas: []string{
			"467..114..",
			"...*......",
			"..35..633.",
			"......#...",
			"617*......",
			".....+.58.",
			"..592.....",
			"......755.",
			"...$.*....",
			".664.598..",
		},
	}


	schematics.processSchemas()

	if schematics.schemaSum != 4361 {
		t.Errorf("Expected: %d, Got: %d", 4361, schematics.schemaSum)
	}
}