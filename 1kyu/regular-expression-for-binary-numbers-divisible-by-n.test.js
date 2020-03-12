const {
  regexDivisibleBy
} = require("./regular-expression-for-binary-numbers-divisible-by-n");

describe("regexDivisibleBy", () => {
  it("n = 2", () => {
    const regex = new RegExp(regexDivisibleBy(2));
    for (let i = 1; i < 100; i++) {
      expect(i % 2 === 0).toBe(regex.test(i.toString()));
    }
  });
});
