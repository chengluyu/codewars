-- https://www.codewars.com/kata/number-of-trailing-zeros-of-n/train/haskell

module Zeros where

    zeros :: Int -> Int
    zeros n = zeros' 5 n
        where zeros' d n = if d < n then div n d + zeros' (d * 5) n else 0
