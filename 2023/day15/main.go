package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("2023/day15/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	processSeq(strings.Split(string(file), ","))
}

type Step struct {
	label string
	focus int
}

func hashSeq(seq string) int {
	var hash int

	for _, v := range seq {
		hash = (hash + int(v)) * 17 % 256
	}

	return hash
}

func itemExists(slice []Step, item Step) (bool, int) {
	for i, a := range slice {
		if a.label == item.label {
			return true, i
		}
	}
	return false, -1
}

func strToInt(str string) int {
	num, err := strconv.Atoi(strings.TrimSpace(str))

	if err != nil {
		log.Fatalf("Failed to convert the string to int: %v", err)
		
	}

	return num
}

func unMarshalStep(step string) Step {
	if strings.Contains(step, "=") {
		splitStep := strings.Split(step, "=")
		return Step{label: splitStep[0], focus: strToInt(splitStep[1])}
	} else if strings.Contains(step, "-") {
		splitStep := strings.Split(step, "-")
		return Step{label: splitStep[0], focus: -1}
	}

	return Step{}
}

func processSeq(seq []string) {
	var totalHash int
	boxes := make(map[int][]Step, 256)

	for _, s := range seq {
		step := unMarshalStep(s)
		boxNum := hashSeq(step.label)

		switch {
		case strings.Contains(s, "="):
			if exists, index := itemExists(boxes[boxNum], step); exists {
				newSeq := append(boxes[boxNum][:index], step)
				newSeq = append(newSeq, boxes[boxNum][index+1:]...)
				boxes[boxNum] = newSeq
			} else {
				boxes[boxNum] = append(boxes[boxNum], step)
			}

		case strings.Contains(s, "-"):
			if exists, index := itemExists(boxes[boxNum], step); exists {
				newSeq := append(boxes[boxNum][:index], boxes[boxNum][index+1:]...)
				boxes[boxNum] = newSeq
			}
		}
	}

	for boxNum, steps := range boxes {
		for slot, step := range steps {
			totalHash += (boxNum + 1) * (slot + 1) * step.focus
		}
	}

	fmt.Println(totalHash)
}
