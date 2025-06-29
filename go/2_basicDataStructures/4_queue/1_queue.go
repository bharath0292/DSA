package queue

import "errors"

type Queue struct {
	items []any
}

func NewQueue() *Queue {
	return &Queue{
		items: make([]any, 0),
	}
}

func (q *Queue) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue) Size() int {
	return len(q.items)
}

func (q *Queue) Enqueue(item any) {
	q.items = append(q.items, item)
}

func (q *Queue) Dequeue() (any, error) {
	if q.IsEmpty() {
		return nil, errors.New("queue is empty")
	}

	item := q.items[0]
	q.items = q.items[1:]

	return item, nil
}

func (q *Queue) Front() (any, error) {
	if q.IsEmpty() {
		return nil, errors.New("queue is empty")
	}

	return q.items[0], nil
}
