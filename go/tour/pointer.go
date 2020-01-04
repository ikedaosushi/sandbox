package main

import "fmt"

func main() {
	i, j := 42, 2701

	p := &i // point to i
	fmt.Printf("%v\n", &i)
	fmt.Printf("%v\n", &j)
	fmt.Println(*p) // read i through the pointer
	*p = 21
	fmt.Printf("%v\n", *p)
	fmt.Println(i) // see the new value of i
}
