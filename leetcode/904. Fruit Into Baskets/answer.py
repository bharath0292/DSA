from typing import List
from collections import defaultdict


class Solution:
    def totalFruit(self, fruits: List[int]) -> int:
        if not fruits:
            return 0
        if len(fruits) == 1:
            return 1

        left = 0
        basket = defaultdict(int)
        max_fruits = 0

        for right in range(len(fruits)):
            basket[fruits[right]] += 1

            while len(basket) > 2:
                basket[fruits[left]] -= 1
                if basket[fruits[left]] == 0:
                    del basket[fruits[left]]
                left += 1

            max_fruits = max(max_fruits, right - left + 1)

        return max_fruits


if __name__ == "__main__":
    sol = Solution()
    print(sol.totalFruit([1, 1, 2, 3, 3, 1]))
