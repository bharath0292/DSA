package genericTree

func CountNodes[T any](root *GenericTreeNode[T]) int {
	if root == nil {
		return 0
	}

	count := 0

	for _, childNode := range root.Children {
		count += CountNodes(childNode)
	}

	return count

}
