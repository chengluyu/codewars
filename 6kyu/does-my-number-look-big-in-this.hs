module Narcissistic where

narcissistic :: Integral n => n -> Bool
narcissistic n = n == sumUp n where
    m = ceiling $ logBase 10 $ fromIntegral n
    sumUp 0 = 0
    sumUp x = (mod x 10) ^ m + (sumUp $ div x 10)
