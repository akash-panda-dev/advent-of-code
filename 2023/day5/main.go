package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"

	"slices"
)

func main() {
	file, err := os.ReadFile("2023/day5/input_test.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	processAlmanac(strings.Split(string(file), "\n"))
}

func parseSeeds(seedStr string, seedToSoil map[int][]int) {
	seedsNumsStr := strings.Split(seedStr, ":")[1]
	seedsNumsAndRanges := strings.Fields(seedsNumsStr)

	for i := 0; i <= len(seedsNumsAndRanges)-1; i += 2 {
		seedStart := strToInt(seedsNumsAndRanges[i])
		seedRange := strToInt(seedsNumsAndRanges[i+1])

		for seed := seedStart; seed < seedStart+seedRange; seed++ {
			seedToSoil[seed] = []int{}
		}
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

		endIndex = i
	}

	for source := range sourceMap {
		if _, ok := sourceToFoundMap[source]; !ok {
			sourceMap[source] = append(sourceMap[source], source)
			destinationMap[source] = []int{}
		}
	}

	return endIndex + 2
}

type ResourceType string

const (
	Seed        ResourceType = "seed"
	Soil        ResourceType = "soil"
	Fertilizer  ResourceType = "fertilizer"
	Water       ResourceType = "water"
	Light       ResourceType = "light"
	Temperature ResourceType = "temperature"
	Humidity    ResourceType = "humidity"
)

var ResourceToResourceMap = map[ResourceType]ResourceType{
	Seed:        Soil,
	Soil:        Fertilizer,
	Fertilizer:  Water,
	Water:       Light,
	Light:       Temperature,
	Temperature: Humidity,
}

func findMinLocation(resource int, resourceType ResourceType, resourceTypeMap map[ResourceType]map[int][]int) int {
	if resourceType == Humidity {
		return slices.Min(resourceTypeMap[Humidity][resource])
	}

	if _, ok := resourceTypeMap[resourceType][resource]; !ok {
		return math.MaxInt32
	}

	var minLocations []int

	for _, r := range resourceTypeMap[resourceType][resource] {
		minLocations = append(minLocations, findMinLocation(r, ResourceToResourceMap[resourceType], resourceTypeMap))
	}

	return slices.Min(minLocations)
}

func processAlmanac(input []string) {
	var (
		seedToSoil = make(map[int][]int)
		soilToFertilizer      = make(map[int][]int)
		fertilizerToWater     = make(map[int][]int)
		waterToLight          = make(map[int][]int)
		lightToTemperature    = make(map[int][]int)
		temperatureToHumidity = make(map[int][]int)
		humidityToLocation    = make(map[int][]int)
	)

	resourceTypeMap := map[ResourceType]map[int][]int {
		Seed: seedToSoil,
		Soil: soilToFertilizer,
		Fertilizer: fertilizerToWater,
		Water: waterToLight,
		Light: lightToTemperature,
		Temperature: temperatureToHumidity,
		Humidity: humidityToLocation,
	}

	fmt.Println("Parsing seeds")
	parseSeeds(input[0], seedToSoil)
	fmt.Printf("Number of seeds to plant: %d \n", len(seedToSoil))

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

	for source := range humidityToLocation {
		if locations := humidityToLocation[source]; len(locations) == 0 {
			humidityToLocation[source] = append(humidityToLocation[source], source)
		}
	}

	var minLocs []int

	for seed := range seedToSoil {
		fmt.Println("Processing seed: ", seed)
		minLocs = append(minLocs, findMinLocation(seed, Seed, resourceTypeMap))
	}
	log.Printf("Min location: %v", slices.Min(minLocs))

	// Print all the maps
	// for k, v := range seedToSoil {
	// 	log.Printf("Seed %d -> Soil %v", k, v)
	// }

	// // Create a slice to hold the keys
	// keys := make([]int, 0, len(seedToSoil))

	// // Add keys to the slice
	// for key := range seedToSoil {
	// 	keys = append(keys, key)
	// }

	// // Sort the slice of keys
	// sort.Ints(keys)

	// // Print sorted keys
	// for _, key := range keys {
	// 	fmt.Println(key)
	// }

	// for k, v := range soilToFertilizer {
	// 	log.Printf("Soil %d -> Fertilizer %v", k, v)
	// }

	// for k, v := range fertilizerToWater {
	// 	log.Printf("Fertilizer %d -> Water %v", k, v)
	// }

	// for k, v := range waterToLight {
	// 	log.Printf("Water %d -> Light %v", k, v)
	// }

	// for k, v := range lightToTemperature {
	// 	log.Printf("Light %d -> Temperature %v", k, v)
	// }

	// for k, v := range temperatureToHumidity {
	// 	log.Printf("Temperature %d -> Humidity %v", k, v)
	// }

	// for k, v := range humidityToLocation {
	// 	log.Printf("Humidity %d -> Location %v", k, v)
	// }

	// DFS on this tree
	// [seed1, seed2....]
	// [soil1, soil2....]


}
