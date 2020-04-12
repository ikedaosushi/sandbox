object ControlSyntax {
  def doubleLoop(): Unit = {
    for (x <- 1 to 5; y <- 1 until 5) {
      println("x = " + x + " y = " + y)
    }
  }
  def collectionLoop(): Unit =
    for(e <- Seq("A", "B", "C", "D", "E")) println(e)

  def printIsBaby(age: Int, isSchoolStarted: Boolean): Unit = {
    if(1 <= age && age <= 6) println("yes") else println("no")
  }

  def loopFrom0To9(): Unit = {
    var i = 0
    do {
      println(i)
      i += 1
    } while (i < 10)
  }
}
