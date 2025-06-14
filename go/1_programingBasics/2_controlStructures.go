package topics

import "fmt"

// If-else statements
func CheckNumber(x int) {
	if x > 0 {
		fmt.Println("Positive")
	} else if x < 0 {
		fmt.Println("Negative")
	} else {
		fmt.Println("Zero")
	}
}

// loops
func LoopExamples() {
	// Basic for loop
	for i := range 5 {
		fmt.Println(i)
	}

	// While loop
	count := 0
	for count < 5 {
		count++
	}

	// Infinite loop
	for {
		break
	}

	numbers := []int{1, 2, 3, 4, 5, 6}
	for index, value := range numbers {
		fmt.Printf("Index: %d, Value: %d\n", index, value)
	}

	// Switch statement
	switch day := 4; day {
	case 1:
		fmt.Println("Monday")
	case 2:
		fmt.Println("Tuesday")
	default:
		fmt.Println("Other day")
	}
}
