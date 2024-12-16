package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.ReadFile("2023/day19/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	input := strings.Split(string(file), "\n\n")

	result := processPartRatings(input[0], input[1])

	fmt.Println("Total rating:", result)
}

type RatingType rune

const (
	x RatingType = 'x'
	m RatingType = 'm'
	a RatingType = 'a'
	s RatingType = 's'
)

func getRating(ratingMap map[RatingType]int, workflowMap map[string]string) int {
	currentWorkflowName := "in"

	for {
		if currentWorkflowName == "A" {
			return ratingMap[x] + ratingMap[m] + ratingMap[a] + ratingMap[s]
		}

		if currentWorkflowName == "R" {
			return 0
		}

		workflowSpec := workflowMap[currentWorkflowName]
		workflowParts := strings.Split(workflowSpec, ",")
		var nextWorkflowName string
		ruleMatched := false

		for _, workflowPart := range workflowParts[:len(workflowParts)-1] {
			var ratingType rune
			var operator rune
			var limit int

			fmt.Sscanf(workflowPart, "%c%c%d:%s", &ratingType, &operator, &limit, &nextWorkflowName)

			if operator == '<' && ratingMap[RatingType(ratingType)] < limit {
				ruleMatched = true
				break
			} else if operator == '>' && ratingMap[RatingType(ratingType)] > limit {
				ruleMatched = true
				break
			}
		}

		if !ruleMatched {
			nextWorkflowName = workflowParts[len(workflowParts)-1]
		}

		currentWorkflowName = nextWorkflowName

	}
}

func getPartRanges(workflowMap map[string]string) []int {
	xMin, xMax, mMin, mMax, aMin, aMax, sMin, sMax := 4001, 0, 4001, 0, 4001, 0, 4001, 0

	for _, workflowSpec := range workflowMap {
		workflowParts := strings.Split(workflowSpec, ",")
		defaultNextWorkflowName := workflowParts[len(workflowParts)-1]

		for _, workflowPart := range workflowParts[:len(workflowParts)-1] {
			var ratingType rune
			var operator rune
			var limit int
			var nextWorkflowName string

			fmt.Sscanf(workflowPart, "%c%c%d:%s", &ratingType, &operator, &limit, &nextWorkflowName)

			if nextWorkflowName == "R" {
				
		}

	}
}

func processPartRatings(workflows string, ratings string) int {
	var ratingMaps = []map[RatingType]int{}

	for _, rating := range strings.Split(ratings, "\n") {
		var xVal, mVal, aVal, sVal int
		fmt.Sscanf(rating, "{x=%d,m=%d,a=%d,s=%d}", &xVal, &mVal, &aVal, &sVal)
		ratingMaps = append(ratingMaps, map[RatingType]int{x: xVal, m: mVal, a: aVal, s: sVal})
	}

	var workflowMap = map[string]string{}

	for _, workflow := range strings.Split(workflows, "\n") {
		workflowParts := strings.Split(workflow, "{")
		workflowName := workflowParts[0]
		workflowSpec := strings.TrimRight(workflowParts[1], "}")

		workflowMap[workflowName] = workflowSpec
	}

	var totalRating int
	for _, ratingMap := range ratingMaps {
		totalRating += getRating(ratingMap, workflowMap)
	}

	return totalRating
}
