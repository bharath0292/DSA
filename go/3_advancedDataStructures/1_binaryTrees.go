package advanceddatastructures

type TreeNode struct {
	Value int
	Left  *TreeNode
	Right *TreeNode
}

type BinaryTree struct {
	Root *TreeNode
}

func NewBinaryTree() *BinaryTree {
	return &BinaryTree{nil}
}

// Basic traversals - Time: O(1), Space: O(h) where h is height
func (bt *BinaryTree) InorderTraversal(node *TreeNode) []int {
	var result []int

	if node != nil {
		result = append(result, bt.InorderTraversal(node.Left)...)
		result = append(result, node.Value)
		result = append(result, bt.InorderTraversal(node.Right)...)
	}
	return result
}

// Iterative inorder traversal
func (bt *BinaryTree) InorderIterative() []int {
	var result []int

	stack := make([]*TreeNode, 0)
	current := bt.Root

	for current != nil || len(stack) > 0 {
		// Reach the leftmodst node
		for current != nil {
			stack = append(stack, current)
			current = current.Left
		}

		current = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		result = append(result, current.Value)
		current = current.Right
	}
	return result
}

// Time: O(n), Space: O(w) where w is max width
func (bt *BinaryTree) LevelOrder() [][]int {
	var result [][]int

	if bt.Root == nil {
		return result
	}

	queue := []*TreeNode{bt.Root}
	for len(queue) > 0 {
		levelSize := len(queue)
		currentLevel := make([]int, 0)

		for _ = range levelSize {
			node := queue[0]
			queue = queue[1:]
			currentLevel = append(currentLevel, node.Value)

			if node.Left != nil {
				queue = append(queue, node.Left)
			}

			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		result = append(result, currentLevel)
	}
	return result
}
