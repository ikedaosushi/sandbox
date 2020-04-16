import org.scalatest._
import org.scalatest.concurrent.TimeLimits._
import org.scalatest.time.SpanSugar._
import org.scalatest.mockito.MockitoSugar
import org.mockito.Mockito._

class CalcSpec extends FlatSpec with DiagrammedAssertions with MockitoSugar {
  val calc = new Calc

  "sum" should "returns sum of integers in array" in {
    assert(calc.sum(Seq(1, 2, 3)) === 6)
    assert(calc.sum(Seq(0)) === 0)
    assert(calc.sum(Seq(-1, 1)) === 0)
    assert(calc.sum(Seq()) === 0)
  }

  it should "returns MIN_VALUE if the value is over MAX_VALUE of integer" in {
    assert(calc.sum(Seq(Integer.MAX_VALUE, 1)) === Integer.MIN_VALUE)
  }

  "div" should "returns divided value with given two numbers" in {
    assert(calc.div(6, 3) === 2.0)
    assert(calc.div(1, 3) === 0.3333333333333333333333)
  }

  it should "raise Exception if divide with 0" in {
    intercept[ArithmeticException] {
      calc.div(1, 0)
    }
  }

  "isPrime" should "return if this value is prime or not" in {
    assert(!calc.isPrime(0))
    assert(!calc.isPrime(-1))
    assert(calc.isPrime(2))
    assert(calc.isPrime(17))
  }

  it should "can calc within 1 sec with big value" in {
    failAfter(1000 millis) {
      assert(calc.isPrime(9999991))
    }
  }

  "mock of calc object" should "can disguise" in {
    val mockCalc = mock[Calc]
    when(mockCalc.sum(Seq(3, 4, 5))).thenReturn(12)
    assert(mockCalc.sum(Seq(3, 4, 5)) === 12)
  }

}
