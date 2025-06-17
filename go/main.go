package main

import (
	arrays "dsa/2_basicDataStructures/1_arrays"
	"fmt"
)

func main() {
	nums := []int{1, 4, 2, 10, 23, 3, 1, 0, 20}
	k := 4
	result := arrays.MaxSum(nums, k)
	fmt.Println("Maximum sum of subarray of size", k, "is:", result)
}
