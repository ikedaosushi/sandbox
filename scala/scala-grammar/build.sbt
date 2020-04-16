import sbt.util

name := "scala-grammar"

version := "1.0"

scalaVersion := "2.12.7"

scalastyleSources in Compile := (unmanagedSourceDirectories in Compile).value

libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % "test"
libraryDependencies += "org.mockito" % "mockito-core" % "2.21.0" % "test"

scalacOptions ++= Seq("-encoding", "UTF-8")

