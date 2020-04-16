object Match {
  def message(message: String): String = {
    message match {
      case "good" | "bad" => "game"
    }
  }

  def patternMatch(): Unit = {
    val seq = Seq("A", "B", "C", "D", "E")
    seq match {
      case Seq("A", b, c, d, e) =>
        println("b = " + b)
        println("c = " + c)
        println("d = " + d)
        println("e = " + e)
      case _ =>
        println("nothing")
    }
  }
}
