from typing import Any, List


class TreeNode:
    def __init__(self, data: Any):
        self.data = data
        self.children: List["TreeNode"] = []

    def add_child(self, child: "TreeNode"):
        self.children.append(child)


# To print
def print_tree(root: TreeNode, level: int = 0):
    print("    " * level + f"- {root.data}")
    for child in root.children:
        print_tree(child, level + 1)


# Build tree
root = TreeNode("CEO")

head_of_sales = TreeNode("Head of Sales")
head_of_marketing = TreeNode("Head of Marketing")
sales_executive = TreeNode("Sales Executive")
marketing_executive = TreeNode("Marketing Executive")

root.add_child(head_of_sales)
root.add_child(head_of_marketing)

head_of_sales.add_child(sales_executive)
head_of_marketing.add_child(marketing_executive)

# Print tree
print_tree(root)
