package advancedpatterns

import (
	stack "dsa/2_basicDataStructures/3_stack"
	"strconv"
	"strings"
)

type Calculator struct {
	stack *stack.Stack
}

func NewCalculator() *Calculator {
	return &Calculator{
		stack: stack.NewStack(),
	}
}

func isOperator(token string) bool {
	return token == "+" || token == "-" || token == "*" || token == "/"
}

func applyOperator(a, b float64, operator string) float64 {
	switch operator {
	case "+":
		return a + b
	case "-":
		return a - b
	case "*":
		return a * b
	case "/":
		return a / b
	default:
		return 0
	}
}

func (c *Calculator) Evaluate(postFix string) (float64, error) {
	for char := range strings.FieldsSeq(postFix) {
		if isOperator(char) {
			b := c.stack.Pop()
			a := c.stack.Pop()
			result := applyOperator(a.(float64), b.(float64), char)
			c.stack.Push(result)
		} else {
			num, _ := strconv.ParseFloat(char, 64)
			c.stack.Push(num)
		}
	}

	result := c.stack.Pop()
	return result.(float64), nil
}
