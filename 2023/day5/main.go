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

	input := strings.Split(string(file), "\n\n")

	processAlmanac(input)
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

func getUpdatedIntervals(resourceInput []string, sourceIntervals []interval) []interval {
	var result []interval

	for _, inter := range sourceIntervals {
		interModified := false
		for i := 0; i <= len(resourceInput)-1; i++ {
			resourcesRange := strings.Fields(resourceInput[i])
			destinationStart := strToInt(resourcesRange[0])
			sourceStart := strToInt(resourcesRange[1])
			rangeLength := strToInt(resourcesRange[2])
			sourceEnd := sourceStart + rangeLength

			overlapStart := max(inter.start, sourceStart)
			overlapEnd := min(inter.end, sourceEnd)

			if overlapStart < overlapEnd {
				interModified = true
				result = append(result, interval{
					start: overlapStart + (destinationStart - sourceStart),
					end:   overlapEnd + (destinationStart - sourceStart),
				})

				if overlapStart > inter.start {
					result = append(result, interval{
						start: inter.start,
						end:   overlapStart,
					})
				}

				if overlapEnd < inter.end {
					result = append(result, interval{
						start: overlapEnd,
						end:   inter.end,
					})
				}
				break
			}
		}

		if !interModified {
			result = append(result, inter)
		}
	}

	return result
}

func processAlmanac(input []string) {
	interval := parseSeeds(input[0])


	for i := 1; i <= len(input)-1; i++ {
		inputSlice := strings.Split(input[i], "\n")[1:]

		interval = getUpdatedIntervals(inputSlice, interval)
	}

	minLocation := math.MaxInt64

	for _, inter := range interval {
		if inter.start < minLocation {
			minLocation = inter.start
		}
	}

	fmt.Println("Min location, ", minLocation)

}
