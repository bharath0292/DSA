package genericTree

func HeightOfTree[T any](root *GenericTreeNode[T]) int {
	if root == nil {
		return 0
	}

	height := 1
	maxChildrenHeight := 0

	for _, childNode := range root.Children {
		maxChildrenHeight = max(maxChildrenHeight, HeightOfTree(childNode))
	}

	height += maxChildrenHeight

	return height

}
