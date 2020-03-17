module Codewars.Kata.Braces where

    removeBraces :: String -> String
    removeBraces []           = []
    removeBraces ('(':')':xs) = removeBraces xs
    removeBraces ('[':']':xs) = removeBraces xs
    removeBraces ('{':'}':xs) = removeBraces xs
    removeBraces (x:xs)       = x : removeBraces xs
    
    firstDuplicate :: Eq a => [a] -> Maybe a
    firstDuplicate [] = Nothing
    firstDuplicate [_] = Nothing
    firstDuplicate (x:x':xs) = if x == x' then Just x else firstDuplicate (x':xs)
    
    validBraces :: String -> Bool
    validBraces xs =
        case firstDuplicate $ iterate removeBraces xs of
            Nothing -> False
            Just s  -> null s