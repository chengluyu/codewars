func getDigits(of number: Int) -> [Int] {
    var numberCopy = number
    var digits: [Int] = []
    repeat {
        digits.append(numberCopy % 10)
        numberCopy /= 10
    } while numberCopy > 0
    return digits.reversed()
}

func pow(_ a: Int, _ b: Int) -> Int {
    var product = 1;
    for _ in 0..<b {
        product *= a
    }
    return product
}

func digPow(for number: Int, using power: Int) -> Int {
    let digits = getDigits(of: number)
    let result = zip(digits, power..<(power + digits.count))
        .reduce(0, { sum, element in
            return sum + pow(element.0, element.1)
        })
    if result % number == 0 {
        return result / number
    }
    return -1
}
