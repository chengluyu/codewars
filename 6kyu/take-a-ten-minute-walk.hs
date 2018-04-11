module Codewars.Kata.TenMinuteWalk where

oneStep :: Char -> (Int, Int) -> (Int, Int)
oneStep 'n' (x, y) = (x, y + 1)
oneStep 's' (x, y) = (x, y - 1)
oneStep 'w' (x, y) = (x - 1, y)
oneStep 'e' (x, y) = (x + 1, y)

tryWalk :: [Char] -> (Int, Int) -> Int -> (Int, Int)
tryWalk []     pos n = if n == 10 then pos else (2, 2)
tryWalk (x:xs) pos n = if n < 10 then (tryWalk xs (oneStep x pos) (n + 1)) else (1, 1)

isValidWalk :: [Char] -> Bool
isValidWalk walk =
    let (x, y) = tryWalk walk (0, 0) 0 in x == 0 && y == 0