package bigonotation

// Time Complexity: O(2ⁿ)
// Space Complexity: O(n)
func Fibonnacci(n int) int {
	if n <= 1 {
		return n
	}
	return Fibonnacci(n-1) + Fibonnacci(n-2)
}

// Time Complexity: O(n)
// Space Complexity: O(n)
func FibonnacciMemoized(n int, memo map[int]int) int {
	if value, exixts := memo[n]; exixts {
		return value
	}

	if n <= 1 {
		return n
	}

	memo[n] = FibonnacciMemoized(n-1, memo) + FibonnacciMemoized(n-2, memo)
	return memo[n]
}
