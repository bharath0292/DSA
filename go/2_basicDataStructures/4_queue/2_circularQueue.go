package queue

import "errors"

type CircularQueue struct {
	items    []any
	front    int
	rear     int
	size     int
	capacity int
}

func NewCircularQueue(capacity int) *CircularQueue {
	return &CircularQueue{
		items:    make([]any, capacity),
		front:    0,
		rear:     -1,
		size:     0,
		capacity: capacity,
	}
}

func (cq *CircularQueue) IsEmpty() bool {
	return cq.size == 0
}

func (cq *CircularQueue) IsFull() bool {
	return cq.size == cq.capacity
}

func (cq *CircularQueue) Enqueue(item any) error {
	if cq.IsFull() {
		return errors.New("queue is full")
	}

	cq.rear = (cq.rear + 1) % cq.capacity
	cq.items[cq.rear] = item
	cq.size++

	return nil
}

func (cq *CircularQueue) Dequeue() (any, error) {
	if cq.IsEmpty() {
		return nil, errors.New("queue is empty")
	}

	item := cq.items[cq.front]
	cq.front = (cq.front + 1) % cq.capacity
	cq.size--
	return item, nil
}
