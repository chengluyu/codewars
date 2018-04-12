module Codewars.Kata.FindOdd where

    -- | Given a list, find the [Int] that appears an 
    --   odd number of times. The tests will always
    --   provide such a number, and the list will
    --   always contain at least one element.
    
    unique xs = unique' xs [] where
        unique' []     ys = ys
        unique' (x:xs) ys = unique' xs (if elem x ys then ys else x:ys)
    
    occurOddTimes x [] = False
    occurOddTimes x (y:ys) = (if x == y then not else id) (occurOddTimes x ys)
    
    firstOf :: (a -> Bool) -> [a] -> Maybe a
    firstOf _ [] = Nothing
    firstOf f (y:ys) = if f y then Just y else firstOf f ys
    
    findOdd :: [Int] -> Int
    findOdd xs =
        case firstOf (`occurOddTimes` xs) (unique xs) of
            Just x -> x
            Nothing -> undefined