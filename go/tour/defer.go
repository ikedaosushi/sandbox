package main

import "fmt"

func main() {
	for i := 0; i < 10; i++ {
		defer fmt.Printf("Hello %v\n", i)

	}

	fmt.Println("go")
}
