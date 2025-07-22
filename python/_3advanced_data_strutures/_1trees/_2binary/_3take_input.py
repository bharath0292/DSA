from typing import Optional, Deque
from collections import deque

from _1tree import BinaryTreeNode
from _2print import print_binary_tree


def take_input() -> Optional[BinaryTreeNode]:
    data = int(input("Enter the data for the Node: "))
    if data == -1:
        return

    node = BinaryTreeNode(data=data)

    print(f"Enter the left child of {data}")
    node.left = take_input()

    print(f"Enter the right child of {data}")
    node.right = take_input()

    return node


def take_input_level_wise() -> Optional[BinaryTreeNode]:
    data = int(input("Enter the data for the Node: "))
    if data == -1:
        return

    root = BinaryTreeNode(data=data)

    queue: Deque[BinaryTreeNode] = deque([root])

    while queue:
        current_node = queue.popleft()

        left_child_data = int(
            input(f"Enter the left child data for {current_node.data}: ")
        )
        if left_child_data != -1:
            left_node = BinaryTreeNode(left_child_data)
            current_node.left = left_node
            queue.append(left_node)

        right_child_data = int(
            input(f"Enter the right child data for {current_node.data}: ")
        )
        if right_child_data != -1:
            right_node = BinaryTreeNode(right_child_data)
            current_node.right = right_node
            queue.append(right_node)

    return root


if __name__ == "__main__":
    root = take_input_level_wise()
    print_binary_tree(root)
