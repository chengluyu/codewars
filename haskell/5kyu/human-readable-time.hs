module HumanTime where

    humanReadable :: Int -> String
    humanReadable x = (display h) ++ ":" ++ (display m) ++ ":" ++ (display s) where
        display n = let m = show n in if null $ tail m then '0' : m else m
        s = x `mod` 60
        m = x `div` 60 `mod` 60
        h = x `div` (60 * 60)
