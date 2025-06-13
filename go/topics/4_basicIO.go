package topics

import (
	"bufio"
	"fmt"
	"os"
)

func BasicIO() {
	// Basic Output
	fmt.Println("Hello World!")
	fmt.Printf("Formatted: %d, %s\n", 42, "text")

	// Basic Input
	var name string
	fmt.Printf("Enter Name: ")
	fmt.Scan(&name)

	// Formatted input
	var age int
	var height float64
	fmt.Println("Enter age and height: ")
	fmt.Scanf("%d %f", &age, &height)

	// Readinf strings with spaces
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter full name: ")
	fullName, _ := reader.ReadString('\n')
	fmt.Printf("Name: %s", fullName)

	// File writing
	file, err := os.Create("output.txt")
	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	defer file.Close()

	file.WriteString("Hello, File!")

	// File reading
	content, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	fmt.Println(string(content))

	// Buffered file reading
	fileReader, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	defer fileReader.Close()

	scanner := bufio.NewScanner(fileReader)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}
}
