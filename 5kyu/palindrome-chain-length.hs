module PalindromeChain where

    palindromeChainLength :: Integer -> Integer
    palindromeChainLength = toInteger . length . takeWhile (not . isPalindrome) . (iterate addWithReverse) where
        isPalindrome n = (show n) == (reverse $ show n)
        addWithReverse n = n + (read (reverse $ show n) :: Integer)
