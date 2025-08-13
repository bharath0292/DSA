from typing import List


def twoSum(arr: List[int], target: int) -> List[int]:
    left, right = 0, len(arr) - 1

    while left < right:
        current_sum = arr[left] + arr[right]
        if current_sum == target:
            return [left + 1, right + 1]
        elif current_sum < target:
            left += 1
        else:
            right -= 1

    return [-1, -1]


if __name__ == "__main__":
    arr = [2, 11, 15, 7]
    target = 9
    result = twoSum(arr, target)
    print(result)
