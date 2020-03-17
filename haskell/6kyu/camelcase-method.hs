module CamelCase.JorgeVS.Kata where

    import Data.Char
    
    camelCase :: String -> String
    camelCase xs = foldl (\x y -> x ++ camelCase' y) "" (words xs)
        where camelCase' []     = []
              camelCase' (x:xs) = (toUpper x) : xs