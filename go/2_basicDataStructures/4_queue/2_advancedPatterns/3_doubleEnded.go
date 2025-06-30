package advancedpatterns

import "errors"

type Dequeue struct {
	items []any
}

func NewDequeue() *Dequeue {
	return &Dequeue{
		items: make([]any, 0),
	}
}

func (d *Dequeue) AddFront(item any) {
	d.items = append([]any{item}, d.items...)
}

func (d *Dequeue) AddRear(item any) {
	d.items = append(d.items, item)
}

func (d *Dequeue) RemoveFront() (any, error) {
	if len(d.items) == 0 {
		return nil, errors.New("dequeue is empty")
	}

	item := d.items[0]
	d.items = d.items[1:]

	return item, nil
}

func (d *Dequeue) RemoveRear() (any, error) {
	if len(d.items) == 0 {
		return nil, errors.New("dequeue is empty")
	}

	lastIdx := len(d.items) - 1
	item := d.items[lastIdx]
	d.items = d.items[:lastIdx]

	return item, nil
}
