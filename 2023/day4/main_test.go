package main

import "testing"

func TestProcessCards(t *testing.T) {
	testcases := []struct {
		name                string
		cards               []string
		points              int
		totalCardsProcessed int
	}{
		{
			name: "Basic case",
			cards: []string{
				"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
				"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
				"Card 3: 21  1 53 59 44 | 69 82 63 72 16 21 14  1",
				"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
				"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
				"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
			},
			points:              13,
			totalCardsProcessed: 30,
		},
		// {
		// 	name: "Case 2",
		// 	cards: []string{
		// 		"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 83 17",
		// 		"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
		// 		"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
		// 		"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
		// 		"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
		// 		"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
		// 	},
		// 	points: 13,
		// },
		// {
		// 	name: "Case 3",
		// 	cards: []string{
		// 		"Card   1: 75 68 35 36 86 83 30 11 14 59 | 86 25 63 57 59 91 68 14 72 32 36 74 66 44 30 28 11 35 75 34 55 83 69 56 38",
		// 		"Card   2: 49 62 66 89 53 16 59 19 58 99 | 99 29 21 59 53 66  1 77 15 92 98 23  9 49 75  4 16 12 62 89 58 82 19 60 14",
		// 		"Card   3: 37 77  5 90 41 15 46 27 38 53 | 47 27 41 90 77 53 65 50 69 72 37 91  9 31 67 11 46 56 85 49 15 20 40  5 38",
		// 		"Card   4: 97 24 29 70 37 95 83 78 66 19 | 24 44 21 29 39 51 78 79 66 97 19 88 89 35 83 95 84 70  6 34 62 32 37 72 80",
		// 		"Card   5: 41 58 67 35 33 36 73 70 64 55 | 93 29 77 60 56 35 68 53  2 55  3 92 81 78  8 30 87 73 64 85 16 20 33  5 66",
		// 		"Card   6: 25 72 59 52 79  4 17 15 69 41 | 98 84 36 15 71 67 53 34 26 48 43 90 94 89 85 81 45 29 47 75  7 82 27 19 96",
		// 	},
		// 	points: 13,
		// },
	}

	for _, testCase := range testcases {
		t.Run(testCase.name, func(t *testing.T) {
			_, totalCardsProcessed := processCards(testCase.cards)

			if totalCardsProcessed != testCase.totalCardsProcessed {
				t.Errorf("Expected %d cards, got %d", testCase.totalCardsProcessed, totalCardsProcessed)
			}
		})
	}
}
