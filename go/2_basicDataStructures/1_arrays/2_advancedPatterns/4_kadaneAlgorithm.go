package arraysAdvancedPatterns

func MaxSubArraySum(arr []int) int {
	if len(arr) == 0 {
		return 0
	}

	currentSum := arr[0]
	maxSum := arr[0]

	for i := 1; i < len(arr); i++ {
		currentSum = max(arr[i], currentSum+arr[i])
		maxSum = max(maxSum, currentSum)
	}

	return maxSum
}
