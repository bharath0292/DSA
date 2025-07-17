package trees

type BasicTreeNode[T any] struct {
	Value    T
	Children []BasicTreeNode[T]
}

func (b *BasicTreeNode[T]) AddChild(child BasicTreeNode[T]) {
	b.Children = append(b.Children, child)
}

func BuildTree() BasicTreeNode[string] {
	root := BasicTreeNode[string]{Value: "CEO"}

	headOfSales := BasicTreeNode[string]{Value: "Head of Sales"}
	headOfMarketing := BasicTreeNode[string]{Value: "Head of Marketing"}
	salesExec := BasicTreeNode[string]{Value: "Sales Executive"}
	marketingExec := BasicTreeNode[string]{Value: "Marketing Executive"}

	headOfSales.AddChild(salesExec)
	headOfMarketing.AddChild(marketingExec)

	root.AddChild(headOfSales)
	root.AddChild(headOfMarketing)

	return root
}
