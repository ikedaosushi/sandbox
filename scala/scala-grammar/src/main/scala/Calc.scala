class Calc {
  def sum(seq: Seq[Int]): Int = seq.foldLeft(0)(_ + _)

  def div(num: Int, den: Int): Double = {
    if (den == 0) throw new ArithmeticException("den must be more than zero")
    num.toDouble / den.toDouble
  }

  def isPrime(n: Int): Boolean = {
    if (n < 2) false else !((2 to Math.sqrt(n).toInt) exists (n % _ == 0))
  }
}
