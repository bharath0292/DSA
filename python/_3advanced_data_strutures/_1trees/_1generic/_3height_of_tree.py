from _1node import GenericTreeNode


def height_of_tree(root: GenericTreeNode) -> int:
    if root is None:
        return 0

    height = 1
    max_child_height = 0

    for child in root.children:
        max_child_height = max(max_child_height, height_of_tree(child))

    height += max_child_height

    return height


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

    print(f"root1: {height_of_tree(root1)}")
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

    print(f"root2: {height_of_tree(root2)}")
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

    print(f"root3: {height_of_tree(root3)}")
    ##################################################
