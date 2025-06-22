package strings

import (
	"fmt"
	"strings"
)

func StringBasics() {
	// String declaration and initialization
	str1 := "Hello"
	str2 := `Multi-lin
	String literal`
	fmt.Println(str1)
	fmt.Println(str2)

	// String length and accessing characters
	length := len(str1)
	firstChar := str1[0] // byte not rune
	fmt.Println(length)
	fmt.Println(firstChar)

	// Rune handling (for unicode)
	for i, r := range str1 {
		fmt.Printf("Position %d: %c\n", i, r)
	}

	// String conversion
	runes := []rune(str1)
	bytes := []byte(str1)
	fmt.Println(runes)
	fmt.Println(bytes)

	// String concatenation
	result := str1 + " World"
	fmt.Println(result)

	// Using strings.builder for efficient concatenation
	var builder strings.Builder
	builder.WriteString("Hello")
	builder.WriteString(" ")
	builder.WriteString("World")
	result = builder.String()
	fmt.Println(result)

}

func StringOperations() {
	str := "Hello, World!"

	// Substring
	sub := str[0:5]
	fmt.Println(sub)

	// Split
	parts := strings.Split(str, ",")
	fmt.Println(parts)

	// Join
	joined := strings.Join([]string{"Hello", "World"}, " ")
	fmt.Println(joined)

	// Contains, HasPrefix, HasSuffix
	contains := strings.Contains(str, "World")
	hasPrefix := strings.HasPrefix(str, "Hello")
	hasSuffix := strings.HasSuffix(str, "!")
	fmt.Println(contains)
	fmt.Println(hasPrefix)
	fmt.Println(hasSuffix)

	// ToUpper, ToLower
	upper := strings.ToUpper(str)
	lower := strings.ToLower(str)
	fmt.Println(upper)
	fmt.Println(lower)

	// Trim
	trimmed := strings.TrimSpace(" Hello ")
	fmt.Println(trimmed)

	// Replace
	replaced := strings.Replace(str, "World", "Golang", 1)
	fmt.Println(replaced)
}
