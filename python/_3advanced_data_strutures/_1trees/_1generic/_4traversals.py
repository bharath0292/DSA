from _1node import GenericTreeNode


def pre_order_traversal(root: GenericTreeNode):
    if root is None:
        return

    print(root.data, end=" ")
    for child_node in root.children:
        pre_order_traversal(child_node)


def post_order_traversal(root: GenericTreeNode):
    if root is None:  # Edge case
        return

    for child_node in root.children:
        post_order_traversal(child_node)

    print(root.data, end=" ")


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

    print("------------------root1------------------")
    print("Pre Order Traversal: ")
    pre_order_traversal(root1)
    print("\n")

    print("Post Order Traversal: ")
    post_order_traversal(root1)
    print("\n")

    ##################################################

    ################ Tree 2 ##########################
    """
    root2: Contains 7 nodes, Height = 4
    Structure:
          10
       /   |   \
     20    30   40
           / \
         50   60
          |
         100
    """
    root2 = GenericTreeNode(10)
    child1 = GenericTreeNode(20)
    child2 = GenericTreeNode(30)
    child3 = GenericTreeNode(40)
    root2.children.append(child1)
    root2.children.append(child2)
    root2.children.append(child3)

    child4 = GenericTreeNode(50)
    child4.children.append(GenericTreeNode(100))

    child2.children.append(child4)
    child2.children.append(GenericTreeNode(60))

    print("------------------root2------------------")
    print("Pre Order Traversal: ")
    pre_order_traversal(root2)
    print("\n")

    print("Post Order Traversal: ")
    post_order_traversal(root2)
    print("\n")

    ##################################################

    ################ Tree 3 ##########################
    """
    root3: Contains 5 nodes, Height = 3
    Structure:
          100
         /   \
       200   300
      / \
    400 500
    """
    root3 = GenericTreeNode(100)
    child1 = GenericTreeNode(200)
    root3.children.append(child1)
    root3.children.append(GenericTreeNode(300))

    child1.children.append(GenericTreeNode(400))
    child1.children.append(GenericTreeNode(500))

    print("------------------root3------------------")
    print("Pre Order Traversal: ")
    pre_order_traversal(root3)
    print("\n")

    print("Post Order Traversal: ")
    post_order_traversal(root3)
    print("\n")

    ##################################################
