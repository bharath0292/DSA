package bigonotation

// O(n²) time complexity
func BubbleSort(arr []int) []int {
	n := len(arr)

	for i := range n {
		swapped := false
		for j := range n - i - 1 {
			if arr[j] > arr[j+1] {
				arr[j], arr[j+1] = arr[j+1], arr[j]
				swapped = true
			}
		}

		if !swapped {
			break
		}
	}

	return arr
}

// O(n²) space complexity
func CreateMatrix(n int) [][]int {
	matrix := make([][]int, n) // Create n*n matrix
	for i := range n {
		matrix[i] = make([]int, n)
	}

	return matrix
}
