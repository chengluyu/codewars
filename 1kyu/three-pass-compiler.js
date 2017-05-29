function Compiler () {};

Compiler.prototype.compile = function (program) {
  return this.pass3(this.pass2(this.pass1(program)));
};

Compiler.prototype.tokenize = function (program) {
  // Turn a program string into an array of tokens.  Each token
  // is either '[', ']', '(', ')', '+', '-', '*', '/', a variable
  // name or a number (as a string)
  let regex = /\s*([-+*/\(\)\[\]]|[A-Za-z]+|[0-9]+)\s*/g;
  return program.replace(regex, ":$1").substring(1).split(':').map(tok => {
    return isNaN(tok) ? tok : (tok | 0);
  });
};

Compiler.prototype.pass1 = function (program) {
  const inspect = require('util').inspect;

  let stream = this.tokenize(program)[Symbol.iterator](), peek;

  let env = new Map;

  function next() {
    peek = stream.next();
  }

  function match(value) {
    if (peek.value === value) {
      peek = stream.next();
      return true;
    }
    return false;
  }

  function matchRegExp(regex) {
    if (peek.value.match(regex)) {
      peek = stream.next();
      return true;
    }
    return false;
  }

  function expect(value) {
    if (peek.value === value) {
      peek = stream.next();
    } else {
      throw new Error(`expect a ${inspect(value)} instead of ${peek.done ? 'EOF' : inspect(peek.value)}`);
    }
  }

  function parseFunction() {
    expect('[');
    parseArgList().forEach((arg, index) => {
      env.set(arg, index);
    });

    expect(']');
    let expression = parseExpression();

    return expression;
  }

  function parseArgList() {
    let args = [];
    while (!peek.done && peek.value.match(/[A-Za-z]+/)) {
      args.push(peek.value);
      next();
    }
    return args;
  }

  function parseExpression() {
    let left = parseTerm();
    while (peek.value === '+' || peek.value === '-') {
      left = { op: peek.value, a: left };
      next();
      left.b = parseTerm();
    }
    return left;
  }

  function parseTerm() {
    let left = parseFactor();
    while (peek.value === '*' || peek.value === '/') {
      left = { op: peek.value, a: left };
      next();
      left.b = parseFactor();
    }
    return left;
  }

  function parseFactor() {
    if (peek.done) {
      throw new Error('unexpected end of source');
    }

    if (match('(')) {
      let expression = parseExpression();
      expect(')');
      return expression;
    }

    if (typeof peek.value === "number") {
      let ast = { op: 'imm', n: peek.value };
      next();
      return ast;
    }

    if (peek.value.match(/[A-Za-z]+/)) {
      if (env.has(peek.value)) {
        let ast = { op: 'arg', n: env.get(peek.value) };
        next();
        return ast;
      } else {
        throw new Error(`undefined identifier: ${inspect(peek.value)}`);
      }
    }
    
    throw new Error(`unknown token: ${inspect(peek.value)}`);
  }

  next();
  return parseFunction();
};

Compiler.prototype.pass2 = function (ast) {
  // return AST with constant expressions reduced
  function reduce(node) {
    // type check
    if (typeof node !== 'object') {
      throw new Error('unqualified object');
    }

    if (node.op === 'arg' || node.op === 'imm') {
      return node;
    }

    node.a = reduce(node.a);
    node.b = reduce(node.b);

    if (node.a.op === 'imm' && node.b.op === 'imm') {
      let reduced;
      switch (node.op) {
        case '+': reduced = node.a.n + node.b.n; break;
        case '-': reduced = node.a.n - node.b.n; break;
        case '*': reduced = node.a.n * node.b.n; break;
        case '/': reduced = node.a.n / node.b.n; break;
        default: throw new Error(`unknown op: ${inspect(node.op)}`)
      }
      return { op: 'imm', n: reduced };
    }

    return node;
  }

  return reduce(ast);
};

Compiler.prototype.pass3 = function (ast) {
  // return assembly instructions
  let asm = [];

  function dump(node, hint) {
    switch (node.op) {
      case '+':
      case '-':
      case '*':
      case '/':
        asm.push('PU');
        dump(node.a);
        asm.push('SW');
        dump(node.b);
      case 'imm':
        
      case 'arg':

    }
  }
};

exports.Compiler = Compiler;
