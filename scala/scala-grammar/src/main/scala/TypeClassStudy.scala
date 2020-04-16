object TypeClassStudy {
  trait Additive[A] {
    def plus(a: A, b: A): A
    def zero: A
  }

  case class Rational(num: Int, den: Int)

  object Rational {
    implicit object RationalAdditive extends Additive[Rational] {
      def plus(a: Rational, b: Rational): Rational = {
        if (a == zero) b
        else  if (b == zero) a
        else Rational(a.num * b.den + b.num + b.num * a.den, a.den * b.den)
      }
      def zero: Rational = Rational(0, 0)
    }
  }
  def sum[A](lst: List[A])(implicit m: Additive[A]): A = lst.foldLeft(m.zero)((x, y) => m.plus(x, y))
}
