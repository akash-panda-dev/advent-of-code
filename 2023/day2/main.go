package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const (
	MAX_RED   = 12
	MAX_BLUE  = 14
	MAX_GREEN = 13
)

func main() {
	file, err := os.ReadFile("2023/day2/input.txt")

	if err != nil {
		log.Fatalf("cannot read the file due to : %v", err)
	}

	feasibleGamesSum, gamesPower := processGames(strings.Split(string(file), "\n"))

	fmt.Printf("Sum of feasible games: %d and Games Power: %d", feasibleGamesSum, gamesPower)
}

type GameSet struct {
	red   int
	blue  int
	green int
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func (gameSet *GameSet) updateMaxColors(inputGameSetString string) {
	parsedGameSet := parseGameSet(inputGameSetString)

	gameSet.red = max(gameSet.red, parsedGameSet.red)
	gameSet.green = max(gameSet.green, parsedGameSet.green)
	gameSet.blue = max(gameSet.blue, parsedGameSet.blue)
}

func (gameSet *GameSet) getPower() int {
	return gameSet.red * gameSet.blue * gameSet.green
}

// Input example: " 2 red, 2 green"
func parseGameSet(input string) GameSet {
	inputGameSets := strings.Split(input, ",")
	parsedGameSet := GameSet{}

	for _, gameSet := range inputGameSets {
		//" 2 red"
		gameSetValues := strings.Split(strings.TrimSpace(gameSet), " ")
		gameSetCount, err := strconv.Atoi(gameSetValues[0])

		if err != nil {
			log.Fatalf("Failed to parse gameSets: %v", err)
		}

		switch gameSetValues[1] {
		case "red":
			parsedGameSet.red = gameSetCount
		case "green":
			parsedGameSet.green = gameSetCount
		case "blue":
			parsedGameSet.blue = gameSetCount
		}
	}

	return parsedGameSet
}

func (gameSet *GameSet) isValidgameSet() bool {
	if gameSet.red > MAX_RED || gameSet.blue > MAX_BLUE || gameSet.green > MAX_GREEN {
		return false
	}

	return true
}

func processGames(games []string) (int, int) {
	var feasibleGamesSum int
	var gamesPowerSum int

	for gameNum, game := range games {
		maxgameSet := GameSet{
			red:   -1,
			green: -1,
			blue:  -1,
		}

		gameSetString := strings.Split(game, ":")[1]
		// Now we have this string:
		// 2 red, 2 green; 6 red, 3 green; 2 red, 1 green, 2 blue; 1 red
		gameSets := strings.Split(gameSetString, ";")
		// 2 red, 2 green - 6 red, 3 green - 2 red, 1 green, 2 blue - 1 red
		for _, gameSet := range gameSets {
			maxgameSet.updateMaxColors(gameSet)
		}

		if maxgameSet.isValidgameSet() {
			feasibleGamesSum += gameNum + 1
		}

		gamesPowerSum += maxgameSet.getPower()
	}

	return feasibleGamesSum, gamesPowerSum
}
