# LeetCode Problem 3477: Fruits Into Baskets II

## Problem Description

You are given two integer arrays `fruits` and `baskets`, both of length `n`:

- `fruits[i]` represents the quantity of the \(i^{th}\) type of fruit.
- `baskets[j]` represents the capacity of the \(j^{th}\) basket.

You need to assign fruit types to baskets following these rules:

- Each basket can hold **only one type** of fruit.
- For each fruit type, assign it to the **leftmost basket** that has enough capacity to hold all fruits of that type.
- Once a basket is assigned to a fruit type, it cannot be used for any other fruit type.
- If there is no basket available that can hold all fruits of that type, those fruits remain **unplaced**.

Your task is to determine **how many fruit types remain unplaced** after attempting to assign all fruit types to baskets following the above rules.

---

## Examples

### Example 1:

Input:
fruits = [4, 2, 5]
baskets = [3, 5, 4]

Explanation:

- Fruit type 0 has quantity 4: placed in basket 1 (capacity 5).
- Fruit type 1 has quantity 2: placed in basket 0 (capacity 3).
- Fruit type 2 has quantity 5: cannot be placed (basket 2 capacity 4 is too small; baskets 0 and 1 are already used).

Output: 1  
(One fruit type remains unplaced)

### Example 2:

Input:
fruits = [3, 6, 1]
baskets = [6, 4, 7]

Explanation:

- Fruit type 0 has quantity 3: placed in basket 0 (capacity 6).
- Fruit type 1 has quantity 6: placed in basket 2 (capacity 7).
- Fruit type 2 has quantity 1: placed in basket 1 (capacity 4).

Output: 0  
(All fruit types are placed)

## Constraints

- n == fruits.length == baskets.length
- 1 <= n <= 100
- 1 <= fruits[i], baskets[i] <= 1000
