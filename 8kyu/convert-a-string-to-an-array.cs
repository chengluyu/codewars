// https://www.codewars.com/kata/convert-a-string-to-an-array/train/csharp

namespace Solution {
    using System;

    public class Solution
    {
        public static string[] StringToArray(string str)
        {
            return str.Split(' ');
        }
    }
}
