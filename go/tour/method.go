package main

import (
	"fmt"
)

type Sushi struct {
	kan  int
	neta string
}

func (s Sushi) Omachi() {
	fmt.Printf("%v、%vカンお待ち！\n", s.neta, s.kan)
}

func (s *Sushi) Double() {
	s.kan = s.kan * 2
}

func main() {
	maguro := Sushi{3, "maguro"}
	maguro.Omachi()
	maguro.Double()
	maguro.Omachi()

}
