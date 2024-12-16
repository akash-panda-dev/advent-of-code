package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("2023/day4/input.txt")

	if err != nil {
		log.Fatalf("Unable to read the file, %v", err)
	}

	totalPoints, totalCardsProcessed := processCards(strings.Split(string(file), "\n"))

	fmt.Printf("The elf has won a total of %d points!!\n", totalPoints)
	fmt.Printf("Total cards processed %d !!", totalCardsProcessed)
}

func parseNumbers(numStr string) map[int]struct{} {
	numStrs := strings.Fields(numStr)
	var numInts []int

	for _, num := range numStrs {
		numInt, err := strconv.Atoi(strings.TrimSpace(num))

		if err != nil {
			log.Fatalf("Failed to convert the string to int: %v", err)
		}
		numInts = append(numInts, numInt)
	}

	return hashNums(numInts)
}

func hashNums(nums []int) map[int]struct{} {
	numset := make(map[int]struct{})

	for _, num := range nums {
		numset[num] = struct{}{}
	}

	return numset
}

func getPoints(selfNumsSet map[int]struct{}, winnerNums map[int]struct{}) (int, int) {
	var matchedNumCount int

	for selfNum := range selfNumsSet {
		if _, ok := winnerNums[selfNum]; ok {
			matchedNumCount++
		}
	}

	return int(math.Pow(2, float64(matchedNumCount)-1)), matchedNumCount
}

func processCards(cards []string) (int, int) {
	var totalPoints int
	cardCounts := make(map[int]int)
	var totalCardsProcessed int

	for i := 1; i <= len(cards); i++ {
		cardCounts[i] = 1
	}

	for i := 1; i <= len(cards); i++ {
		cardSplit := strings.Split(cards[i-1], ":")

		lotteryNums := cardSplit[1]
		winSelfNumsStr := strings.Split(lotteryNums, "|")
		winnerNums := parseNumbers(winSelfNumsStr[0])
		selfNums := parseNumbers(winSelfNumsStr[1])

		points, matchedCount := getPoints(selfNums, winnerNums)

		for k := 0; k <= cardCounts[i]-1; k++ {
			totalPoints += points

			for j := 1; j <= matchedCount; j++ {
				cardCounts[i+j] += 1
			}
			totalCardsProcessed += 1
		}
	}

	return totalPoints, totalCardsProcessed
}
