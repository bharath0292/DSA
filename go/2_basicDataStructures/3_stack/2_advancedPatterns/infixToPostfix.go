package advancedpatterns

import (
	stack "dsa/2_basicDataStructures/3_stack"
	"strings"
	"unicode"
)

func precedence(op rune) int32 {
	switch op {
	case '+', '-':
		return 1
	case '*', '/':
		return 2
	default:
		return 0
	}
}

func popUntilOpen(stack stack.Stack, postFix strings.Builder) {
	for !stack.IsEmpty() {
		top := stack.Pop()
		if top == nil || top == '(' {
			break
		}
		postFix.WriteRune(top.(rune))
	}
}

func popWithHigherPrec(stack stack.Stack, currentOp rune, postFix strings.Builder) {
	for !stack.IsEmpty() {
		top := stack.Peek()
		if top == nil || precedence(top.(rune)) < precedence(currentOp) {
			break
		}
		postFix.WriteRune(stack.Pop().(rune))
	}
}

func popAll(stack stack.Stack, postFix strings.Builder) {
	for !stack.IsEmpty() {
		top := stack.Pop()
		if top != nil {
			postFix.WriteRune(top.(rune))
		}
	}
}

func InfixToPostfix(expr string) string {
	var postFix strings.Builder
	var stack stack.Stack

	for _, c := range expr {
		switch {
		case unicode.IsLetter(c) || unicode.IsDigit(c):
			postFix.WriteRune(c)
		case c == '(':
			stack.Push(c)
		case c == ')':
			popUntilOpen(stack, postFix)
		case c == '+', c == '-', c == '*', c == '/':
			popWithHigherPrec(stack, c, postFix)
			stack.Push(c)
		}
	}

	popAll(stack, postFix)

	return postFix.String()
}
