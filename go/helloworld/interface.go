package main

import (
	"fmt"
	"strconv"
)

type Car interface {
	run(int) string
	stop()
}

//構造体の定義
type MyCar struct {
	name  string
	speed int
}

//メソッドの実装
func (u *MyCar) run(speed int) string {
	u.speed = speed
	return strconv.Itoa(speed) + "kmで走ります"
}

func (u *MyCar) stop() {
	fmt.Println("停止します")
	u.speed = 0
}

func main() {
	//ポインタを返すこと
	myCar := &MyCar{name: "マイカー", speed: 0}

	var i Car = myCar
	fmt.Println(i.run(50))
	i.stop()
}
