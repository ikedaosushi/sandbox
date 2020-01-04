package main

import "fmt"

// ポニーは歩ける
type Pony interface {
	Walk()
}

// アースポニーも歩けるので、Ponyインタフェースに渡せる
type EarthPony struct {
}

func (ep *EarthPony) Walk() {
	fmt.Println("I can walk")
}

// ペガサスはアースポニーの上位互換
type Pegasus struct {
	EarthPony
}

func (u *Pegasus) Fly() {
	fmt.Println("I can fly")
}

func (u *Pegasus) Walk() {
	u.Fly() // 歩けと言われても、ペガサスは飛ぶよ！
}

func main() {
	var p = Pegasus{}
	p.Walk()
}
