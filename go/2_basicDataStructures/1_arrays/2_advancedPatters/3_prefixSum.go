package arraysAdvancedPatterns

func CalculatePrefixSum(arr []int) []int {
	prefixSum := make([]int, len(arr))
	prefixSum[0] = arr[0]

	for i := 1; i < len(arr); i++ {
		prefixSum[i] = prefixSum[i-1] + arr[i]
	}

	return prefixSum
}

/*
	 This function returns the sum of elements between left and right indices (inclusive) in the original array(from CalculatePrefixSum)
	 Eg)
		orginalArray := arr := []int{2, 4, 6, 8}
		prefixSum := [2, 6, 12, 20]  // from the earlier example
		RangeSum(prefixSum, 1, 3)   // means: sum of arr[1:4] → 4 + 6 + 8 = 18
*/
func RangeSum(prefixSum []int, left, right int) int {
	if left == 0 {
		return prefixSum[right]
	}
	return prefixSum[right] - prefixSum[left-1]
}
