import scala.math.sqrt

object Factorization extends App {
    var target = 24
    val maxDivisor = sqrt(target).toInt
    println(maxDivisor)
}