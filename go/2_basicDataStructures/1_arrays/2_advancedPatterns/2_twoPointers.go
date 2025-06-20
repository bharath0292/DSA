package arraysAdvancedPatterns

// Find a pair with given sum (only sorted array)
func FindPairSum(arr []int, target int) (int, int) {
	left, right := 0, len(arr)-1

	for left < right {
		current := arr[left] + arr[right]
		if current == target {
			return left, right
		}
		if current < target {
			left++
		} else {
			right--
		}
	}
	return -1, -1
}

// Remove duplicates from sorted array (only sorted array)
func RemoveDuplicates(arr []int) int {
	// If the input array is empty, there's nothing to process, so return 0.
	if len(arr) == 0 {
		return 0
	}

	// writePtr is used to keep track of where the next unique value should be placed.
	// Index 0 is always considered unique, so we start at index 1.
	writePtr := 1

	/*
		- readPtr scans through the array starting from index 1.
		- At each step:
			* Compare current value arr[readPtr] with previous value arr[readPtr - 1].
			* If they are different, that means a new unique element is found:
				~ Copy arr[readPtr] to arr[writePtr].
				~ Move writePtr forward.
		- If values are the same, it skips them (thus removing duplicates logically).
	*/
	for readPtr := 1; readPtr < len(arr); readPtr++ {
		if arr[readPtr] != arr[readPtr-1] {
			arr[writePtr] = arr[readPtr]
			writePtr++
		}
	}

	return writePtr
}
