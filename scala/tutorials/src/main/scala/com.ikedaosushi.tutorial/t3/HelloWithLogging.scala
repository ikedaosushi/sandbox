package com.ikedaosushi.tutorial.t3

import com.typesafe.scalalogging.LazyLogging

object HelloWithLogging extends App with LazyLogging {
    logger.info("Hello World with logging")
}