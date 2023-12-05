package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("2023/day5/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	processAlmanac(strings.Split(string(file), "\n"))
}

func parseSeeds(seedStr string, seedToSoil map[int][]int) {
	seedsNumsStr := strings.Split(seedStr, ":")[1]
	seedsNums := strings.Fields(seedsNumsStr)

	for _, num := range seedsNums {
		seedToSoil[strToInt(num)] = []int{}
	}
}

func strToInt(str string) int {
	num, err := strconv.Atoi(strings.TrimSpace(str))

	if err != nil {
		log.Fatalf("Failed to convert the string to int: %v", err)
	}

	return num
}

func populateResourceMap(resourceInput []string, sourceMap map[int][]int, destinationMap map[int][]int, startIndex int) int {
	var endIndex int
	var sourceToFoundMap = make(map[int]bool)

	for i := startIndex; i <= len(resourceInput)-1; i++ {
		resourcesRange := strings.Fields(resourceInput[i])

		if len(resourcesRange) == 0 {
			break
		}

		destinationStart := strToInt(resourcesRange[0])
		sourceStart := strToInt(resourcesRange[1])
		rangeLength := strToInt(resourcesRange[2])
		var destinationEnd int

		for source := range sourceMap {
			if sourceStart <= source && source <= sourceStart+rangeLength {
				destinationEnd = destinationStart + (source - sourceStart)
				sourceMap[source] = append(sourceMap[source], destinationEnd)
				destinationMap[destinationEnd] = []int{}
				sourceToFoundMap[source] = true
			}
		}

		for source := range sourceMap {
			if _, ok := sourceToFoundMap[source]; !ok {
				sourceMap[source] = append(sourceMap[source], source)
				destinationMap[source] = []int{}
			}
		}
		endIndex = i
	}

	return endIndex + 2
}

func processAlmanac(input []string) {
	var (
		seedToSoil            = make(map[int][]int)
		soilToFertilizer      = make(map[int][]int)
		fertilizerToWater     = make(map[int][]int)
		waterToLight          = make(map[int][]int)
		lightToTemperature    = make(map[int][]int)
		temperatureToHumidity = make(map[int][]int)
		humidityToLocation    = make(map[int][]int)
	)

	parseSeeds(input[0], seedToSoil)

	for i := 1; i <= len(input)-1; {
		inputSlice := strings.Split(input[i], " ")

		switch inputSlice[0] {
		case "seed-to-soil":
			i = populateResourceMap(input, seedToSoil, soilToFertilizer, i+1)
			continue
		case "soil-to-fertilizer":
			i = populateResourceMap(input, soilToFertilizer, fertilizerToWater, i+1)
			continue
		case "fertilizer-to-water":
			i = populateResourceMap(input, fertilizerToWater, waterToLight, i+1)
			continue
		case "water-to-light":
			i = populateResourceMap(input, waterToLight, lightToTemperature, i+1)
			continue
		case "light-to-temperature":
			i = populateResourceMap(input, lightToTemperature, temperatureToHumidity, i+1)
			continue
		case "temperature-to-humidity":
			i = populateResourceMap(input, temperatureToHumidity, humidityToLocation, i+1)
			continue
		case "humidity-to-location":
			i = populateResourceMap(input, humidityToLocation, humidityToLocation, i+1)
			continue
		default:
			i++
		}
	}
}
