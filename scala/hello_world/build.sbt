name := "Hello World"
version := "1.0"
scalaVersion := "2.13.1"

lazy val commonSettings = Seq(
  version := "0.1-SNAPSHOT",
  organization := "com.ikedaosushi",
  scalaVersion := "2.13.1",
  assemblyJarName in assembly := "hello_world.jar",
)
