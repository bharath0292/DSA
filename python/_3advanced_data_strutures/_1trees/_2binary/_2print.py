from _1tree import BinaryTreeNode


def print_binary_tree(node: BinaryTreeNode, prefix: str = "", is_left: bool = True):
    if node is not None:
        connector = "└── " if is_left else "├── "
        print(prefix + connector + str(node.data))

        # Update prefix for children
        new_prefix = prefix + ("    " if is_left else "│   ")

        # If either left or right child exists, recurse
        if node.left or node.right:
            # If both children exist, print right first (to maintain tree shape)
            if node.left:
                print_binary_tree(node.left, new_prefix, False)
            if node.right:
                print_binary_tree(node.right, new_prefix, True)


if __name__ == "__main__":
    ################ Tree 1 ##########################
    # Basic Tree with height 3
    """
    root1: Contains 6 nodes, Height = 3
    Structure:
          1
        /   \
       2     3
      / \     \
     4   5     6
    
    Preorder = 1 2 4 5 3 6
    Post Order = 4 5 2 6 3 1
    Inorder Traversal = 4 2 5 1 3 6
    """

    root1 = BinaryTreeNode(1)
    root1.left = BinaryTreeNode(2)
    root1.right = BinaryTreeNode(3)
    root1.left.left = BinaryTreeNode(4)
    root1.left.right = BinaryTreeNode(5)
    root1.right.right = BinaryTreeNode(6)

    print("root1")
    print_binary_tree(root1)
    ##################################################

    ################ Tree 2 ##########################
    # Slightly complex tree with height 4
    """
    root2: Contains 8 nodes, Height = 4
    Structure:
        10
        /    \
    20      30
    /  \    /  \
    40   50 60  70
    /
    80
    """
    root2 = BinaryTreeNode(10)
    root2.left = BinaryTreeNode(20)
    root2.right = BinaryTreeNode(30)
    root2.left.left = BinaryTreeNode(40)
    root2.left.right = BinaryTreeNode(50)
    root2.right.left = BinaryTreeNode(60)
    root2.right.right = BinaryTreeNode(70)
    root2.left.left.left = BinaryTreeNode(80)

    print("root2")
    print_binary_tree(root2)
    ##################################################

    ################ Tree 3 ##########################
    # More complex tree with height 5
    """
    root3: Contains 12 nodes, Height = 5
    Structure:
           100
         /     \
       200     300
      /  \     /  \
    400  500  600  700
    / \        /  \
    800 900   1000 1100
    """
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

    print("root3")
    print_binary_tree(root3)
    ##################################################
