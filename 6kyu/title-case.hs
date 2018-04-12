module TitleCase (titleCase) where

    import Data.Char
    
    toLowerCase str = [ toLower loweredString | loweredString <- str]
    
    capitalize [] = []
    capitalize (x:xs) = (toUpper x):xs
    
    titleCase :: String -> String -> String
    titleCase minor title =
        case lowerCaseWords title of
            []     -> ""
            [x]    -> capitalize x
            (x:xs) -> capitalize x ++ " " ++ capitalizeSentence xs
        where lowerCaseWords xs = map toLowerCase $ words xs
              minorWords = lowerCaseWords minor
              capitalizeSentence [] = ""
              capitalizeSentence [x] = if elem x minorWords then x else capitalize x
              capitalizeSentence (x:xs) = (if elem x minorWords then x else capitalize x) ++ " " ++ (capitalizeSentence xs)
    