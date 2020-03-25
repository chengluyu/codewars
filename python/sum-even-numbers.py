def sum_even_numbers(seq):
    return sum(filter(lambda x: x % 2 == 0, seq))
