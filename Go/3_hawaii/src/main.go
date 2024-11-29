package main

import (
	"fmt"
	s "strings"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

// -----------------------------------------------------------------------------
// Nifty Hawaiian Phonetic Generator
//
// Description:
// Go implementation of Stanford Nifty Assignment "Hawaiian Phonetic Generator"
// http://nifty.stanford.edu/2019/bingham-hawaiian-phonetic-generator/
//
// -----------------------------------------------------------------------------

func main() {
	for {
		var word string
		fmt.Print("\nInput a Hawaiian Word Here\n==> ")
		fmt.Scanln(&word)

		if validizeWord(word) {
			fmt.Printf("\n%s is pronounced as %s.\n", word, pronounceSentence(word))
		}

		fmt.Println("\nWould you like to play again?")
		fmt.Println("  [Enter] - Yes")
		fmt.Println("  [N] - No")
		fmt.Print("\n==> ")

		fmt.Scanln(&word)

		if s.Trim(s.ToLower(word), " ") == "n" {
			break
		}
	}
}

var VOWELS = map[string]string{
	"a": "ah",
	"e": "eh",
	"i": "ee",
	"o": "oh",
	"u": "oo",
}

var VOWEL_PAIRS = map[string]string{
	"ai": "eye",
	"ae": "eye",
	"ao": "ow",
	"au": "ow",
	"ei": "ay",
	"eu": "eh-oo",
	"iu": "ew",
	"oi": "oy",
	"ou": "ew",
	"ui": "ooey",
}

func validizeWord(word string) bool {
	sanitizedWord := cases.Lower(language.English, cases.Compact).String(word)

	if sanitizedWord == "" {
		fmt.Println("\nYou have to input something!")
		return false
	}

	// Check if all characters are valid Hawaiian characters
	for _, char := range sanitizedWord {
		if !s.ContainsRune(" aehiklmnopuw'", char) {
			fmt.Printf("%c is not a valid Hawaiian character!\n", char)
			return false
		}
	}
	return true
}

// helper method to abstract away the abomination that is cases
func capitalize(word string) string {
	return cases.Title(language.English, cases.Compact).String(word)
}

// Go-like copy of the rust implementation, which is based off of
// https://stackoverflow.com/questions/55291856/hawaiian-pronouncer
func pronounceWord(word string) string {
	chars := s.ToLower(word)
	i := 0
	result := []string{}

	for i < len(chars) {
		char := string(chars[i])
		var tr string = ""

		// Check for vowel pair
		if i < len(chars)-1 {
			pair := chars[i : i+2]
			if val, exists := VOWEL_PAIRS[pair]; exists {
				tr = val
			}
		}

		// if no vowel pair found, check single character vowel
		if tr == "" {
			if val, exists := VOWELS[char]; exists {
				tr = val
			}
		} else {
			// otherwise just go next
			i++
		}

		// Add a dash if it's not the last character
		if tr != "" && i < len(chars)-1 {
			tr += "-"
		}

		// Append the translation or the original character
		if tr != "" {
			result = append(result, tr)
		} else {
			result = append(result, char)
		}

		i++
	}

	return capitalize(s.Join(result, ""))
}

// Generalized case of pronounceWord
func pronounceSentence(sentence string) string {
	// Split sentence into words
	words := s.Fields(s.ToLower(sentence))
	var results []string

	for _, word := range words {
		results = append(results, pronounceWord(word))
	}

	// combines results arr into string with spaces between
	return s.Join(results, " ")
}
