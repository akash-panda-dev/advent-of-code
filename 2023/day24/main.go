package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

/*
b=−mx1 + y1
​m=y2-y1/x2-x1

Using this we can get the intersection point of two lines

x_int = (b2 - b1) / (m1 - m2)
y_int = m1 * x_int + b1

*/

type xyz struct {
	x float64
	y float64
	z float64
}

type location xyz
type velocity xyz

type hailstone struct {
	loc  location
	velo velocity
}

type Point struct {
	x float64
	y float64
}

// Vector between two points
func vector(a, b Point) Point {
	return Point{b.x - a.x, b.y - a.y}
}

// Dot product of two vectors
func dotProduct(v1, v2 Point) float64 {
	return v1.x*v2.x + v1.y*v2.y
}

func isIntersectionInPast(intsecPoint Point, pStone Point, fStone Point) bool {
	// Check if AC dot AB is less than 0
	vecAC := vector(pStone, intsecPoint)
	vecAB := vector(pStone, fStone)
	return dotProduct(vecAC, vecAB) < 0
}

func parseHailstones(hailStoneSpec[] string) []hailstone {
	var stones []hailstone
	
	for _, spec := range hailStoneSpec {
		// 19, 13, 30 @ -2,  1, -2
		var x1, y1, z1, vx1, vy1, vz1 float64
		fmt.Sscanf(spec, "%f, %f, %f @ %f, %f, %f", &x1, &y1, &z1, &vx1, &vy1, &vz1)

		stones = append(stones, hailstone{
			loc: location{
				x: x1,
				y: y1,
				z: z1,
			},
			velo: velocity{
				x: vx1,
				y: vy1,
				z: vz1,
			},
		})
	}

	return stones
}

func getFutureHailstone(presentStone hailstone) hailstone {
	return hailstone{
		loc: location{
			x: presentStone.loc.x + presentStone.velo.x,
			y: presentStone.loc.y + presentStone.velo.y,
			z: presentStone.loc.z + presentStone.velo.z,
		},
		velo: velocity{
			x: presentStone.velo.x,
			y: presentStone.velo.y,
			z: presentStone.velo.z,
		},
	}
}

func getIntersectionsCount(hailStoneSpec []string) int {
	futureIntsecCount := 0
	hailstones := parseHailstones(hailStoneSpec)
	// Define limits for x and y coordinates
	xLimits := struct{ min, max float64 }{min: 200000000000000, max: 400000000000000}
	yLimits := struct{ min, max float64 }{min: 200000000000000, max: 400000000000000}

	for i := 0; i < len(hailstones); i++ {
		for j := i + 1; j < len(hailstones); j++ {
			pStone1 := hailstones[i]
			fStone1 := getFutureHailstone(pStone1)
			slope1 := (pStone1.loc.y - fStone1.loc.y) / (pStone1.loc.x - fStone1.loc.x) // m = y2-y1/x2-x1
			b1 := pStone1.loc.y - (slope1 * pStone1.loc.x) // b = y - mx

			pStone2 := hailstones[j]
			fStone2 := getFutureHailstone(pStone2)
			slope2 := (pStone2.loc.y - fStone2.loc.y) / (pStone2.loc.x - fStone2.loc.x) // m = y2-y1/x2-x1
			b2 := pStone2.loc.y - (slope2 * pStone2.loc.x) // b = y - mx

			if slope1 == slope2 {
				continue // Parallel lines
			}

			xIsect := (b2 - b1) / (slope1 - slope2)
			yIsect := (slope1 * xIsect) + b1

			if isIntersectionInPast(Point{x: xIsect, y: yIsect}, Point{x: pStone1.loc.x, y: pStone1.loc.y}, Point{x: fStone1.loc.x, y: fStone1.loc.y}) || 
				isIntersectionInPast(Point{x: xIsect, y: yIsect}, Point{x: pStone2.loc.x, y: pStone2.loc.y}, Point{x: fStone2.loc.x, y: fStone2.loc.y}) {
				continue
			}

			if xIsect >= xLimits.min && xIsect <= xLimits.max && yIsect >= yLimits.min && yIsect <= yLimits.max{
				futureIntsecCount++
			}
		}
	}

	return futureIntsecCount
}

func main() {
	file, err := os.ReadFile("2023/day24/input.txt")

	if err != nil {
		log.Fatalf("Failed to read the file, %v", err)
	}

	fmt.Println("Part 1 - Number of intersections:", getIntersectionsCount(strings.Split(string(file), "\n")))
}
