module IPv4 where
    type IPString = String
    
    int32ToIP :: (Show a, Integral a) => a -> IPString
    int32ToIP = foldl1 (\x y -> x ++ "." ++ y) . map (show . (`mod` 256)) . reverse . take 4 . iterate (`div` 256)