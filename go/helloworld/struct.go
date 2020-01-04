package main

import "fmt"

//User構造体定義。大文字から始まっているので他パッケージからアクセス可能
type User struct {
	name string
	age  int
	// sex  string
}

func main() {
	//User構造体にアクセス
	var u User
	u.name = "taro"
	u.age = 30
	// u.sex = "male"
	// fmt.Printf("name:%s:%s", u.name, u.sex)
	fmt.Printf("name:%s\n", u.name)

	var u1 User
	u1.name = "jiro"
	u1.age = 26
	fmt.Printf("name:%s\n", u1.name)

	//順番で初期化
	u2 := User{"saburo", 23}
	fmt.Printf("name:%s\n", u2.name)

	//Key:Valueで初期化。
	u3 := User{name: "shiro", age: 22}
	fmt.Printf("name:%s\n", u3.name)
}
