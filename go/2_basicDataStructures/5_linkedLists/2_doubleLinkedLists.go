package linkedlists

type DoublyNode struct {
	Value any
	Next  *DoublyNode
	Prev  *DoublyNode
}

type DoublyLinkedList struct {
	head *DoublyNode
	tail *DoublyNode
	size int
}

func NewDoublyLinkedList() *DoublyLinkedList {
	return &DoublyLinkedList{nil, nil, 0}
}

// Time: O(1), Space: O(1)
func (list *DoublyLinkedList) InsertFront(value any) {
	newNode := &DoublyNode{
		Value: value,
	}

	if list.size == 0 {
		list.head = newNode
		list.tail = newNode
	} else {
		newNode.Next = list.head
		list.head.Prev = newNode
		list.head = newNode
	}

	list.size++

}

// Time: O(1), Space: O(1)
func (list *DoublyLinkedList) InsertEnd(value any) {
	newNode := &DoublyNode{
		Value: value,
	}

	if list.size == 0 {
		list.head = newNode
		list.tail = newNode
	} else {
		newNode.Prev = list.tail
		list.head.Next = newNode
		list.tail = newNode
	}
}

// Time: O(n), Space: O(1)
func (list *DoublyLinkedList) Delete(value any) bool {
	current := list.head

	for current != nil {
		if current.Value == value {
			if current.Prev != nil {
				current.Prev.Next = current.Next
			} else {
				current.Prev = current.Next
			}

			if current.Next != nil {
				current.Next.Prev = current.Prev
			} else {
				list.tail = current.Prev
			}

			return true
		}

		current = current.Next
	}

	return false
}
