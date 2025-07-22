from typing import Optional

from _1tree import BinaryTreeNode


def height(root: Optional[BinaryTreeNode]) -> int:
    if root is None:
        return 0

    left_height = height(root.left)
    right_height = height(root.right)

    if left_height == -1 or right_height == -1 or abs(left_height - right_height) > 1:
        return -1

    return 1 + max(left_height, right_height)


def is_balanced(root) -> bool:
    return height(root) > 0


if __name__ == "__main__":
    ################ Tree 1 ##########################
    # Basic Tree with height 3
    # root1: Contains 6 nodes, Height = 3
    # Structure:
    #       1
    #     /   \
    #    2     3
    #   / \     \
    #  4   5     6

    # Preorder = 1 2 4 5 3 6
    # Post Order = 4 5 2 6 3 1
    # Inorder Traversal = 4 2 5 1 3 6

    root1 = BinaryTreeNode(1)
    root1.left = BinaryTreeNode(2)
    root1.right = BinaryTreeNode(3)
    root1.left.left = BinaryTreeNode(4)
    root1.left.right = BinaryTreeNode(5)
    root1.right.right = BinaryTreeNode(6)

    print(f"root1: {is_balanced(root1)}")
    ##################################################

    ################ Tree 2 ##########################
    # Slightly complex tree with height 4
    # root2: Contains 8 nodes, Height = 4
    # Structure:
    #     10
    #     /    \
    # 20      30
    # /  \    /  \
    # 40   50 60  70
    # /
    # 80

    root2 = BinaryTreeNode(10)
    root2.left = BinaryTreeNode(20)
    root2.right = BinaryTreeNode(30)
    root2.left.left = BinaryTreeNode(40)
    root2.left.right = BinaryTreeNode(50)
    root2.right.left = BinaryTreeNode(60)
    root2.right.right = BinaryTreeNode(70)
    root2.left.left.left = BinaryTreeNode(80)

    print(f"root2: {is_balanced(root2)}")
    ##################################################

    ################ Tree 3 ##########################
    # More complex tree with height 5
    # root3: Contains 12 nodes, Height = 5
    # Structure:
    #        100
    #      /     \
    #    200     300
    #   /  \     /  \
    # 400  500  600  700
    # / \        /  \
    # 800 900   1000 1100

    root3 = BinaryTreeNode(100)
    root3.left = BinaryTreeNode(200)
    root3.right = BinaryTreeNode(300)
    root3.left.left = BinaryTreeNode(400)
    root3.left.right = BinaryTreeNode(500)
    root3.right.left = BinaryTreeNode(600)
    root3.right.right = BinaryTreeNode(700)
    root3.left.left.left = BinaryTreeNode(800)
    root3.left.left.right = BinaryTreeNode(900)
    root3.right.right.left = BinaryTreeNode(1000)
    root3.right.right.right = BinaryTreeNode(1100)

    print(f"root3: {is_balanced(root3)}")
    ##################################################

    ################ Tree 4 ##########################
    #            1
    #           / \
    #          2   3
    #         /  \
    #        4   5
    #       /
    #      8

    root3 = BinaryTreeNode(1)
    root3.left = BinaryTreeNode(2)
    root3.right = BinaryTreeNode(3)
    root3.left.left = BinaryTreeNode(4)
    root3.left.right = BinaryTreeNode(5)
    root3.left.left.left = BinaryTreeNode(8)
    print(f"root3: {is_balanced(root3)}")
    ##################################################
