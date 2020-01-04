package main

import "fmt"

type Sushi struct {
	n    int
	neta string
}

func main() {
	maguro := Sushi{3, "maguro"}
	ika := Sushi{2, "ika"}
	fmt.Println("neta", maguro.neta)

	// p := &ika
	ika.n = 4
	fmt.Println(ika.n, "kan")

	ebi := Sushi{neta: "ebi", n: 1}
	fmt.Println(ebi)

	sushis := make(map[string]Sushi)

	sushis["e"] = ebi
	sushis["i"] = ika

	fmt.Printf("%v", sushis)

}
