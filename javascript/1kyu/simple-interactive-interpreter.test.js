const { Interpreter, Parser } = require("./simple-interactive-interpreter");

describe("Parser", () => {
  const interpreter = new Interpreter();
  const parser = new Parser(interpreter);
  const parse = source => parser.parse(interpreter.tokenize(source));

  const avg = {
    type: "fn",
    name: "avg",
    params: ["x", "y"],
    body: {
      type: "binary",
      op: "/",
      left: {
        type: "binary",
        op: "+",
        left: { type: "lookup", name: "x" },
        right: { type: "lookup", name: "y" }
      },
      right: {
        type: "number",
        value: 2
      }
    }
  };

  it("parse binary", () => {
    expect(parse("1 + 1")).toEqual({
      type: "binary",
      op: "+",
      left: { type: "number", value: 1 },
      right: { type: "number", value: 1 }
    });
    expect(parse("2 - 1")).toEqual({
      type: "binary",
      op: "-",
      left: { type: "number", value: 2 },
      right: { type: "number", value: 1 }
    });
    expect(parse("2 * 3")).toEqual({
      type: "binary",
      op: "*",
      left: { type: "number", value: 2 },
      right: { type: "number", value: 3 }
    });
    expect(parse("8 / 4")).toEqual({
      type: "binary",
      op: "/",
      left: { type: "number", value: 8 },
      right: { type: "number", value: 4 }
    });
    expect(parse("7 % 4")).toEqual({
      type: "binary",
      op: "%",
      left: { type: "number", value: 7 },
      right: { type: "number", value: 4 }
    });
  });

  it("parse assignment", () => {
    expect(parse("x = 1")).toEqual({
      type: "assignment",
      name: "x",
      value: { type: "number", value: 1 }
    });

    expect(parse("x = 1 + 1")).toEqual({
      type: "assignment",
      name: "x",
      value: {
        type: "binary",
        op: "+",
        left: { type: "number", value: 1 },
        right: { type: "number", value: 1 }
      }
    });
  });

  it("with variables", () => {
    expect(parse("x")).toEqual({ type: "lookup", name: "x" });
    expect(parse("x + 3")).toEqual({
      type: "binary",
      op: "+",
      left: { type: "lookup", name: "x" },
      right: { type: "number", value: 3 }
    });
  });

  it("functions", () => {
    expect(parse("fn avg x y => (x + y) / 2")).toEqual(avg);
    interpreter.functions.set("avg", avg);
    expect(parse("avg 4 2")).toEqual({
      type: "call",
      callee: "avg",
      args: [
        { type: "number", value: 4 },
        { type: "number", value: 2 }
      ]
    });
    expect(() => parse("avg 7")).toThrow();
    expect(() => parse("avg 7")).toThrow();
    expect(() => parse("avg 7 2 4")).toThrow();
    interpreter.functions.delete("avg");
  });

  it("corner cases", () => {
    interpreter.vars.set("x", 0);
    interpreter.functions.set("avg", avg);
    expect(() => parse("fn x => 0")).toThrow();
    expect(() => parse("")).toThrow();
    expect(() => parse("avg = 5")).toThrow();
    expect(() => parse("fn avg => 0")).not.toThrow();
    interpreter.vars.delete("x");
    interpreter.functions.delete("avg");
  });
});

describe("Interpreter", () => {
  const interpreter = new Interpreter();

  it("default test", () => {
    expect(interpreter.input("1 + 1")).toBe(2);
    expect(interpreter.input("2 - 1")).toBe(1);
    expect(interpreter.input("2 * 3")).toBe(6);
    expect(interpreter.input("8 / 4")).toBe(2);
    expect(interpreter.input("7 % 4")).toBe(3);

    //Variables
    expect(interpreter.input("x = 1")).toBe(1);
    expect(interpreter.input("x")).toBe(1);
    expect(interpreter.input("x + 3")).toBe(4);
    expect(() => interpreter.input("y")).toThrow();

    //Functions
    expect(() => interpreter.input("fn avg x y => (x + y) / 2")).not.toThrow();
    expect(interpreter.input("avg 4 2")).toBe(3);
    expect(() => interpreter.input("avg 7")).toThrow();
    expect(() => interpreter.input("avg 7 2 4")).toThrow();

    //Conflicts
    expect(() => interpreter.input("fn x => 0")).toThrow();
    expect(() => interpreter.input("avg = 5")).toThrow();
    expect(() => interpreter.input("fn avg => 0")).not.toThrow();
  });
});
