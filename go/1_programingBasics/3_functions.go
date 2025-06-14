package topics

import "errors"

// Basic function
func Add(x, y int) int {
	return x + y
}

// Multiple return values
func Divide(x, y float64) (float64, error) {
	if y == 0 {
		return 0, errors.New("division by zero")
	}

	return x / y, nil
}

// Named return function
func Rectangle(width, height float64) (area, perimeter float64) {
	area = width * height
	perimeter = 2 * (width + height)

	return // Naked return
}

// Variadic function
func Sum(numbers ...int) int {
	total := 0

	for _, num := range numbers {
		total += num
	}

	return total
}

// Function as parameter
func Calculate(fn func(int, int) int, x, y int) int {
	return fn(x, y)
}

// Closure
func Counter() func() int {
	count := 0

	return func() int {
		count++
		return count
	}
}

// Method (function with reciever)
type RectangleType struct {
	Width  float64
	Height float64
}

func (r RectangleType) Area() float64 {
	return r.Width * r.Height
}

// Pointer reciever
func (r *RectangleType) Scale(factor float64) {
	r.Width *= factor
	r.Height *= factor
}
