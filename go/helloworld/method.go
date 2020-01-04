package main

import "fmt"

type User struct {
	name string
	age  int
}

func (u User) greet() string {
	return "hello," + u.name
}

func main() {
	u := User{"taro", 30}
	fmt.Println(u.greet())
}
