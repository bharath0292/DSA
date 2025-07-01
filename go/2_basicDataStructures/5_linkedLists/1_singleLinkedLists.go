package linkedlists

type Node struct {
	Value any
	Next  *Node
}

type SinglyLinkedList struct {
	head *Node
	size int
}

func NewSinglyLinkedList() *SinglyLinkedList {
	return &SinglyLinkedList{nil, 0}
}

// Time: O(1), Space: O(1)
func (list *SinglyLinkedList) InsertFront(value any) {
	newNode := &Node{Value: value}
	newNode.Next = list.head
	list.head = newNode
	list.size++
}

// Time: O(n), Space: O(1)
func (list *SinglyLinkedList) InsertEnd(value any) {
	newNode := &Node{Value: value}
	if list.head == nil {
		list.head = newNode
		list.size++
		return
	}

	current := list.head
	for current.Next != nil {
		current = current.Next
	}

	current.Next = newNode
	list.size++
}

// Time: O(n), Space: O(1)
func (list *SinglyLinkedList) Delete(value any) bool {
	if list.head == nil {
		return false
	}

	if list.head.Value == value {
		list.head = list.head.Next
		list.size--
		return true
	}

	current := list.head
	for current.Next != nil {
		if current.Next.Value == value {
			current.Next = current.Next.Next
			list.size--
			return true
		}
		current = current.Next
	}

	return false
}

// Time: O(n), Space: O(1)
func (list *SinglyLinkedList) Reverse() {
	var prev *Node
	current := list.head

	for current != nil {
		nextTemp := current.Next
		current.Next = prev
		prev = current
		current = nextTemp
	}

	list.head = prev
}
