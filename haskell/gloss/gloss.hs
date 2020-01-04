import Graphics.Gloss

main :: IO ()
main = display inWindow white message
  where
    inWindow = InWindow "Haskell Day 2018" (640, 480) (100, 100)
    message = text "Hello World"