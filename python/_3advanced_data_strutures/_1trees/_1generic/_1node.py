from collections import deque
from typing import Any, List


class GenericTreeNode:
    def __init__(self, data: Any):
        self.data: Any = data
        self.children: List[GenericTreeNode] = []

    def add_child(self, child: "GenericTreeNode"):
        self.children.append(child)


def print_tree(root: GenericTreeNode, prefix: str = "", is_last: bool = True):
    connector = "└── " if is_last else "├── "
    print(prefix + connector + str(root.data))

    new_prefix = prefix + ("    " if is_last else "│   ")
    child_count = len(root.children)
    for i, child in enumerate(root.children):
        is_last_child = i == (child_count - 1)
        print_tree(child, new_prefix, is_last_child)


def take_input_level_wise():
    data = int(input("Enter the root node data: "))
    root = GenericTreeNode(data)
    queue = deque([root])

    while queue:
        current_node = queue.popleft()
        num_children = int(input(f"Enter number of children for {current_node.data}: "))
        for i in range(num_children):
            child_data = int(
                input(f"Enter data for child {i + 1} of {current_node.data}: ")
            )
            child_node = GenericTreeNode(child_data)
            current_node.children.append(child_node)
            queue.append(child_node)

    return root


__all__ = ["GenericTreeNode"]
