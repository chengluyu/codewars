class Expression {
  constructor() {}

  differentiate() {
    throw new Error("try to call `differentiate` on base class");
  }

  simplify() {
    throw new Error("try to call `simplify` on base class");
  }

  toString() {
    throw new Error("try to call `toString` on base class");
  }

  static parse(expr) {
    const NUMBER_PATTERN = /^([-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?)/;
    const FUNCTION_PATTERN = /^\w+/;
    const OPERATOR_PATTERN = /^[\+\-\*\/\^]/;
    const WHITESPACE_PATTERN = /^\s+/;

    const FUNCION_MAP = {
      cos: Cosine,
      sin: Sine,
      tan: Tangent,
      ln: Logarithm,
      exp: Exponent
    };

    const OPERATOR_MAP = {
      "+": Add,
      "-": Subtract,
      "*": Multiply,
      "/": Divide,
      "^": Pow
    };

    function whitespace() {
      const result = expr.match(WHITESPACE_PATTERN);
      if (result) {
        expr = expr.substr(result[0].length);
      }
    }

    function expression() {
      let result = expr.match(NUMBER_PATTERN);
      if (result) {
        expr = expr.substr(result[0].length);
        return new Constant(parseFloat(result[0]));
      }
      if (expr.charAt(0) === "x") {
        expr = expr.substr(1);
        return new Variable();
      }
      if (expr.charAt(0) === "(") {
        let node;
        expr = expr.substr(1);
        whitespace();
        if ((result = expr.match(FUNCTION_PATTERN))) {
          expr = expr.substr(result[0].length);
          whitespace();
          const argument = expression();
          if (FUNCION_MAP.hasOwnProperty(result[0])) {
            node = new FUNCION_MAP[result[0]](argument);
          } else {
            throw new Error(`illegal function name "${result[0]}"`);
          }
        } else if ((result = expr.match(OPERATOR_PATTERN))) {
          expr = expr.substr(1);
          whitespace();
          const lhs = expression();
          whitespace();
          const rhs = expression();
          node = new OPERATOR_MAP[result[0]](lhs, rhs);
        } else {
          throw new Error(
            `expect an operator or a function name instead of "${expr}"`
          );
        }
        whitespace();
        if (expr.charAt(0) === ")") {
          expr = expr.substr(1);
        } else {
          throw new Error(`expect a ")" instead of "${expr}"`);
        }
        return node;
      }
      throw new Error(`expect a prefix expression instead of "${expr}"`);
    }

    const result = expression();
    whitespace();
    if (expr.length > 0) {
      throw new Error(`unrecognized character sequence ${expr}`);
    }
    return result;
  }
}

class Constant extends Expression {
  constructor(value) {
    super();
    this.value = value;
    Object.freeze(this);
  }

  differentiate() {
    return Constant.ZERO;
  }

  simplify() {
    return this;
  }

  toString() {
    return this.value.toString();
  }
}

Constant.ZERO = new Constant(0);
Constant.ONE = new Constant(1);

class Variable extends Expression {
  constructor() {
    super();
  }

  differentiate() {
    return Constant.ONE;
  }

  simplify() {
    return this;
  }

  toString() {
    return "x";
  }
}

class Add extends Expression {
  constructor(lhs, rhs) {
    super();
    this.lhs = lhs;
    this.rhs = rhs;
  }

  differentiate() {
    return new Add(this.lhs.differentiate(), this.rhs.differentiate());
  }

  simplify() {
    const lhs = this.lhs.simplify();
    const rhs = this.rhs.simplify();
    // constant folding
    if (lhs instanceof Constant) {
      if (lhs.value === 0) {
        return rhs;
      }
      if (rhs instanceof Constant) {
        return new Constant(lhs.value + rhs.value);
      }
    }
    if (rhs instanceof Constant) {
      if (rhs.value === 0) {
        return lhs;
      }
    }
    // no change
    if (lhs === this.lhs && rhs === this.rhs) {
      return this;
    }
    return new Add(lhs, rhs);
  }

  toString() {
    return `(+ ${this.lhs.toString()} ${this.rhs.toString()})`;
  }
}

class Subtract extends Expression {
  constructor(lhs, rhs) {
    super();
    this.lhs = lhs;
    this.rhs = rhs;
  }

  differentiate() {
    return new Subtract(this.lhs.differentiate(), this.rhs.differentiate());
  }

  simplify() {
    const lhs = this.lhs.simplify();
    const rhs = this.rhs.simplify();
    // x - x === 0
    if (lhs instanceof Variable && rhs instanceof Variable) {
      return new Constant(0);
    }
    // constant folding
    if (lhs instanceof Constant && rhs instanceof Constant) {
      if (rhs.value === 0) {
        return lhs;
      }
      return new Constant(lhs.value - rhs.value);
    }
    // no change
    if (lhs === this.lhs && rhs === this.rhs) {
      return this;
    }
    return new Subtract(lhs, rhs);
  }

  toString() {
    return `(- ${this.lhs.toString()} ${this.rhs.toString()})`;
  }
}

class Multiply extends Expression {
  constructor(lhs, rhs) {
    super();
    if (rhs instanceof Constant) {
      this.lhs = rhs;
      this.rhs = lhs;
    } else {
      this.lhs = lhs;
      this.rhs = rhs;
    }
  }

  differentiate() {
    return new Add(
      new Multiply(this.lhs.differentiate(), this.rhs),
      new Multiply(this.lhs, this.rhs.differentiate())
    );
  }

  simplify() {
    let lhs = this.lhs.simplify();
    let rhs = this.rhs.simplify();
    // put constant in the front
    if (rhs instanceof Constant) {
      const temp = lhs;
      lhs = rhs;
      rhs = temp;
    }
    // constant folding
    if (lhs instanceof Constant) {
      if (lhs.value === 0) {
        return Constant.ZERO;
      }
      if (lhs.value === 1) {
        return rhs;
      }
      if (rhs instanceof Constant) {
        return new Constant(lhs.value * rhs.value);
      }
    }
    // no change
    if (lhs === this.lhs && rhs === this.rhs) {
      return this;
    }
    return new Multiply(lhs, rhs);
  }

  toString() {
    return `(* ${this.lhs.toString()} ${this.rhs.toString()})`;
  }
}

class Divide extends Expression {
  constructor(lhs, rhs) {
    super();
    this.lhs = lhs;
    this.rhs = rhs;
  }

  differentiate() {
    // f(x) / a
    if (this.rhs instanceof Constant) {
      return new Divide(this.lhs.differentiate(), this.rhs);
    }
    // f(x) / g(x)
    return new Divide(
      new Subtract(
        new Multiply(this.lhs.differentiate(), this.rhs),
        new Multiply(this.lhs, this.rhs.differentiate())
      ),
      new Pow(this.rhs, new Constant(2))
    );
  }

  simplify() {
    const lhs = this.lhs.simplify();
    const rhs = this.rhs.simplify();
    if (lhs instanceof Constant) {
      if (lhs.value === 0) {
        return Constant.ZERO;
      }
      if (rhs instanceof Constant) {
        return new Constant(lhs.value / rhs.value);
      }
    }
    if (rhs instanceof Constant) {
      if (rhs.value === 0) {
        throw new Error("try to divide by zero");
      }
      if (rhs.value === 1) {
        return lhs;
      }
    }
    if (lhs === this.lhs && rhs === this.rhs) {
      return this;
    }
    return new Divide(lhs, rhs);
  }

  toString() {
    return `(/ ${this.lhs.toString()} ${this.rhs.toString()})`;
  }
}

class Pow extends Expression {
  constructor(base, exponent) {
    super();
    this.base = base;
    this.exponent = exponent;
  }

  differentiate() {
    // TODO use `hasVariable` instead
    // f(x) ^ a
    if (this.exponent instanceof Constant) {
      const a = this.exponent.value;
      if (a === 0) {
        return Constant.ZERO;
      }
      return new Multiply(
        new Constant(a),
        new Multiply(
          this.base.differentiate(),
          new Pow(this.base, new Constant(a - 1))
        )
      );
    }
    // a ^ f(x)
    if (this.base instanceof Constant) {
      const a = this.base.value;
      return new Multiply(
        new Logarithm(a),
        new Multiply(this.exponent.differentiate(), this)
      );
    }
    // f(x) ^ g(x)
    return new Exponent(
      new Multiply(this.exponent, new Logarithm(this.base))
    ).differentiate();
  }

  simplify() {
    const base = this.base.simplify();
    const exponent = this.exponent.simplify();
    if (base instanceof Constant) {
      if (exponent instanceof Constant) {
        return new Constant(Math.pow(base.value, exponent.value));
      }
      if (base.value === 1) {
        return Constant.ONE;
      }
    }
    if (exponent instanceof Constant) {
      // pow(x, 0) === 1
      if (exponent.value === 0) {
        return Constant.ONE;
      }
      // pow(x, 1) === x
      if (exponent.value === 1) {
        return base;
      }
    }
    // simplification doesn't change base and exponent
    if (base === this.base && exponent === this.exponent) {
      return this;
    }
    return new Pow(base, exponent);
  }

  toString() {
    return `(^ ${this.base.toString()} ${this.exponent.toString()})`;
  }
}

class Cosine extends Expression {
  constructor(argument) {
    super();
    this.argument = argument;
  }

  differentiate() {
    return new Multiply(
      this.argument.differentiate(),
      new Multiply(new Constant(-1), new Sine(this.argument))
    );
  }

  simplify() {
    const argument = this.argument.simplify();
    if (argument instanceof Constant) {
      return new Constant(Math.cos(argument.value));
    }
    if (argument === this.argument) {
      return this;
    }
    return new Cosine(argument);
  }

  toString() {
    return `(cos ${this.argument.toString()})`;
  }
}

class Sine extends Expression {
  constructor(argument) {
    super();
    this.argument = argument;
  }

  differentiate() {
    return new Multiply(
      this.argument.differentiate(),
      new Cosine(this.argument)
    );
  }

  simplify() {
    const argument = this.argument.simplify();
    if (argument instanceof Constant) {
      return new Constant(Math.sin(argument.value));
    }
    if (argument === this.argument) {
      return this;
    }
    return new Sine(argument);
  }

  toString() {
    return `(sin ${this.argument.toString()})`;
  }
}

class Tangent extends Expression {
  constructor(argument) {
    super();
    this.argument = argument;
  }

  differentiate() {
    return new Multiply(
      this.argument.differentiate(),
      new Add(new Constant(1), new Pow(this, new Constant(2)))
    );
  }

  simplify() {
    const argument = this.argument.simplify();
    if (argument instanceof Constant) {
      return new Constant(Math.tan(argument.value));
    }
    if (argument === this.argument) {
      return this;
    }
    return new Tangent(argument);
  }

  toString() {
    return `(tan ${this.argument.toString()})`;
  }
}

class Exponent extends Expression {
  constructor(argument) {
    super();
    this.argument = argument;
  }

  differentiate() {
    return new Multiply(this.argument.differentiate(), this);
  }

  simplify() {
    const argument = this.argument.simplify();
    if (argument instanceof Constant) {
      return new Constant(Math.exp(argument.value));
    }
    if (argument === this.argument) {
      return this;
    }
    return new Exponent(argument);
  }

  toString() {
    return `(exp ${this.argument.toString()})`;
  }
}

class Logarithm extends Expression {
  constructor(argument) {
    super();
    this.argument = argument;
  }

  differentiate() {
    return new Multiply(
      this.argument.differentiate(),
      new Divide(new Constant(1), this.argument)
    );
  }

  simplify() {
    const argument = this.argument.simplify();
    if (argument instanceof Constant) {
      return new Constant(Math.ln(argument.value));
    }
    if (argument === this.argument) {
      return this;
    }
    return new Logarithm(argument);
  }

  toString() {
    return `(exp ${this.argument.toString()})`;
  }
}

function checkConstructionAndStringify(expr) {
  const root = Expression.parse(expr);
  const serialized = root.toString();
  if (expr === serialized) {
    console.log(`Everything works well with "${expr}".`);
  } else {
    console.log(`Oops! "${expr}" !== "${serialized}".`);
  }
}

function diff(expr) {
  const y = Expression.parse(expr);
  console.log("The original tree:", y.toString());
  const dy = y.differentiate();
  console.log("Differentiated expression:", dy.toString());
  const reduced_dy = dy.simplify();
  console.log("Reduced differentiated expression:", reduced_dy.toString());
  return reduced_dy.toString();
}
