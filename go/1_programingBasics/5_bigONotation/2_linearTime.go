package bigonotation

/* LINEAR TIME O(n) */

// O(n) time complexity
func FindSum(arr []int) int {
	sum := 0
	for _, v := range arr { // Loops through n elements
		sum += v
	}
	return sum
}

// O(n) space complexity
func CopyArray(arr []int) []int {
	newArr := make([]int, len(arr))
	copy(newArr, arr)
	return newArr
}
