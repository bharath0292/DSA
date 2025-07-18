package genericTree

type GenericTreeNode[T any] struct {
	Value    T
	Children []*GenericTreeNode[T]
}

func (b *GenericTreeNode[T]) AddChild(child *GenericTreeNode[T]) {
	b.Children = append(b.Children, child)
}

func BuildTree() GenericTreeNode[string] {
	root := GenericTreeNode[string]{Value: "CEO"}

	headOfSales := GenericTreeNode[string]{Value: "Head of Sales"}
	headOfMarketing := GenericTreeNode[string]{Value: "Head of Marketing"}
	salesExec := GenericTreeNode[string]{Value: "Sales Executive"}
	marketingExec := GenericTreeNode[string]{Value: "Marketing Executive"}

	headOfSales.AddChild(&salesExec)
	headOfMarketing.AddChild(&marketingExec)

	root.AddChild(&headOfSales)
	root.AddChild(&headOfMarketing)

	return root
}
