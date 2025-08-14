from typing import List


def removeElement(arr: List[int], ele: int) -> int:
    k = 0
    for i in arr:
        if i != ele:
            k += 1
    return k


if __name__ == "__main__":
    arr = [0, 1, 3, 0, 2, 2, 2]
    ele = 2
    print(removeElement(arr, ele))
