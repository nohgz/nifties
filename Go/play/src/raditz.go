package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	// Create a new random number generator with a seeded source
	rng := rand.New(rand.NewSource(time.Now().UnixNano()))

	// Create a slice of ints with a size of 3
	numbers := make([]int, 3)

	// Define a variable to store the sum
	sum := 0

	// Assign random integers between 0 and 100 to the slice using a for loop
	for i := 0; i < len(numbers); i++ {
		numbers[i] = rng.Intn(101) // Random number between 0 and 100
		sum += numbers[i]          // Add to the sum
	}

	// Output each number individually
	fmt.Println("Numbers:")
	for _, num := range numbers {
		fmt.Println(num)
	}

	// Output the sum
	fmt.Printf("Sum: %d\n", sum)
}
