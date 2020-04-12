import java.util.function.Consumer
import javafx.animation.{KeyFrame, KeyValue, Timeline}
import javafx.application.Application
import javafx.scene.{Group, Node, Scene}
import javafx.scene.paint.Color
import javafx.scene.shape.{Circle, StrokeType}
import javafx.stage.Stage
import javafx.util.Duration
import java.lang.Math.random

object Main extends App {
  Application.launch(classOf[Main], args: _*)
}

class Main extends Application {

  override def start(primaryStage: Stage): Unit = {
    val root = new Group()
    val scene = new Scene(root, 800, 600, Color.BLACK)
    primaryStage.setScene(scene)

    val circles = new Group()
    for (i <- 1 to 30) {
      val circle = new Circle(150, Color.web("white", 0.05))
      circle.setStrokeType(StrokeType.OUTSIDE)
      circle.setStroke(Color.web("white", 0.16))
      circle.setStrokeWidth(4)
      circles.getChildren().add(circle)
    }
    root.getChildren().add(circles)

    val timeline = new Timeline()
    circles.getChildren().forEach(new Consumer[Node] {

      override def accept(circle: Node): Unit = {
        timeline.getKeyFrames().addAll(
          new KeyFrame(Duration.ZERO,
            new KeyValue(circle.translateXProperty(), random() * 800: Number),
            new KeyValue(circle.translateYProperty(), random() * 600: Number)
          ),
          new KeyFrame(new Duration(40000),
            new KeyValue(circle.translateXProperty(), random() * 800: Number),
            new KeyValue(circle.translateYProperty(), random() * 600: Number)
          )
        )
      }

    })
    timeline.play()
    primaryStage.show()
  }

}