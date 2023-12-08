package main

import "testing"

func TestProcessCards(t *testing.T) {
	input := []string{
		// "QQAAJ 1",
		// "AAKKK 2",
		"32T3K 765",
		"T55J5 684",
		"KK677 28",
		"KTJJT 220",
		"QQQJA 483",
	}

	result := processCards(input)

	if result != 5905 {
		t.Errorf("Expected %d, Got %d", 5905, result)
	}
}

func TestGetHandType(t *testing.T) {
	tests := []struct {
		cards    string
		expected HandType
	}{
		{"ABCDE", HighCard},
		{"AACDE", Pair},
		{"AAACD", ThreeOfAKind},
		{"AAAAB", FourOfAKind},
		{"AAAAJ", FiveOfAKind}, 
		{"AAABB", FullHouse},
		{"AABBC", TwoPair},
		{"AJJJJ", FiveOfAKind}, 
		{"AJJJD", FourOfAKind}, 
		{"AJJDD", FourOfAKind}, 
		{"AJDDD", FourOfAKind}, 
		{"AAAJJ", FiveOfAKind}, 
		{"AAJJJ", FiveOfAKind}, 
		{"AABJJ", FourOfAKind}, 
		{"ABBJJ", FourOfAKind}, 
		{"AABBJ", FullHouse},   
		{"ABBJJ", FourOfAKind}, 
		{"ABJJK", ThreeOfAKind},
		{"ABJKK", ThreeOfAKind},
		{"AAJKK", FullHouse},   
		{"AJKKK", FourOfAKind}, 
		{"AABJK", ThreeOfAKind},
		{"ABBJK", ThreeOfAKind},
		{"ABJKL", Pair},        
		{"AJKLM", Pair},    
	}

	for _, test := range tests {
		result := getHandType(test.cards)

		if result != test.expected {
			t.Errorf("Expected %v, Got %v for card: %v", test.expected, result, test.cards)
		}
	}
}