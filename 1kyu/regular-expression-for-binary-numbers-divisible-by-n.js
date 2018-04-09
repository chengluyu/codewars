class State {
  constructor(name, accept = false) {
    this.name = name;
    this.accept = accept;
    this.constantTerm = accept ? '' : null;
    this.transition = new Map();
  }

  reduce() {
    let prefix = this.transition.get(this.name);
    if (prefix === undefined) return;
    prefix = `(?:${prefix})*`;
    this.transition.delete(this.name);
    for (const [ target, via ] of this.transition) {
      this.transition.set(target, prefix + via);
    }
    if (this.constantTerm !== null) {
      this.constantTerm = prefix + this.constantTerm;
    }
  }

  substitute(that) {
    // if this is a out state of that state
    let prefix = that.transition.get(this.name);
    if (prefix === undefined) return;
    that.transition.delete(this.name);
    for (const [ target, via ] of this.transition) {
      const expr = that.transition.get(target);
      if (expr === undefined) {
        that.transition.set(target, prefix + via);
      } else {
        that.transition.set(target, `(?:${expr}|(?:${prefix + via}))`);
      }
    }
  }

  toString() {
    return this.name + ' = ' + [...this.transition.entries()].map(
      ([ target, via ]) => `(${via}) ${target}`
    ).join(' + ');
  }
}

function regexDivisibleBy(n) {
  if (n === 1) {
    return '^[01]+$';
  }
  const states = [];
  for (let i = 0; i < n; i++) {
    states.push(new State(`Q_${i}`, i === 0));
  }
  for (let i = 0; i < n; i++) {
    states[i].transition.set(states[i * 2 % n].name, '0');
    states[i].transition.set(states[(i * 2 + 1) % n].name, '1');
  }
  while (states.length > 1) {
    // console.log(`======== Stage ${states.length - 1} ========`);
    // states.forEach(s => console.log(s.toString()));
    states.forEach(s => s.reduce());
    // console.log('After reduction');
    // states.forEach(s => console.log(s.toString()));
    const last = states[states.length - 1];
    states.forEach(s => last.substitute(s));
    // console.log('After substitution');
    // states.forEach(s => console.log(s.toString()));
    states.pop();
  }
  states[0].reduce();
  const pattern = `^(?:${states[0].constantTerm})*$`;
  // console.log(pattern);
  return pattern;
}

function testDivisiblity(n) {
  const regex = new RegExp(regexDivisibleBy(n));
  let failed = false;
  for (let i = 0; i < 100; i++) {
    const expect = i % n === 0;
    const actual = regex.test(i.toString(2));
    if (expect !== actual) {
      failed = true;
      console.log(`Test failed when i = ${i}`);
    }
  }
  if (!failed) {
    console.log('All test passed!');
  }
}
