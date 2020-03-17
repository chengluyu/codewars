module Codewars.Kata.BreakCamelCase where
    import Data.Char

    solution :: String -> String
    solution [] = []
    solution (x:xs) = x : solution' xs where
        solution' [] = []
        solution' s@(x:xs) | isUpper x = ' ' : x : solution' xs
                           | otherwise = x : solution' xs
    