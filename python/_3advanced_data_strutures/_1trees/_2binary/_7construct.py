from _1tree import BinaryTreeNode


def construct_tree_from_inorder_and_preorder(inorder, preorder, inS, inE, prS, prE):
    if inS > inE or prS > prE:  # Base Condition
        return None

    root_data = preorder[prS]  # preorder[0] # To avoid using 0
    root = BinaryTreeNode(root_data)

    rootIndexInInorder = -1
    for i in range(inS, inE + 1):
        if inorder[i] == root_data:
            rootIndexInInorder = i
            break

    if rootIndexInInorder == -1:
        print("Root not found in Inorder,please check logic")
        return None

    linS = inS
    linE = rootIndexInInorder - 1
    lprS = prS + 1
    lprE = lprS + (linE - linS)

    rinE = inE
    rprS = lprE + 1
    rprE = prE
    # rprE - rprS = rinE - rinS
    rinS = rootIndexInInorder + 1

    root.left = construct_tree_from_inorder_and_preorder(
        inorder, preorder, linS, linE, lprS, lprE
    )
    root.right = construct_tree_from_inorder_and_preorder(
        inorder, preorder, rinS, rinE, rprS, rprE
    )

    return root
