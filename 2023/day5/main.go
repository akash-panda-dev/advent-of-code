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

func main() {
	file, err := os.ReadFile("2023/day5/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	processAlmanac(strings.Split(string(file), "\n"))
}

type SeedsRanges struct {
	start     int
	seedRange int
}

type interval struct {
	start int
	end   int
}

func parseSeeds(seedStr string) []interval {
	seedsNumsStr := strings.Split(seedStr, ":")[1]
	seedsNumsAndRanges := strings.Fields(seedsNumsStr)
	fmt.Println("Seed ranges", seedsNumsAndRanges)
	var seedsRanges []SeedsRanges
	var result []interval

	for i := 0; i <= len(seedsNumsAndRanges)-1; i += 2 {
		seedStart := strToInt(seedsNumsAndRanges[i])
		seedRange := strToInt(seedsNumsAndRanges[i+1])

		seedsRanges = append(seedsRanges, SeedsRanges{
			start:     seedStart,
			seedRange: seedRange,
		})
	}

	sort.Slice(seedsRanges, func(i, j int) bool {
		return seedsRanges[i].start < seedsRanges[j].start
	})

	for _, seed := range seedsRanges {
		result = append(result, interval{
			start: seed.start,
			end:   seed.start + seed.seedRange - 1,
		})
	}

	return result
}

func strToInt(str string) int {
	num, err := strconv.Atoi(strings.TrimSpace(str))

	if err != nil {
		log.Fatalf("Failed to convert the string to int: %v", err)
	}

	return num
}

func getUpdatedIntervals(resourceInput []string, sourceIntervals []interval, startIndex int) ([]interval, int) {
	var result []interval
	var endIndex int


	for _, inter := range sourceIntervals {
		interModified := false
		for i := startIndex; i <= len(resourceInput)-1; i++ {
			resourcesRange := strings.Fields(resourceInput[i])

			if len(resourcesRange) == 0 {
				break
			}

			destinationStart := strToInt(resourcesRange[0])
			sourceStart := strToInt(resourcesRange[1])
			rangeLength := strToInt(resourcesRange[2])
			sourceEnd := sourceStart + rangeLength

			if sourceStart <= inter.start && inter.end <= sourceEnd {
				interModified = true
				result = append(result, interval{
					start: inter.start + (destinationStart - sourceStart),
					end:   inter.end + (destinationStart - sourceStart),
				})
			} else if inter.start < sourceStart && sourceEnd < inter.end {
				interModified = true
				result = append(result,
					interval{
						start: inter.start,
						end:   sourceStart},
					interval{
						start: sourceStart + (destinationStart - sourceStart),
						end:   sourceEnd + (destinationStart - sourceStart),
					},
					interval{
						start: sourceEnd,
						end:   inter.end,
					})
			} else if inter.start < sourceStart && sourceStart < inter.end {
				interModified = true
				result = append(result,
					interval{
						start: inter.start,
						end:   sourceStart},
					interval{
						start: sourceStart + (destinationStart - sourceStart),
						end:   inter.end + (destinationStart - sourceStart),
					},
				)
			} else if inter.start < sourceEnd && sourceEnd < inter.end {
				interModified = true
				result = append(result,
					interval{
						start: inter.start + (destinationStart - sourceStart),
						end:   sourceEnd + (destinationStart - sourceStart),
					},
					interval{
						start: sourceEnd,
						end:   inter.end,
					})
			}
			endIndex = i
		}

		if !interModified {
			result = append(result, inter)
		}
	}

	return result, endIndex
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

func processAlmanac(input []string) {
	var (
	// seedToSoil            = make(map[int][]int)
	// soilToFertilizer      = make(map[int][]int)
	// fertilizerToWater     = make(map[int][]int)
	// waterToLight          = make(map[int][]int)
	// lightToTemperature    = make(map[int][]int)
	// temperatureToHumidity = make(map[int][]int)
	// humidityToLocation    = make(map[int][]int)
	// locationToDummyMap    = make(map[int][]int)
	)

	// resourceTypeMap := map[ResourceType]map[int][]int{
	// 	Seed:        seedToSoil,
	// 	Soil:        soilToFertilizer,
	// 	Fertilizer:  fertilizerToWater,
	// 	Water:       waterToLight,
	// 	Light:       lightToTemperature,
	// 	Temperature: temperatureToHumidity,
	// 	Humidity:    humidityToLocation,
	// }

	mapIds := map[string]struct{}{
		"seed-to-soil":            {},
		"soil-to-fertilizer":      {},
		"fertilizer-to-water":     {},
		"water-to-light":          {},
		"light-to-temperature":    {},
		"temperature-to-humidity": {},
		"humidity-to-location":    {},
	}

	fmt.Println("Parsing seeds")
	interval := parseSeeds(input[0])

	fmt.Println("Seeds parsed, ", interval)

	// interval = getUpdatedIntervals(input, interval, 1)

	for i := 1; i <= len(input)-1; {
		inputSlice := strings.Split(input[i], " ")

		if _, ok := mapIds[inputSlice[0]]; ok {
			interval, i = getUpdatedIntervals(input, interval, i+1)
		}

		i++
	}

	fmt.Println("Intervals updated, ", interval)

	minLocation := math.MaxInt32

	for _, inter := range interval {
		if inter.start < minLocation {
			minLocation = inter.start
		}
	}

	fmt.Println("Min location, ", minLocation)

	// for source := range humidityToLocation {
	// 	if locations := humidityToLocation[source]; len(locations) == 0 {
	// 		humidityToLocation[source] = append(humidityToLocation[source], source)
	// 	}
	// }

	// var minLocs []int

	// for seed := range seedToSoil {
	// 	minLocs = append(minLocs, findMinLocation(seed, Seed, resourceTypeMap))
	// }
	// log.Printf("Min location: %v", slices.Min(minLocs))

	// fmt.Printf("Number of seeds to plant: %d \n", len(seedToSoil))
	// fmt.Printf("Number of soils to plant: %d \n", len(soilToFertilizer))
	// fmt.Printf("Number of fertilizers to plant: %d \n", len(fertilizerToWater))
	// fmt.Printf("Number of waters to plant: %d \n", len(waterToLight))
	// fmt.Printf("Number of lights to plant: %d \n", len(lightToTemperature))
	// fmt.Printf("Number of temperatures to plant: %d \n", len(temperatureToHumidity))
	// fmt.Printf("Number of humidities to plant: %d \n", len(humidityToLocation))

	// // Print all the maps
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

	// for k, v := range locationToDummyMap {
	// 	log.Printf("Location %d -> Dummy %v", k, v)
	//}

	// DFS on this tree
	// [seed1, seed2....]
	// [soil1, soil2....]

}
