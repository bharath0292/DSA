package advancedpatterns

import "errors"

type Item struct {
	value    any
	priority int
}

type PriorityQueue struct {
	items []Item
}

func (pq *PriorityQueue) NewPriorityQueue() *PriorityQueue {
	return &PriorityQueue{
		items: make([]Item, 0),
	}
}

func (pq *PriorityQueue) IsEmpty() bool {
	return len(pq.items) == 0
}

func (pq *PriorityQueue) bubbleUp(index int) {
	for index > 0 {
		parent := (index - 1) / 2 // Get parent index

		// If the parent has a smaller or equal priority, heap property is satisfied
		if pq.items[parent].priority <= pq.items[index].priority {
			break
		}

		// Swap with parent and move up
		pq.items[parent], pq.items[index] = pq.items[index], pq.items[parent]
		index = parent
	}
}

func (pq *PriorityQueue) bubbleDown(index int) {
	for {
		smallest := index
		left := 2*index + 1
		right := 2*index + 2

		// Check if left child exists and is smaller
		if left < len(pq.items) && pq.items[left].priority < pq.items[smallest].priority {
			smallest = left
		}

		// Check if right child exists and is smaller
		if right < len(pq.items) && pq.items[right].priority < pq.items[smallest].priority {
			smallest = right
		}

		// If the current node is already the smallest, heap is correct
		if smallest == index {
			break
		}

		// Swap with the smaller child and continue
		pq.items[smallest], pq.items[index] = pq.items[index], pq.items[smallest]
		index = smallest
	}
}

func (pq *PriorityQueue) Enqueue(value any, priority int) {
	item := Item{value: value, priority: priority}
	pq.items = append(pq.items, item)

	// Fix the heap going upward
	pq.bubbleUp(len(pq.items) - 1)
}

func (pq *PriorityQueue) Dequeue() (any, error) {
	if pq.IsEmpty() {
		return nil, errors.New("queue is empty")
	}

	// Save the root item to return
	item := pq.items[0]

	// Move the last item to the root
	lastIdx := len(pq.items) - 1
	pq.items[0] = pq.items[lastIdx]
	pq.items = pq.items[:lastIdx] // Remove last item

	// Restore heap order
	if !pq.IsEmpty() {
		pq.bubbleDown(0)
	}

	return item.value, nil
}
