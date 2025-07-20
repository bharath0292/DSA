from collections import deque
from _1node import GenericTreeNode
from typing import Deque, List


def count_nodes(root: GenericTreeNode) -> int:
    """
    Program to count number of nodes
    which has more children than its parent
    """
    if root is None:
        return 0

    count = 0

    queue: Deque[GenericTreeNode] = deque([root])

    while queue:
        current_node = queue.popleft()
        for child_node in current_node.children:
            if len(child_node.children) > len(current_node.children):
                count += 1
            queue.append(child_node)

    return count


def sum_nodes(root: GenericTreeNode) -> int:
    """
    Given the root of an N-ary tree, return the sum of all the node values in the tree.
    """
    if root is None:
        return 0

    value = root.data
    for child_node in root.children:
        value += sum_nodes(child_node)

    return value


def largest_values(root: GenericTreeNode) -> List[int]:
    if not root:
        return []

    result = []
    queue = deque([root])

    while queue:
        level_size = len(queue)
        max_val = float("-inf")

        for _ in range(level_size):
            node = queue.popleft()
            max_val = max(max_val, node.data)

            for child in node.children:
                queue.append(child)

        result.append(max_val)

    return result


if __name__ == "__main__":
    ################ Tree 1 ##########################
    """
    Properties (Examples)
    root1: Contains 5 nodes, Height = 3
    Structure: 
          1
        /   \
       2     3
      / \
     4   5
    """
    root1 = GenericTreeNode(1)
    child1 = GenericTreeNode(2)
    child2 = GenericTreeNode(3)
    root1.children.append(child1)
    root1.children.append(child2)

    child1.children.append(GenericTreeNode(4))
    child1.children.append(GenericTreeNode(5))

    child2.children.append(GenericTreeNode(6))

    print(f"count_largest root1: {count_nodes(root1)}")
    print(f"sum_all_nodes root1: {sum_nodes(root1)}")
    print(f"largest_values root1: {largest_values(root1)}")
    ##################################################
