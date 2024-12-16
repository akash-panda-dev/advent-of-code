package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

type node struct {
	row int
	col int
}

type Schematics struct {
	schemas      []string
	schemaSum    int
	gearRatioSum int
}

func main() {
	file, err := os.ReadFile("2023/day3_alt/input.txt")

	if err != nil {
		log.Fatalf("failed to read the file due to %v", err)
	}

	s := Schematics{
		schemas: strings.Split(string(file), "\n"),
	}

	s.processSchemas()

	fmt.Printf("The final sum is %d \n", s.schemaSum)
	fmt.Printf("The final gear ratio sum is %d", s.gearRatioSum)
}

func isASymbol(char rune) bool {
	return !unicode.IsLetter(char) && !unicode.IsDigit(char) && char != '.'
}

func (s *Schematics) traverseAndFindNum(startNode node) int {
	schemaToCheck := s.schemas[startNode.row]
	leftI, rightI := startNode.col, startNode.col

	for i := startNode.col - 1; 0 <= i; i-- {
		if !unicode.IsDigit(rune(schemaToCheck[i])) {
			break
		}
		leftI = i
	}

	for i := startNode.col + 1; i <= len(schemaToCheck)-1; i++ {
		if !unicode.IsDigit(rune(schemaToCheck[i])) {
			break
		}
		rightI = i
	}

	resultNum, err := strconv.Atoi(schemaToCheck[leftI : rightI+1])
	if err != nil {
		log.Fatalf("Failed to convert string to int due to %v", err)
	}

	dotStr := strings.Repeat(".", rightI-leftI+1)
	s.schemas[startNode.row] = schemaToCheck[:leftI] + dotStr + schemaToCheck[rightI+1:]

	return resultNum

}

func (s *Schematics) updateSchemaSum(symbolNode node) {
	rDelta := []int{0, -1, -1, -1, 0, 1, 1, 1}
	cDelta := []int{-1, -1, 0, 1, 1, 1, 0, -1}
	var numbers []int

	for i := range rDelta {
		n_node := node{
			row: symbolNode.row + rDelta[i],
			col: symbolNode.col + cDelta[i],
		}

		if 0 <= n_node.row && n_node.row < len(s.schemas) && 0 <= n_node.col && n_node.col < len(s.schemas[0]) {
			// check if n_node is a number
			if unicode.IsDigit(rune(s.schemas[n_node.row][n_node.col])) {
				numbers = append(numbers, s.traverseAndFindNum(n_node))
			}
		}
	}

	for _, num := range numbers {
		s.schemaSum += num
	}

	if len(numbers) == 2 && rune(s.schemas[symbolNode.row][symbolNode.col]) == '*' {
		s.gearRatioSum += numbers[0] * numbers[1]
	}
}

func (s *Schematics) processSchemas() {

	row_len := len(s.schemas)
	col_len := len(s.schemas[0])

	for i := 0; i <= row_len-1; i++ {
		// 467..114..
		for j := 0; j <= col_len-1; j++ {
			// 4
			// temp := string(char)
			// fmt.Println(temp)
			if isASymbol(rune(s.schemas[i][j])) {
				// Check neighbors
				s.updateSchemaSum(node{
					row: i,
					col: j,
				})
			}
		}
	}
}
