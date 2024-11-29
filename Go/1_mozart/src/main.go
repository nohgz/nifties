package main

import (
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	"github.com/faiface/beep"
	"github.com/faiface/beep/speaker"
	"github.com/faiface/beep/wav"
)

// -----------------------------------------------------------------------------
// Nifty Mozart Dice Game
//
// Description:
// Go implementation of Stanford Nifty Assignment "Mozart Musical Dice Game"
// http://nifty.stanford.edu/2023/wayne-musical-dice-game/
//
// Usage:
// Run with the instrument name as an argument. Example:
//   mozart.exe piano
//
// -----------------------------------------------------------------------------

func main() {

	// Handle user input
	if len(os.Args) != 2 {
		fmt.Printf("\n Usage: %s <clarinet / flute-harp / mbira / piano> \n", os.Args[0])
		return
	}

	fmt.Printf("\nPlaying a bunch of random %s parts and still sounding good...\n", os.Args[1])

	const sampleRate = 44100
	speaker.Init(beep.SampleRate(sampleRate), beep.SampleRate(sampleRate).N(time.Second/10))

	// Create a global buffer to hold all audio data. I did it this
	// way in rust and it made the most sense
	buffer := beep.NewBuffer(beep.Format{
		SampleRate:  beep.SampleRate(sampleRate),
		NumChannels: 2,
		Precision:   2,
	})

	// Append audio data for both parts into the buffer
	appendPartsToBuffer(buffer, os.Args[1], "minuet", 16)
	appendPartsToBuffer(buffer, os.Args[1], "trio", 16)

	// Play the full sequence from the buffer
	streamer := buffer.Streamer(0, buffer.Len())
	done := make(chan bool)
	speaker.Play(beep.Seq(streamer, beep.Callback(func() { close(done) })))
	<-done // Wait for playback to finish
}

// createFileName generates a randomized file name
func createFileName(instrument, part string, phrase int) string {
	diceRoll := rand.Intn(11) + 2 // Minuet: 2-12
	if part == "trio" {
		diceRoll = rand.Intn(6) + 1 // Trio: 1-6
	}
	return fmt.Sprintf("assets/%s/%s%d-%d.wav", instrument, part, phrase, diceRoll)
}

// appendPartsToBuffer decodes and appends audio data into the buffer
func appendPartsToBuffer(buffer *beep.Buffer, instrument, part string, count int) {
	for i := 0; i < count; i++ {
		fileName := createFileName(instrument, part, i)

		// open and decode file
		f, err := os.Open(fileName)
		if err != nil {
			log.Fatalf("Failed to open file %s: %v", fileName, err)
		}
		defer f.Close()

		streamer, format, err := wav.Decode(f)
		if err != nil {
			log.Fatalf("Failed to decode file %s: %v", fileName, err)
		}
		defer streamer.Close()

		// resample the audio to handle different rates
		resampled := beep.Resample(4, format.SampleRate, buffer.Format().SampleRate, streamer)

		// Append the resampled audio to the buffer
		buffer.Append(resampled)
	}
}
