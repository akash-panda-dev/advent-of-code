package main

import (
	"fmt"
	"log"
	"math"
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

	games := strings.Split(string(file), "\n")

	feasibleGamesSum, gamesPower := processGames(games)

	fmt.Printf("Sum of feasible games: %d and Games Power: %d", feasibleGamesSum, gamesPower)
}

type Cube struct {
	red   int
	blue  int
	green int
}

func (cube *Cube) updateMaxValue(inputCubeSet string) {
	parsedCube := parseCube(inputCubeSet)

	cube.red = int(math.Max(float64(cube.red), float64(parsedCube.red)))
	cube.green = int(math.Max(float64(cube.green), float64(parsedCube.green)))
	cube.blue = int(math.Max(float64(cube.blue), float64(parsedCube.blue)))
}

func (cube *Cube) getPower() int {
	return cube.red * cube.blue * cube.green
}

// Input example: " 2 red, 2 green"
func parseCube(input string) Cube {
	cubes := strings.Split(input, ",")
	parsedCube := Cube{}

	for _, cube := range cubes {
		//" 2 red"
		trimmedCube := strings.TrimSpace(cube)
		cubeValues := strings.Split(trimmedCube, " ")
		cubeCount, err := strconv.Atoi(cubeValues[0])

		if err != nil {
			log.Fatalf("Failed to parse cubes: %v", err)
		}

		switch cubeValues[1] {
		case "red":
			parsedCube.red = cubeCount
		case "green":
			parsedCube.green = cubeCount
		case "blue":
			parsedCube.blue = cubeCount
		}
	}

	return parsedCube
}

func isValidCube(cube Cube) bool {
	if (cube.red > MAX_RED || cube.blue > MAX_BLUE || cube.green > MAX_GREEN) {
		return false
	}

	return true
}

func processGames(games []string) (int, int) {
	var feasibleGamesSum int
	var gamesPower int

	for gameNum, game := range games {
		maxCube := Cube{
			red: -1,
			green: -1,
			blue: -1,
		}

		cubeSetString := strings.Split(game, ":")[1]
		// Now we have this string:
		// 2 red, 2 green; 6 red, 3 green; 2 red, 1 green, 2 blue; 1 red
		cubeSets := strings.Split(cubeSetString, ";")
		// 2 red, 2 green - 6 red, 3 green - 2 red, 1 green, 2 blue - 1 red
		for _, cubeSet := range cubeSets {
			maxCube.updateMaxValue(cubeSet)
		}

		if isValidCube(maxCube) {
			feasibleGamesSum += gameNum + 1
		}

		gamesPower += maxCube.getPower()
	}

	return feasibleGamesSum, gamesPower
}
