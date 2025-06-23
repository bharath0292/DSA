package stack

import "errors"

type Stack struct {
	items []any
}

func NewStack() *Stack {
	return &Stack{
		items: make([]any, 0),
	}
}

func (s *Stack) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *Stack) Push(item any) {
	s.items = append(s.items, item)
}

func (s *Stack) Pop() (any, error) {
	if s.IsEmpty() {
		return nil, errors.New("stack is empty")
	}
	removeItemIdx := len(s.items) - 1
	item := s.items[removeItemIdx]
	s.items = s.items[:removeItemIdx]

	return item, nil
}

func (s *Stack) Peek() (any, error) {
	if s.IsEmpty() {
		return nil, errors.New("stack is empty")
	}

	return s.items[len(s.items)-1], nil
}

func (s *Stack) Size() int {
	return len(s.items)
}
