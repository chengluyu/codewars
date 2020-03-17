const tokenizationPattern = /\s*(=>|[-+*/%=()]|[A-Za-z_][A-Za-z0-9_]*|[0-9]*\.?[0-9]+)\s*/g;
const whitespacePattern = /^\s*$/;

const identifierPattern = /^[A-Za-z_][A-Za-z0-9_]*$/;
const numberPattern = /^[0-9]*\.?[0-9]+$/;

class Parser {
  constructor(context) {
    this.context = context;
    this.tokens = [];
    this.at = 0;
    this.fn = false;
    this.params = new Set();
  }

  get peek() {
    return this.at < this.tokens.length ? this.tokens[this.at] : null;
  }

  advance() {
    this.at += 1;
  }

  get() {
    if (this.at < this.tokens.length) {
      return this.tokens[this.at++];
    }
    throw new Error("unexpected EOF");
  }

  expect(word) {
    if (word === null) {
      if (this.peek === null) {
        return null;
      }
      throw new Error("expect EOF");
    }
    if (typeof word === "string") {
      if (this.peek === word) {
        return this.get();
      }
      throw new Error(`expected ${word}`);
    } else {
      const result = this.peek.match(word);
      if (result === null) {
        throw new Error(`expect ${word}`);
      }
      this.advance();
      return result;
    }
  }

  match(word) {
    if (typeof word === "string") {
      if (this.peek === word) {
        return this.get();
      }
      return null;
    } else {
      const result = this.peek.match(word);
      if (result !== null) {
        this.advance();
      }
      return result;
    }
  }

  test(word) {
    if (typeof word === "string") {
      return this.peek === word;
    } else {
      return word.test(this.peek);
    }
  }

  parse(tokens) {
    this.tokens = tokens;
    this.at = 0;
    this.fn = false;
    this.params = new Set();
    const result =
      this.peek === "fn" ? this.parseFunction() : this.parseAdditive();
    this.expect(null);
    return result;
  }

  parseFunction() {
    this.fn = true;
    this.expect("fn");
    const name = this.expect(identifierPattern)[0];
    const params = [];
    while (this.peek !== "=>") {
      const param = this.expect(identifierPattern)[0];
      if (this.params.has(param)) {
        throw new Error(`duplicated parameter name "${param}"`);
      }
      this.params.add(param);
      params.push(param);
    }
    this.expect("=>");
    const body = this.parseAdditive();
    const fn = { type: "fn", name, params, body };
    this.context.define(name, fn);
    return fn;
  }

  parseAdditive() {
    let left = this.parseMulplicative();
    while (this.peek === "+" || this.peek === "-") {
      const op = this.get();
      left = { type: "binary", left, op, right: this.parseMulplicative() };
    }
    return left;
  }

  parseMulplicative() {
    let left = this.parsePrimary();
    while (this.peek === "*" || this.peek === "/" || this.peek === "%") {
      const op = this.get();
      left = { type: "binary", left, op, right: this.parsePrimary() };
    }
    return left;
  }

  parsePrimary() {
    // number
    let result = this.match(numberPattern);
    if (result) {
      return { type: "number", value: parseFloat(result[0]) };
    }

    // identifier, assignment or function-call
    if (this.test(identifierPattern)) {
      if (this.context.functions.has(this.peek)) {
        return this.parseFunctionCall();
      }
      const name = this.get();
      if (this.match("=")) {
        return { type: "assignment", name, value: this.parseAdditive() };
      }
      // If in a function context
      if (this.fn && !this.params.has(name)) {
        throw new Error(`undeclared parameter "${name}"`);
      }
      return { type: "lookup", name };
    }

    // parthensized expression
    if (this.match("(")) {
      const expr = this.parseAdditive();
      this.expect(")");
      return expr;
    }

    throw new Error(
      "expect number, identifier, assignment, parthensized expression or function call"
    );
  }

  parseFunctionCall() {
    const callee = this.expect(identifierPattern)[0];
    const fn = this.context.functions.get(callee);
    if (fn === undefined) {
      throw new Error(`undeclared function ${callee}`);
    }
    const args = [];
    for (let i = 0; i < fn.params.length; i++) {
      args.push(this.parseAdditive());
    }
    return { type: "call", callee, args };
  }
}

function evaluate(env, t) {
  switch (t.type) {
    case "lookup":
      return env.lookup(t.name);
    case "number":
      return t.value;
    case "binary":
      switch (t.op) {
        case "+":
          return evaluate(env, t.left) + evaluate(env, t.right);
        case "-":
          return evaluate(env, t.left) - evaluate(env, t.right);
        case "*":
          return evaluate(env, t.left) * evaluate(env, t.right);
        case "/":
          return evaluate(env, t.left) / evaluate(env, t.right);
        case "%":
          return evaluate(env, t.left) % evaluate(env, t.right);
        default:
          throw new Error(`unsupported operator ${t.op}`);
      }
    case "call":
      if (env.context.functions.has(t.callee)) {
        const fn = env.context.functions.get(t.callee);
        const args = t.args.map(x => evaluate(env, x));
        return evaluate(env.enter(fn.params, args), fn.body);
      } else {
        throw new Error(`function ${t.callee} not found`);
      }
    case "assignment":
      if (env.context.functions.has(t.name)) {
        throw new Error(`name ${t.name} had been declared as a function`);
      } else {
        const value = evaluate(env, t.value);
        env.context.define(t.name, value);
        return value;
      }
    default:
      throw new Error(`unsupported node type "${t.type}"`);
  }
}

class Environment {
  constructor(context, stack = []) {
    this.context = context;
    this.stack = stack;
  }

  enter(params = [], args = []) {
    if (params.length !== args.length) {
      throw new Error(
        `expected ${params.length} arguments but ${args.length} were given`
      );
    }
    const frame = new Map(params.map((x, i) => [x, args[i]]));
    return new Environment(this.context, [frame].concat(this.stack));
  }

  lookup(name) {
    for (const frame of this.stack) {
      const value = frame.get(name);
      if (value !== undefined) {
        return value;
      }
    }
    const value = this.context.vars.get(name);
    if (value === undefined) {
      throw new Error(`no such variable ${name}`);
    }
    return value;
  }
}

class Interpreter {
  constructor() {
    this.vars = new Map();
    this.functions = new Map();
  }

  tokenize(source) {
    return source.length === 0
      ? []
      : source
          .split(tokenizationPattern)
          .filter(x => !whitespacePattern.test(x));
  }

  input(source) {
    const tokens = this.tokenize(source);
    // Special case: empty input.
    if (tokens.length === 0) {
      return "";
    }
    const parser = new Parser(this);
    const ast = parser.parse(tokens);
    if (ast.type === "fn") {
      return ""; // Do as required.
    }
    return evaluate(new Environment(this), ast);
  }

  define(name, value) {
    if (typeof value === "number") {
      if (this.functions.has(name)) {
        throw new Error(`cannot redeclare value "${name}" as a function`);
      }
      this.vars.set(name, value);
    } else {
      if (this.vars.has(name)) {
        throw new Error(`cannot redeclare function "${name}" as a value`);
      }
      this.functions.set(name, value);
    }
  }
}

module.exports = { Interpreter, Parser };
