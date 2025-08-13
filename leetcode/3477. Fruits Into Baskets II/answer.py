from typing import List


class Solution:
    def numOfUnplacedFruits(self, fruits: List[int], baskets: List[int]) -> int:
        unplaced = 0

        for fruit in fruits:
            for basket in baskets:
                if fruit > basket:
                    if basket != baskets[-1]:
                        continue
                    unplaced += 1

        return unplaced


if __name__ == "__main__":
    sol = Solution()

    fruits = [4, 2, 5]
    baskets = [3, 5, 4]

    fruits = [3, 6, 1]
    baskets = [6, 4, 7]

    print(sol.numOfUnplacedFruits(fruits, baskets))
