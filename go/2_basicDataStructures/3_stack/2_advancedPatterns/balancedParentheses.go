package advancedpatterns

import stack "dsa/2_basicDataStructures/3_stack"

func IsBalanced(expr string) bool {
	stack := stack.NewStack()

	for _, char := range expr {
		switch char {
		case '(', '{', '[':
			stack.Push(char)

		case ')', '}', ']':
			if stack.IsEmpty() {
				return false
			}

			top, _ := stack.Pop()
			if !isMatching(top.(rune), char) {
				return false
			}
		}
	}

	return stack.IsEmpty()
}

func isMatching(open, close rune) bool {
	return (open == '(' && close == ')') ||
		(open == '{' && close == '}') ||
		(open == '[' && close == ']')
}
