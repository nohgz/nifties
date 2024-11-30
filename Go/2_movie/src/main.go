package main

import (
	"bufio"
	"fmt"
	"math"
	"os"

	sc "strconv"
	s "strings"
)

// -----------------------------------------------------------------------------
// Nifty Movie Review Sentiment Analysis
//
// Description:
// Go implementation of Stanford Nifty Assignment "Movie Review Sentiment Analysis"
// http://nifty.stanford.edu/2016/manley-urness-movie-review-sentiment/
//
// Uses a hashmap to score and index word sentiments for speeeed
//
// Dependencies: none
// -----------------------------------------------------------------------------

type Score struct {
	freq  int
	score float64
}

var SENTIMENTS = map[string]Score{}

func main() {
	populateWordMap(SENTIMENTS, "assets/movieReviews.txt")

	for {
		fmt.Println("\nWhat would you like to do?")
		fmt.Println("1: Get the score of a word")
		fmt.Println("2: Get the average score of words in a file (one word per line)")
		fmt.Println("3: Find the highest/lowest scoring words in a file")
		fmt.Println("4: Sort words from a file into positivewords.txt and negativewords.txt")
		fmt.Println("5: Exit the program")

		var choice uint8
		fmt.Print("\nEnter your choice:\n==> ")
		fmt.Scanln(&choice)

		var input = ""

		switch choice {
		case 1:
			fmt.Print("\nEnter your word here\n==> ")
			fmt.Scanln(&input)
			displayWordInfo(SENTIMENTS, input)
		case 2:
			fmt.Print("\nEnter filepath here\n==> ")
			fmt.Scanln(&input)
			displayFileSentiment(SENTIMENTS, input)
		case 3:
			fmt.Print("\nEnter filepath here\n==> ")
			fmt.Scanln(&input)
			displayExtremeSentiments(SENTIMENTS, input)
		case 4:
			fmt.Print("\nEnter filepath here\n==> ")
			fmt.Scanln(&input)
			writeExtremeSentiments(SENTIMENTS, input)
		case 5:
			fmt.Print("\nGoodbye!")
			return
		default:
			fmt.Print("\nInvalid command!\n")
		}
	}
}

func populateWordMap(wordMap map[string]Score, filePath string) error {
	// Open the file
	file, err := os.Open(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	// Create a buffered reader to read the john line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		// Split each line into the score and review part
		parts := s.SplitN(line, " ", 2)
		if len(parts) < 2 {
			// skips empty lines
			continue
		}
		scoreStr, review := parts[0], parts[1]

		// Parse the score into a float
		score, err := sc.ParseFloat(scoreStr, 64)
		if err != nil {
			continue // skip lines with invalid scores
		}

		// inserts a unique word and adds the review score to a total
		// so its average score can be calculated later when needed
		//
		// this way saves CPU cycles so it doesn't calculate an average
		// score of a word no one cares about
		for _, word := range s.Fields(review) {
			entry := wordMap[word]
			entry.freq++
			entry.score += score
			wordMap[word] = entry
		}
	}

	// Check for scanning errors
	if err := scanner.Err(); err != nil {
		return err
	}

	return nil
}

func getWordSentiment(wordMap map[string]Score, word string) float64 {
	stats := wordMap[word]
	if stats.freq != 0 {
		return stats.score / float64(stats.freq)
	} else {
		return -1.0
	}
}

func displayWordInfo(wordMap map[string]Score, word string) {
	stats := wordMap[word]
	if stats.freq == 0 {
		fmt.Printf("\nThe word %s doesn't appear in the database!\n", word)
	} else {
		fmt.Printf("\nThe word %s appears %d times with an average score of %f.\n",
			word, stats.freq, stats.score/float64(stats.freq))
	}
}

func displayFileSentiment(wordMap map[string]Score, filePath string) {
	// Open the file
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("\nERROR: Failed to open the file located at %s\n", filePath)
		return
	}
	defer file.Close()

	totalAverageScore := 0.0
	wordCount := 0

	// Create a buffered reader to read the file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) < 1 {
			// skips empty lines
			continue
		}

		// if word is valid, then tally it
		sentiment := getWordSentiment(wordMap, line)
		if sentiment >= 0 {
			totalAverageScore += sentiment
			wordCount++
		}
	}
	// Check for scanning errors
	if err := scanner.Err(); err != nil {
		fmt.Printf("\nERROR: Encountered a scanning issue.\n")
		return
	}

	fileSentiment := totalAverageScore / float64(wordCount)

	var rating string

	if fileSentiment >= 2.0 {
		rating = "positive"
	} else {
		rating = "negative"
	}

	fmt.Printf("\nThe sentiment of %s is %s, with a score of %f.\n", filePath, rating, fileSentiment)
}

func displayExtremeSentiments(wordMap map[string]Score, filePath string) {
	// Open the file
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("\nERROR: Failed to open the file located at %s\n", filePath)
		return
	}
	defer file.Close()

	max_score := 0.0
	min_score := math.MaxFloat64

	var max_word string
	var min_word string

	// Create a buffered reader to read the file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		sentiment := getWordSentiment(wordMap, line)
		// if its a word that is not in the database, then skip it
		if sentiment < 0 {
			continue
		}

		if sentiment > max_score {
			max_score = sentiment
			max_word = line
		}
		if sentiment < min_score {
			min_score = sentiment
			min_word = line
		}
	}

	fmt.Printf("\nThe most positive word is %s, with a score of %f.\n", max_word, max_score)
	fmt.Printf("\nThe most negative word is %s, with a score of %f.\n", min_word, min_score)
}

func writeExtremeSentiments(wordMap map[string]Score, filePath string) {
	// Open the specified file
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("\nERROR: Failed to open the file located at %s\n", filePath)
		return
	}
	defer file.Close()

	// Open two files for writing
	posFile, err := os.Create("positive.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer posFile.Close()

	negFile, err := os.Create("negative.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer posFile.Close()

	// Create a buffered reader to read the file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		sentiment := getWordSentiment(wordMap, line)
		if sentiment > 2.1 {
			fmt.Fprintln(posFile, line)
		}
		if sentiment < 1.9 {
			fmt.Fprintln(negFile, line)
		}
	}

}
