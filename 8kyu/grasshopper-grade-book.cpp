// https://www.codewars.com/kata/grasshopper-grade-book/train/cpp

char getGrade(int a, int b, int c) {
    auto s = (a + b + c) / 3;
    if (90 <= s && s <= 100) {
        return 'A';
    } else if (80 <= s && s < 90) {
        return 'B';
    } else if (70 <= s && s < 80) {
        return 'C';
    } else if (60 <= s && s < 70) {
        return 'D';
    } else if (0 <= s && s < 60) {
        return 'F';
    }
    return '?';
}
