trait StreamStudy[+A] {
  def headOption: Option[A] = this match {
    case EmptyStream => None
    case Cons(h, _) => Some(h())
  }
  def tail: StreamStudy[A] = this match {
    case EmptyStream => throw  new NoSuchMethodError()
    case Cons(_, t) => t()
  }
}

case object  EmptyStream extends StreamStudy[Nothing]

case class Cons[+A](h: () => A, t: () => StreamStudy[A]) extends StreamStudy[A]

object StreamStudy {
  def cons[A](h: => A, t: => StreamStudy[A]): StreamStudy[A] = {
    lazy val headResult = h
    lazy val tailResult = t
    Cons(() => headResult, () => tailResult)
  }
  def empty[A]: StreamStudy[A] = EmptyStream
}
