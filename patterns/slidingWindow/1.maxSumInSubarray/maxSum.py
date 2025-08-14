from typing import List


def maxSum(arr: List[int], k: int) -> int:
    n = len(arr)

    if n < k:
        return -1
    elif n == k:
        return sum(arr)

    window_sum = sum(arr[:k])
    max_sum = window_sum

    for i in range(n - k):
        window_sum = window_sum - arr[i] + arr[i + k]
        max_sum = max(max_sum, window_sum)

    return max_sum


if __name__ == "__main__":
    arr = [5, 2, -1, 0, 3]
    k = 3
    print(maxSum(arr, k))
