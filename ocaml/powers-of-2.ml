let rec powersOfTwo n = if n <= 0 then [1] else (powersOfTwo (n - 1))@[1 lsl n]
