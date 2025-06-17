package arrays

import "slices"

func ArrayExamples() {
	// Fixed size array
	var arr [5]int = [5]int{1, 2, 3, 4, 5}
	println(arr)

	// Array with implicit size
	numbers := [...]int{1, 2, 3, 4, 5}
	println(numbers)

	// Slices (dynamic arrays)
	slice := make([]int, 5, 10)
	println(slice)

	// Slice operations
	slice = append(slice, 6) // Adding elements
	subSlice := slice[1:4]   // Slicing
	println(subSlice)

	// 2D array
	matrix := [3][3]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}
	println(matrix)
}

// Common slice operation
func SliceOperations() {
	numbers := [...]int{1, 2, 3, 4, 5}

	// Copy Slice
	dest := make([]int, len(numbers))
	copy(dest, numbers[:])

	// Appending multiple elements
	numbersSlice := numbers[:]                   // convert array to slice
	numbersSlice = append(numbersSlice, 6, 7, 8) // append multiple elements

	// Remove element at index
	i := 2
	//numbersSlice = append(numbersSlice[:i], numbersSlice[i+1:]...) // method 1 => append
	numbersSlice = slices.Delete(numbersSlice, i, i+1) // method 1 => slices.Delete

	// Insert element at index i
	i = 1
	valToInsert := 10
	numbersSlice = append(numbersSlice[:i], append([]int{valToInsert}, numbersSlice[i:]...)...)
}

// Common Slice patterns
func SlicePatterns() {
	// Filter
	nums := []int{1, 2, 3, 4, 5}
	evenNums := make([]int, 0)

	for _, n := range nums {
		if n%2 == 0 {
			evenNums = append(evenNums, n)
		}
	}

	// Map
	doubles := make([]int, len(nums))
	for i, n := range nums {
		doubles[i] = n * 2
	}

	// Reduce
	sum := 0
	for _, n := range nums {
		sum += n
	}
}
