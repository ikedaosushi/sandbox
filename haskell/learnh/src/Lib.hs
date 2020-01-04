module Lib
    where
import Data.Monoid

-- someFunc :: IO ()
-- someFunc = putStrLn "someFunc"

-- addThree :: Int -> Int -> Int -> Int
-- addThree x y z = x + y + z

newtype Product a = Product {getProduct :: a } deriving (Eq, Ord, Show)

-- instance Num a => Monoid (Product a) where
--     mempty = Product 1
--     Product x `mappend` Product y = Product (x * y)