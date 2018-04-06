
const NUMBER_PATTERN = /^([-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?)/;
const WHITESPACE = /^\s+/;

const Tokens = {
  ADD: {
    type: 'operator',
    value: '+',
    apply(x, y) {
      return x + y;
    }
  },
  SUB: {
    type: 'operator',
    value: '-',
    apply(x, y) {
      return x - y;
    }
  },
  MUL: {
    type: 'operator',
    value: '*',
    apply(x, y) {
      return x * y;
    }
  },
  DIV: {
    type: 'operator',
    value: '/',
    apply(x, y) {
      return x / y;
    }
  },
  LPAREN: {
    type: 'symbol',
    value: '('
  },
  RPAREN: {
    type: 'symbol',
    value: ')'
  }
};

function tokenize(expr) {
  function number() {
    const result = expr.match(NUMBER_PATTERN);
    if (result) {
      expr = expr.substr(result[0].length);
      return {
        type: 'number',
        value: parseFloat(result[0])
      };
    }
  }

  const tokens = [];
  let last;

  function append(token) {
    tokens.push(last = token);
  }

  loop: while (expr) {
    // ignore leading whitespace
    const result = expr.match(WHITESPACE);
    if (result) {
      expr = expr.substr(result[0].length);
    }

    let maybeNumber;
    switch (expr.charAt(0)) {
      case '+':
        append(Tokens.ADD);
        break;
      case '-':
        if (!last || last.type === 'operator') {
          maybeNumber = number();
          append(maybeNumber ? maybeNumber : Tokens.SUB);
        } else {
          append(Tokens.SUB);
        }
        break;
      case '*':
        append(Tokens.MUL);
        break;
      case '/':
        append(Tokens.DIV);
        break;
      case '(':
        append(Tokens.LPAREN);
        break;
      case ')':
        append(Tokens.RPAREN);
        break;
      default:
        maybeNumber = number();
        if (maybeNumber) {
          append(maybeNumber);
        } else if (expr.length === 0) {
          break loop;
        } else {
          throw new Error(`Unknown character: "${expr.charAt(0)}"`);
        }
    }

    if (last.type !== 'number') {
      expr = expr.substr(1);
    }
  }
  return tokens.reverse();
}

function calc(expr) {
  const tokens = tokenize(expr);
  let peek;

  function advance() {
    peek = tokens.pop();
  }

  function next() {
    const save = peek;
    advance();
    return save;
  }

  function make(operators, superior) {
    return () => {
      let result = superior();
      while (peek && operators.has(peek.value)) {
        result = next().apply(result, superior());
      }
      return result;
    };
  }

  const term = make(new Set(['*', '/']), value);

  const factor = make(new Set(['+', '-']), term);

  function value() {
    if (peek.value === '(') {
      advance();
      const result = factor();
      if (next().value !== ')') {
        throw new Error(`expect right bracket instead of "${peek.value}"`);
      }
      return result;
    } else if (peek.type === 'number') {
      return next().value;
    } else if (peek.value === '-') {
      advance();
      return -value();
    } else {
      throw new Error(`expect a number or expression enclosed with parentheses instead of "${peek.value}"`);
    }
  }

  advance();
  return factor();
}

[
  ['1+1', 2],
  ['1 - 1', 0],
  ['1* 1', 1],
  ['1 /1', 1],
  ['-123', -123],
  ['123', 123],
  ['2 /2+3 * 4.75- -6', 21.25],
  ['12* 123', 1476],
  ['2 / (2 + 3) * 4.33 - -6', 7.732],
].forEach(m => {
  console.log(`"${m[0]}"`, calc(m[0]), m[1]);
});
