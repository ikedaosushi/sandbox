-- module Main where

import Data.Monoid  hiding (Product, Sum, Any)
import Data.Text
import Data.Semigroup hiding (Sum)

-- data Optional a = Nada | Only a deriving (Eq, Show)
data Foo a = Num a deriving (Eq, Show)

instance Monoid a => Monoid (Foo a) where
    mempty = (Num 0)

instance Semigroup a => Semigroup (Foo a) where
  (Num a) <> (Num a') = (Num a)

-- instance Monoid a => Monoid (Optional a) where
--     mempty = Nada

-- instance Semigroup a => Semigroup (Optional a) where
--   Nada <> (Only a) = Only a
--   (Only a) <> Nada = Only a
--   (Only a) <> (Only a') = Only (a <> a')
--   Nada <> Nada = Nada

-- main :: IO ()
-- main = do
--   print "hello"