def fizzBuzz(n: Int) {
    if (n % 15 == 0) {
        println("FizzBuzz")
    } else if (n % 3 == 0) {
        println("Fizz")
    } else if (n % 5 == 0) {
        println(n)
    }
}
(1 to 100).foreach(fizzBuzz)