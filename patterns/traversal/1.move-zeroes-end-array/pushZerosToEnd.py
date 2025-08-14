from typing import List


def pushZerosToEnd(arr: List[int]) -> List[int]:
    count = 0
    for i in range(len(arr)):
        if arr[i] != 0:
            arr[i], arr[count] = arr[count], arr[i]
            count += 1

    return arr


if __name__ == "__main__":
    example1 = [1, 2, 0, 4, 3, 0, 5, 0]
    print("Example 1: ", pushZerosToEnd(example1))

    example2 = [10, 20, 30]
    print("Example 2: ", pushZerosToEnd(example2))

    example3 = [0, 0]
    print("Example 3: ", pushZerosToEnd(example3))
