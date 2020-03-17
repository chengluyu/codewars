module MaxSequence where

    -- Return the greatest subarray sum within the array of integers passed in.
    maxSequence :: [Int] -> Int
    maxSequence [] = 0
    maxSequence xs = maxSequence' 0 0 xs where
        maxSequence' so_far end_here [] = so_far
        maxSequence' so_far end_here (x:xs) =
            let end_here' = max x (end_here + x) in
                maxSequence' (max so_far end_here') end_here' xs