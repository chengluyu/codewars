function parseRegExp(s) {
  function get() {
    const c = s.charAt(0);
    s = s.substr(1);
    return c;
  }

  function sequence() {
    const ts = [];
    while (s) {
      let c = get();
      let t = null;
      // factor
      if (c === "(") {
        t = choice();
        if ((c = get()) !== ")") {
          console.log(s);
          throw new Error(`expect right bracket instead of "${c}"`);
        }
      } else if (c === ".") {
        t = new Any();
      } else if (c === "*") {
        throw new Error('invalid RegExp factor "*"');
      } else if (c === "|" || c === ")") {
        s = c + s;
        break;
      } else {
        t = new Normal(c);
      }
      // quantifier
      if (s.charAt(0) === "*") {
        s = s.substr(1);
        t = new ZeroOrMore(t);
      }
      ts.push(t);
    }
    if (ts.length === 0) {
      throw new Error("empty regular expression");
    }
    return ts.length > 1 ? new Str(ts) : ts[0];
  }

  function choice() {
    let n = 1;
    let t = sequence();
    while (s.charAt(0) === "|") {
      if (++n > 2) {
        throw new Error('"|" is not associative');
      }
      s = s.substr(1);
      t = new Or(t, sequence());
    }
    return t;
  }

  try {
    const result = choice();
    if (s.length > 0) {
      throw new Error("parse not finished");
    }
    return result;
  } catch (e) {
    console.log(e);
    return null;
  }
}
