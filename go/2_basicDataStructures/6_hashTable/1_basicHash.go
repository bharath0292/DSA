package hashtable

type HashNode struct {
	key   any
	value any
	next  *HashNode
}

type HashTable struct {
	buckets  []*HashNode
	size     int
	capacity int
}

func NewHashTable(capacity int) *HashTable {
	return &HashTable{
		buckets:  make([]*HashNode, capacity),
		size:     0,
		capacity: capacity,
	}
}

func (ht *HashTable) hash(key any) int {
	switch v := key.(type) {
	case string:
		hash := 0
		for i := range v {
			hash = ((hash * 31) + int(v[i])) % ht.capacity
		}
		return hash
	case int:
		return v % ht.capacity
	default:
		return 0
	}
}

// Time: O(n) where n is capacity
func (ht *HashTable) resize() {
	oldBuckets := ht.buckets
	ht.capacity *= 2
	ht.buckets = make([]*HashNode, ht.capacity)
	ht.size = 0

	for _, node := range oldBuckets {
		for node != nil {
			ht.Put(node.key, node.value)
			node = node.next
		}
	}
}

// Time: O(1) average, O(n) worst case
func (ht *HashTable) Put(key, value any) {
	index := ht.hash(key)
	node := ht.buckets[index]

	for node != nil {
		if node.key == key {
			node.value = value
			return
		}
		node = node.next
	}

	newNode := &HashNode{key: key, value: value}
	newNode.next = ht.buckets[index]
	ht.buckets[index] = newNode
	ht.size++

	if float64(ht.size)/float64(ht.capacity) > 0.75 {
		ht.resize()
	}
}

// Time: O(1) average, O(n) worst case
func (ht *HashTable) Get(key any) (any, bool) {
	index := ht.hash(key)
	node := ht.buckets[index]

	for node != nil {
		if node.key == key {
			return node.value, true
		}
		node = node.next
	}

	return nil, false
}
