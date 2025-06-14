package bigonotation

/* CONSTANT TIME O(1) */

// O(1) time complexity
func GetFirstElement(arr []int) int {
	return arr[0] // Direct access, always one operation
}

// O(1) space complexity
func SwapNumbers(a, b int) (int, int) {
	return b, a
}
