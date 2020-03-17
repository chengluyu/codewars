# https://www.codewars.com/kata/count-by-x/train/ruby

def count_by(x, n)
  return (1..n).map {|v| v * x}
end
