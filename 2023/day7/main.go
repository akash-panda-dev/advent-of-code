package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

/*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

## How to find the type of hand

1. Find the frequency of each card and put in a map.
2. Then check the map for the following:
	- If there is a 5, then it's a 5 kind
	- If there is a 4, then it's a four of a kind
	- If there is a 3 and a 2, then it's a full house
	- If there is a 3, then it's a three of a kind
	- If there is a 2 and a 2, then it's a two pair
	- If there is a 2, then it's a pair
	- If there is a 1, then it's a high card
*/

var cardValues = map[rune]int{
	'J': 0,
	'2': 1,
	'3': 2,
	'4': 3,
	'5': 4,
	'6': 5,
	'7': 6,
	'8': 7,
	'9': 8,
	'T': 9,
	'Q': 11,
	'K': 12,
	'A': 13,
}

func main() {
	file, err := os.ReadFile("2023/day7/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	totalWinnings := processCards(strings.Split(string(file), "\n"))

	fmt.Println("The result for part 1: ", totalWinnings)
}

func strToInt(str string) int {
	num, err := strconv.Atoi(strings.TrimSpace(str))

	if err != nil {
		log.Fatalf("Failed to convert the string to int: %v", err)
	}

	return num
}

type hand struct {
	cards    string
	bid      int
	handType HandType
}

type HandType int

const (
	_        HandType = iota // ignore zero value
	HighCard HandType = iota
	Pair
	TwoPair
	ThreeOfAKind
	FullHouse
	FourOfAKind
	FiveOfAKind
)

func getCardComboCount(cardsInput string) map[int]int {
	cardsFrequency := make(map[rune]int)
	comboCount := make(map[int]int)
	var jokerCount int

	// Handling jokers along with the cards
	for _, card := range cardsInput {
		if card == 'J' {
			jokerCount++
		} else {
			cardsFrequency[card]++
		}
	}

	if jokerCount == 5 {
		comboCount[5] = 1
		return comboCount
	}

	maxCount := math.MinInt32
	var maxCard rune

	for card, frequency := range cardsFrequency {
		if frequency > maxCount {
			maxCount = frequency
			maxCard = card
		}
	}

	cardsFrequency[maxCard] += jokerCount

	for _, count := range cardsFrequency {
		comboCount[count]++
	}

	return comboCount
}

func getHandType(cards string) HandType {
	comboCount := getCardComboCount(cards)

	switch {
	case comboCount[5] == 1:
		return FiveOfAKind
	case comboCount[4] == 1:
		return FourOfAKind
	case comboCount[3] == 1 && comboCount[2] == 1:
		return FullHouse
	case comboCount[3] == 1:
		return ThreeOfAKind
	case comboCount[2] == 2:
		return TwoPair
	case comboCount[2] == 1:
		return Pair
	default:
		return HighCard
	}
}

func handComparator(hand1 hand, hand2 hand) bool {
	switch {
	case hand1.handType < hand2.handType:
		return true
	case hand1.handType > hand2.handType:
		return false
	case hand1.handType == hand2.handType:
		for i := 0; i <= len(hand1.cards)-1; i++ {
			switch {
			case cardValues[rune(hand1.cards[i])] < cardValues[rune(hand2.cards[i])]:
				return true
			case cardValues[rune(hand1.cards[i])] > cardValues[rune(hand2.cards[i])]:
				return false
			}
		}
	}

	return false
}

func sortHands(hands []hand) {
	sort.Slice(hands, func(i, j int) bool {
		return handComparator(hands[i], hands[j])
	})
}

func processCards(input []string) int {
	var hands []hand
	//var handTypeToHands = make(map[HandType][]hand)
	var totalWinnings int

	for _, line := range input {
		cards := strings.Fields(line)
		hands = append(hands, hand{
			cards:    cards[0],
			bid:      strToInt(cards[1]),
			handType: getHandType(cards[0]),
		})
	}

	sortHands(hands)

	for rank, hand := range hands {
		totalWinnings += hand.bid * (rank + 1)
	}

	return totalWinnings

	// // No go through the hand type map starting from 5 of a kind and assign ranks and multiply the bid and add to the result
	// var rank int = 1
	// for handType := HighCard; handType >= FiveOfAKind; handType-- {
	// 	handsToCheck := handTypeToHands[handType]
	// 	// if there are no hands of this type, then continue
	// 	if len(handsToCheck) == 0 {
	// 		continue
	// 	}

	// 	// if there is only one hand of this type, then assign the rank and continue
	// 	if len(handsToCheck) == 1 {
	// 		totalWinnings += handTypeToHands[handType][0].bid * rank
	// 		rank++
	// 		continue
	// 	}

	// 	// if there are more than one hand of this type, then we need to compare the hands
	// 	// first sort the hands
	// 	sortHands(handsToCheck)
	// 	for _, hand := range handsToCheck {
	// 		totalWinnings += hand.bid * rank
	// 		rank++
	// 	}
	// }

	// return totalWinnings
}
