package main

import (
	"log"
	"os"
	"strings"
)

type tile struct {
	r int
	c int
}

type direction tile

var (
	N = direction{r: -1, c: 0}
	S = direction{r: 1, c: 0}
	W = direction{r: 0, c: -1}
	E = direction{r: 0, c: 1}
)

// Rule to decide which directions to go to next
// based on the current direction and the tile type
var rules = map[direction]map[string][]direction{
	N: {
		"|":  {N},
		"-":  {W, E},
		"\\": {W},
		"/":  {E},
		".":  {N},
	},
	S: {
		"|":  {S},
		"-":  {W, E},
		"\\": {E},
		"/":  {W},
		".":  {S},
	},
	W: {
		"|":  {N, S},
		"-":  {W},
		"\\": {N},
		"/":  {S},
		".":  {W},
	},
	E: {
		"|":  {N, S},
		"-":  {E},
		"\\": {S},
		"/":  {N},
		".":  {E},
	},
}

// energize Mirrors uses recursion (DFS) to go in every direction possible and collect
// energized Tiles.
// Breaking when the light exits the grid
// or when we visit the same with the same direction (which means there's a cycle)
func energizeMirrors(grid []string, dir direction, sRow, sCol int, totalEnergizedTiles map[tile][]direction) {
	// If the current position is outside the grid, return the totalEnergizedTiles
	curTile := tile{r: sRow, c: sCol}

	if sRow < 0 || sRow >= len(grid) || sCol < 0 || sCol >= len(grid[0]) {
		return
	}

	if visited, ok := totalEnergizedTiles[curTile]; ok {
		for _, v := range visited {
			if v == dir {
				return
			}
		}
	}

	tileType := grid[sRow][sCol]
	totalEnergizedTiles[curTile] = append(totalEnergizedTiles[curTile], dir)

	nextDirs := rules[dir][string(tileType)]

	for _, nextDir := range nextDirs {
		energizeMirrors(grid, nextDir, sRow+nextDir.r, sCol+nextDir.c, totalEnergizedTiles)
	}

}

func processMirrorGrid(grid []string) int {
	// Part 1

	totalEnergizedTiles := map[tile][]direction{}
	energizeMirrors(grid, E, 0, 0, totalEnergizedTiles)

	// Part 2

	// Prepare a list of possible starting tiles and its directions
	startingTiles := map[tile][]direction{}

	startRows := map[int]direction{0: S, len(grid) - 1: N}
	startCols := map[int]direction{0: E, len(grid[0]) - 1: W}

	for row := 0; row <= len(grid)-1; row++ {
		for col, dir := range startCols {
			nTile := tile{r: row, c: col}
			startingTiles[nTile] = append(startingTiles[nTile], dir)
		}
	}

	for col := 0; col <= len(grid[0])-1; col++ {
		for row, dir := range startRows {
			nTile := tile{r: row, c: col}
			startingTiles[nTile] = append(startingTiles[nTile], dir)
		}
	}

	maxEnergizedTiles := -1

	for curTile, dirs := range startingTiles {
		for _, dir := range dirs {
			totalEnergizedTiles := map[tile][]direction{}
			energizeMirrors(grid, dir, curTile.r, curTile.c, totalEnergizedTiles)
			if len(totalEnergizedTiles) > maxEnergizedTiles {
				maxEnergizedTiles = len(totalEnergizedTiles)
			}
		}
	}

	return maxEnergizedTiles
}

func main() {
	file, err := os.ReadFile("2023/day16/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	processMirrorGrid(strings.Split(string(file), "\n"))
}
