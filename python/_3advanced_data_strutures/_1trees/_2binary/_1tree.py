from typing import Any, Optional


class BinaryTreeNode:
    def __init__(self, data: Any):
        self.data: Any = data
        self.left: Optional[BinaryTreeNode] = None
        self.right: Optional[BinaryTreeNode] = None
