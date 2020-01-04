package main

import "fmt"

func fibonacci() func() int {
	a, b := 1, 0
	return func() (i int) {
		defer func() { a = b; b = i }()
		return a + b
	}
}

func main() {
	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}
}
