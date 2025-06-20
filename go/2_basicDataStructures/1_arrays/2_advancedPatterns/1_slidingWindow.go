package arraysAdvancedPatterns

func MaxSum(arr []int, k int) int {
	n := len(arr)

	if n < k {
		return 0 // not enough elements
	}

	/*
		We sum the first k elements: this is our initial window.
		Example: if arr = [1, 4, 2, 10, 23] and k = 3,
			→ windowSum = 1 + 4 + 2 = 7
	*/
	windowSum := 0
	for i := range k {
		windowSum += arr[i]
	}

	maxSum := windowSum

	/*
		This is where the sliding window magic happens:
			- arr[i] is the new element entering the window.
			- arr[i-k] is the element exiting the window (from k steps ago).
		By adding one and subtracting one, you avoid recomputing the whole window.

		Example:
			Suppose k = 3, and you already summed arr[0] + arr[1] + arr[2] = 7
			Next window should be arr[1] + arr[2] + arr[3]
			Instead of summing all 3 again,
			Just do: prev_sum - arr[0] + arr[3]
	*/
	for i := k; i < n; i++ {
		windowSum += arr[i] - arr[i-k]
		if windowSum > maxSum {
			windowSum = maxSum
		}
	}

	return maxSum
}

func LongestUniqueString(s string) int {

	charIndex := make(map[rune]int, len(s))
	start := 0
	maxLength := 0

	for end, char := range s {
		/*
			If the character has already been seen AND its last seen index is >= current window start,
			then move the start just after the last seen index.
			This effectively skips over the duplicate character.
		*/
		if lastSeenIndex, exists := charIndex[char]; exists && lastSeenIndex >= start {
			start = lastSeenIndex + 1
		}
		charIndex[char] = end

		/*
			Calculate the window length: from `start` to current index `i`
			Update maxLen if this window is longer than any previously found
		*/
		if end-start+1 > maxLength {
			maxLength = end - start + 1
		}
	}
	return maxLength
}
